
import json
import os
import numpy as np

class CollectiveField:
    def __init__(self, path="collective_field.json"):
        self.path = path
        if not os.path.exists(self.path):
            with open(self.path, "w") as f:
                json.dump([], f)

    def log_point(self, seed, vector, address, balance):
        point = {
            "seed": seed,
            "vector": vector.tolist(),
            "address": address,
            "balance": balance
        }
        with open(self.path, "r+") as f:
            data = json.load(f)
            data.append(point)
            f.seek(0)
            json.dump(data, f, indent=2)

    def load_vectors(self):
        with open(self.path, "r") as f:
            data = json.load(f)
        vectors = [d["vector"] for d in data]
        return np.array(vectors) if vectors else np.zeros((0, 12))

    def get_density_map(self, bins=10):
        vectors = self.load_vectors()
        if vectors.shape[0] == 0:
            return None
        hist, edges = np.histogramdd(vectors, bins=bins)
        return hist, edges
