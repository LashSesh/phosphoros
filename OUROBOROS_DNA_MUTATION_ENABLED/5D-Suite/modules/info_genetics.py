import matplotlib.pyplot as plt
import hashlib
import numpy as np
import math

class InfoGenetics:
    def __init__(self):
        self.genome = []

    def analyze_seed(self, seed):
        h = hashlib.sha256(seed.encode()).hexdigest()
        # Normalisieren in 5 Stränge à 10 Werte = "Chromosomen"
        chromosomen = [int(h[i:i+2], 16)/255.0 for i in range(0, 100, 2)]
        split = [chromosomen[i:i+10] for i in range(0, 50, 10)]
        self.genome.append(split)

    def plot_dna(self, index=-1):
        if not self.genome:
            print("Kein genetischer Code analysiert.")
            return

        genome = self.genome[index]
        fig = plt.figure(figsize=(12, 6))
        ax = fig.add_subplot(111, projection='3d')

        for i, strand in enumerate(genome):
            theta = np.linspace(0, 4 * np.pi, len(strand))
            z = np.linspace(0, 1, len(strand))
            r = np.array(strand) * 0.2 + 0.1
            x = r * np.cos(theta + i)
            y = r * np.sin(theta + i)
            ax.plot(x, y, z + i * 0.25, label=f"Strang {i+1}")

        ax.set_title("Informationsgenetik – 5D Seed-DNA")
        ax.legend()
        plt.tight_layout()
        plt.show()
