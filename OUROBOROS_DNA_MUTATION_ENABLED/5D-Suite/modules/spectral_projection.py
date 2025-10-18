import matplotlib.pyplot as plt
import numpy as np
import hashlib
import math
import random

class SpectralProjector:
    def __init__(self):
        self.feldlinien = []

    def add_seed(self, seed):
        h = hashlib.sha256(seed.encode()).hexdigest()
        values = [int(h[i:i+2], 16)/255.0 for i in range(0, len(h), 2)]
        self.feldlinien.append(values)

    def plot_projection(self, count=5):
        if not self.feldlinien:
            print("Keine Spektralfelder vorhanden.")
            return

        plt.figure(figsize=(12, 6))
        for i, f in enumerate(self.feldlinien[-count:]):
            x = np.linspace(0, len(f), len(f))
            phase = i * (np.pi / 6)
            mod = [math.sin(v * 2 * np.pi + phase) * 0.5 + 0.5 for v in f]
            plt.plot(x, mod, alpha=0.7, label=f"Feld {len(self.feldlinien)-count+i+1}")
        plt.title("Dynamische Spektralprojektion")
        plt.legend()
        plt.show()
