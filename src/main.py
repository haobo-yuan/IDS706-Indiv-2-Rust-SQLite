import polars as pl
import pandas as pd
import os
from lib import preprocess_data, generate_plot


def main():
    # Load and preprocess data
    stock_AAPL = preprocess_data()


    ## Data Preprocessing for Rust project
    # change column names to make after-processing easier
    stock_AAPL.columns = ["date", "open", "high", "low", "close", "adj_close", "volume", "name", "year"]
    # convert the polars DataFrame to a pandas DataFrame, and then export it to a CSV file
    stock_AAPL.to_pandas().to_csv(os.path.join("data", "stock_AAPL.csv"), index=False)


    # Descriptive Statistics
    yearly_stats = (
        stock_AAPL.group_by("Year")
        .agg(
            [
                pl.col("Close").mean().alias("mean"),
                pl.col("Close").median().alias("median"),
                pl.col("Close").std().alias("std"),
            ]
        )
        .sort("Year")  # Ensure data is sorted by 'Year'
    )

    # Generate Visualization
    generate_plot(yearly_stats)


if __name__ == "__main__":
    main()
