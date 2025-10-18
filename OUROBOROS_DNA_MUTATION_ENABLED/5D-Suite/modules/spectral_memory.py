import matplotlib.pyplot as plt
import numpy as np
import hashlib
import time

class SpectralMemory:
    def __init__(self):
        self.snapshots = []

    def encode_seed(self, seed: str):
        h = hashlib.sha256(seed.encode()).hexdigest()
        return [int(h[i:i+2], 16)/255.0 for i in range(0, len(h), 2)]

    def store_snapshot(self, seed, cell_type):
        timestamp = time.time()
        vec = self.encode_seed(seed)
        self.snapshots.append((timestamp, cell_type, vec))

    def render_latest(self, count=5):
        entries = self.snapshots[-count:]
        if not entries:
            print("Kein Spektral-Gedächtnis gespeichert.")
            return

        fig, ax = plt.subplots()
        for ts, cell, vec in entries:
            x = np.linspace(0, len(vec), len(vec))
            ax.plot(x, vec, label=f"{cell} ({time.strftime('%H:%M:%S', time.localtime(ts))})", alpha=0.7)
        ax.set_title("Spektral-Gedächtnis (Letzte Zellwellen)")
        ax.legend()
        plt.show()
