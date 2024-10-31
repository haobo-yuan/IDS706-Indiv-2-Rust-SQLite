import pandas as pd
import polars as pl
import matplotlib.pyplot as plt

# Data Preprocessing
def preprocess_data():
    # Read the data using Polars
    stock = pl.read_csv("data/NASDAQ_100_Data_From_2010.csv", separator="\t")

    # Filter for AAPL stock data
    stock_AAPL = stock.filter(pl.col("Name") == "AAPL")

    # Convert 'Date' column to datetime
    stock_AAPL = stock_AAPL.with_columns(
        pl.col("Date").str.strptime(pl.Date, "%Y-%m-%d")
    )

    # Add 'Year' column
    stock_AAPL = stock_AAPL.with_columns(pl.col("Date").dt.year().alias("Year"))

    return stock_AAPL


# Plotting the statistics
def generate_plot(yearly_stats):
    plt.figure(figsize=(15, 6))
    years = yearly_stats["Year"].to_numpy()
    means = yearly_stats["mean"].to_numpy()
    medians = yearly_stats["median"].to_numpy()
    stds = yearly_stats["std"].to_numpy()

    plt.plot(years, means, label="Mean", marker="o")
    plt.plot(years, medians, label="Median", marker="x")
    plt.plot(years, stds, label="Standard Deviation", marker="s")
    plt.grid(True)
    plt.title("AAPL Close Price Statistics (2010-2021)")
    plt.xlabel("Year")
    plt.ylabel("Price")
    plt.legend()
    plt.savefig("pictures/plot.png")
