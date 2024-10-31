import unittest
import os
import polars as pl
import matplotlib.pyplot as plt
from lib import preprocess_data, generate_plot


class TestLibFunctions(unittest.TestCase):
    def test_preprocess_data(self):
        # Test the preprocess_data function
        df = preprocess_data()
        # Check if the returned object is a Polars DataFrame
        self.assertIsInstance(df, pl.DataFrame)
        # Check if DataFrame is not empty
        self.assertGreater(df.height, 0, "The DataFrame is empty.")
        # Check if 'Year' column exists
        self.assertIn("Year", df.columns, "'Year' column is missing.")
        # Check if all 'Name' entries are 'AAPL'
        self.assertTrue(
            (df["Name"] == "AAPL").all(), "Not all 'Name' entries are 'AAPL'."
        )

    def test_generate_plot(self):
        # Create a sample DataFrame
        data = {
            "Year": [2019, 2020, 2021],
            "mean": [100, 110, 120],
            "median": [90, 105, 115],
            "std": [5, 10, 8],
        }
        yearly_stats = pl.DataFrame(data)

        # Ensure 'pictures' directory exists
        os.makedirs("pictures", exist_ok=True)

        # Remove 'plot.png' if it exists
        plot_path = "pictures/plot.png"
        if os.path.exists(plot_path):
            os.remove(plot_path)

        # Generate plot
        generate_plot(yearly_stats)

        # Check if 'plot.png' was created
        self.assertTrue(os.path.exists(plot_path), "'plot.png' was not created.")


if __name__ == "__main__":
    unittest.main()
