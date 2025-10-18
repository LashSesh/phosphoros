
import json
import os

class PathRecorder:
    def __init__(self, path="recorded_paths.json"):
        self.path = path
        if not os.path.exists(self.path):
            with open(self.path, "w") as f:
                json.dump([], f)

    def log_step(self, cell_name, vector, seed, address, balance):
        entry = {
            "cell": cell_name,
            "vector": vector.tolist(),
            "seed": seed,
            "address": address,
            "balance": balance
        }
        with open(self.path, "r+") as f:
            data = json.load(f)
            data.append(entry)
            f.seek(0)
            json.dump(data, f, indent=2)

    def load_all(self):
        with open(self.path, "r") as f:
            return json.load(f)

    def get_path_by_cell(self, cell_name):
        return [entry for entry in self.load_all() if entry["cell"] == cell_name]
