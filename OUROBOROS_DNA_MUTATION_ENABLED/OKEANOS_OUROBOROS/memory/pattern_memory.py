
import json
import os
import random

MEMORY_PATH = "data/pattern_memory.json"

class PatternMemory:
    def __init__(self):
        os.makedirs(os.path.dirname(MEMORY_PATH), exist_ok=True)

    def save_path(self, cell_id, vector):
        entry = {
            "cell": cell_id,
            "vector": vector
        }

        data = []
        if os.path.exists(MEMORY_PATH):
            with open(MEMORY_PATH, "r") as f:
                try:
                    data = json.load(f)
                except json.JSONDecodeError:
                    data = []

        data.append(entry)

        with open(MEMORY_PATH, "w") as f:
            json.dump(data, f, indent=2)

    def repeat_path(self, cell_id, mutation=0.0):
        vectors = []
        if not os.path.exists(MEMORY_PATH):
            return None

        with open(MEMORY_PATH, "r") as f:
            try:
                data = json.load(f)
                vectors = [d["vector"] for d in data if d["cell"] == cell_id]
            except json.JSONDecodeError:
                return None

        return [self._mutate_vector(v, mutation) for v in vectors]

    def combine_paths(self, cell_a, cell_b, mutation=0.1):
        a = self.repeat_path(cell_a, mutation)
        b = self.repeat_path(cell_b, mutation)
        if not a or not b:
            return None
        combined = [(x + y) / 2 for x, y in zip(a[0], b[0])]
        return [self._mutate_vector(combined, mutation)]

    def _mutate_vector(self, vector, mutation):
        return [v + random.uniform(-mutation, mutation) for v in vector]
