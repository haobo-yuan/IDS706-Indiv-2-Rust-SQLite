# test_script.py

import unittest
import subprocess
import os


class TestScript(unittest.TestCase):
    def test_script_execution(self):
        # Ensure 'pictures' directory exists
        os.makedirs("pictures", exist_ok=True)

        # Remove 'plot.png' if it exists
        plot_path = "pictures/plot.png"
        if os.path.exists(plot_path):
            os.remove(plot_path)

        # Run the script and capture output
        result = subprocess.run(["python", "script.py"], capture_output=True, text=True)

        # Check if script ran successfully
        self.assertEqual(
            result.returncode, 0, "script.py did not execute successfully."
        )

        # Check if 'plot.png' was created
        self.assertTrue(
            os.path.exists(plot_path), "'plot.png' was not created by script.py."
        )


if __name__ == "__main__":
    unittest.main()
