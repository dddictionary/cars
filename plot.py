import sys

import matplotlib.pyplot as plt
import pandas as pd

# Get the stats file from command-line argument or default
stats_file = sys.argv[1] if len(sys.argv) > 1 else "stats.csv"

# Load the data
df = pd.read_csv(stats_file)

# Create two subplots: one for entropy, one for live cells
fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8), sharex=True)

# Plot entropy over generations
ax1.plot(df["generation"], df["entropy"], color="orange")
ax1.set_ylabel("Entropy")
ax1.set_title("Entropy over Generations")
ax1.grid(True)

# Plot live cells over generations
ax2.plot(df["generation"], df["live_cells"], color="blue")
ax2.set_xlabel("Generation")
ax2.set_ylabel("Live Cells")
ax2.set_title("Live Cell Count over Generations")
ax2.grid(True)

# Layout and display
plt.tight_layout()
plt.show()

