# script.py

import polars as pl
import os
from lib import preprocess_data, generate_plot


def main():
    # Load and preprocess data
    stock_AAPL = preprocess_data()

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
