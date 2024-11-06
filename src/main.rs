use std::error::Error;
use rusqlite::{Connection, Result};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;

// Define `StockRecord` struct
#[derive(Debug, Deserialize)]
pub struct StockRecord {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: i64,
    name: String,
    year: i32,
}

// Initialize database and create table
pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stock_data (
            date TEXT,
            open REAL,
            high REAL,
            low REAL,
            close REAL,
            adj_close REAL,
            volume INTEGER,
            name TEXT,
            year INTEGER
        )",
        [],
    )?;
    Ok(conn)
}

// Load CSV data and insert into database
pub fn load_csv_to_db(conn: &Connection, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(csv_path)?;
    for result in rdr.deserialize() {
        let record: StockRecord = result?;
        conn.execute(
            "INSERT INTO stock_data (date, open, high, low, close, adj_close, volume, name, year)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                record.date,
                record.open,
                record.high,
                record.low,
                record.close,
                record.adj_close,
                record.volume,
                record.name,
                record.year,
            ],
        )?;
    }
    Ok(())
}

// Calculate statistics
pub fn calculate_stats(conn: &Connection) -> Result<Vec<(i32, f64, f64, f64)>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT year, close
         FROM stock_data
         ORDER BY year"
    )?;
    let mut rows = stmt.query([])?;

    let mut data: HashMap<i32, Vec<f64>> = HashMap::new();

    // Group closing prices by year
    while let Some(row) = rows.next()? {
        let year: i32 = row.get(0)?;
        let close: f64 = row.get(1)?;
        data.entry(year).or_insert_with(Vec::new).push(close);
    }

    let mut stats = Vec::new();

    for (year, closes) in data {
        // Calculate mean
        let mean = closes.iter().sum::<f64>() / closes.len() as f64;

        // Calculate median
        let median = {
            let mut sorted = closes.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 {
                (sorted[mid - 1] + sorted[mid]) / 2.0
            } else {
                sorted[mid]
            }
        };

        // Calculate standard deviation
        let std = {
            let mean_diff_sq = closes.iter().map(|v| (v - mean).powi(2)).sum::<f64>();
            (mean_diff_sq / closes.len() as f64).sqrt()
        };

        stats.push((year, mean, median, std));
    }

    // Sort by year
    stats.sort_by_key(|k| k.0);

    Ok(stats)
}

// Save statistics to a new database
pub fn save_stats_to_db(db_path: &str, stats: &Vec<(i32, f64, f64, f64)>) -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open(db_path)?;

    // Create stats_data table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stats_data (
            year INTEGER PRIMARY KEY,
            mean REAL,
            median REAL,
            std REAL
        )",
        [],
    )?;

    // Insert statistics using INSERT OR REPLACE
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO stats_data (year, mean, median, std) VALUES (?1, ?2, ?3, ?4)"
        )?;

        for &(year, mean, median, std) in stats {
            stmt.execute(rusqlite::params![year, mean, median, std])?;
        }
    }
    tx.commit()?;

    Ok(())
}

// Add future statistics
pub fn add_future_stats(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let future_years = [2022, 2023, 2024, 2025];
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO stats_data (year, mean, median, std) VALUES (?1, ?2, ?3, ?4)"
    )?;

    for &year in &future_years {
        stmt.execute(rusqlite::params![year, 0.0, 0.0, 0.0])?;
    }

    println!("\nAdded future statistics for years 2022 to 2025.\n");

    Ok(())
}

// Display statistics
pub fn display_stats(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT year, mean, median, std FROM stats_data ORDER BY year")?;
    let stats_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
        ))
    })?;

    println!("{:<6} {:<10} {:<10} {:<10}", "Year", "Mean", "Median", "Std");
    for stat in stats_iter {
        let (year, mean, median, std) = stat?;
        println!("{:<6} {:<10.2} {:<10.2} {:<10.2}", year, mean, median, std);
    }

    Ok(())
}

// Delete future statistics
pub fn delete_future_stats(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let current_year = 2024;

    println!("Current year is {}, deleting future data.\n", current_year);

    conn.execute("DELETE FROM stats_data WHERE year > ?", rusqlite::params![current_year])?;

    println!("Deleted statistics for years after {}.\n", current_year);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "data/stock_AAPL.db";
    let csv_path = "data/stock_AAPL.csv";

    // Initialize database
    let conn = init_db(db_path)?;

    // Load CSV data into database
    load_csv_to_db(&conn, csv_path)?;

    // Calculate statistics
    let stats = calculate_stats(&conn)?;

    // Output original statistics
    println!("Original statistics:\n");
    println!("{:<6} {:<10} {:<10} {:<10}", "Year", "Mean", "Median", "Std");
    for (year, mean, median, std) in &stats {
        println!("{:<6} {:<10.2} {:<10.2} {:<10.2}", year, mean, median, std);
    }

    // Save statistics to new database
    let stats_db_path = "data/stock_AAPL_stats.db";
    save_stats_to_db(stats_db_path, &stats)?;

    // Open connection to statistics database
    let stats_conn = Connection::open(stats_db_path)?;

    // Update operation: add future data
    add_future_stats(&stats_conn)?;

    // Display updated statistics
    println!("Updated statistics:\n");
    display_stats(&stats_conn)?;

    // Delete operation: delete 2025 data
    delete_future_stats(&stats_conn)?;

    // Display statistics after deletion
    println!("Statistics after deletion:\n");
    display_stats(&stats_conn)?;

    Ok(())
}
