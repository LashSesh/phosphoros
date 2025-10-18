
import json
import os
from collections import defaultdict

HEATMAP_DB = "resonance_heatmap.json"

class ResonanceArchive:
    def __init__(self, db_path=HEATMAP_DB):
        self.db_path = db_path
        self.data = defaultdict(lambda: {"hits": 0, "misses": 0})
        self._load()

    def _load(self):
        if os.path.exists(self.db_path):
            with open(self.db_path, "r") as f:
                self.data.update(json.load(f))

    def _save(self):
        with open(self.db_path, "w") as f:
            json.dump(self.data, f, indent=4)

    def log_result(self, seed: str, resonance_score: float):
        key = seed[:4]  # simplify to field-prefix
        if resonance_score > 0.7:
            self.data[key]["hits"] += 1
        else:
            self.data[key]["misses"] += 1
        self._save()

    def get_hotspots(self):
        return {k: v for k, v in self.data.items() if v["hits"] > v["misses"]}

    def get_coldzones(self):
        return {k: v for k, v in self.data.items() if v["misses"] > v["hits"]}

    def full_data(self):
        return dict(self.data)
