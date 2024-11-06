use std::error::Error;
use rusqlite::{Connection, Result};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;

// Define `StockRecord` struct to represent each stock record; derive `Deserialize` for CSV parsing
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

// Initialize database connection and create the table if it doesn't exist
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

// Load data from CSV file and insert records into the database
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

// Calculate mean, median, and standard deviation of `close` prices grouped by year
pub fn calculate_stats(conn: &Connection) -> Result<Vec<(i32, f64, f64, f64)>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT year, close
         FROM stock_data
         ORDER BY year"
    )?;
    let mut rows = stmt.query([])?;

    let mut data: HashMap<i32, Vec<f64>> = HashMap::new();

    // Group `close` prices by year
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

    // Sort statistics by year for output consistency
    stats.sort_by_key(|k| k.0);

    Ok(stats)
}

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "data/stock_AAPL.db";
    let csv_path = "data/stock_AAPL.csv";

    // Initialize the database
    let conn = init_db(db_path)?;

    // Load CSV data into the database
    load_csv_to_db(&conn, csv_path)?;

    // Calculate annual statistics
    let stats = calculate_stats(&conn)?;

    // Output statistics table
    println!("{:<6} {:<10} {:<10} {:<10}", "Year", "Mean", "Median", "Std");
    for (year, mean, median, std) in stats {
        println!("{:<6} {:<10.2} {:<10.2} {:<10.2}", year, mean, median, std);
    }

    Ok(())
}
