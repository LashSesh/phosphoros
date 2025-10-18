
import json
import os
from collections import defaultdict

MEMORY_DB_PATH = "memory_field.json"

class MemoryField:
    def __init__(self, path=MEMORY_DB_PATH):
        self.path = path
        self.seeds = defaultdict(lambda: {"score": 0, "zelltyp": "", "count": 0})
        self._load()

    def _load(self):
        if os.path.exists(self.path):
            with open(self.path, "r") as f:
                raw = json.load(f)
                self.seeds.update(raw)

    def _save(self):
        with open(self.path, "w") as f:
            json.dump(self.seeds, f, indent=2)

    def add(self, seed, score, zelltyp):
        key = seed[:6]  # simplifiziertes Clustering
        if key in self.seeds:
            self.seeds[key]["score"] = (self.seeds[key]["score"] * self.seeds[key]["count"] + score) / (self.seeds[key]["count"] + 1)
            self.seeds[key]["count"] += 1
        else:
            self.seeds[key] = {"score": score, "zelltyp": zelltyp, "count": 1}
        self._save()

    def get_best(self, top_n=5):
        return sorted(self.seeds.items(), key=lambda x: -x[1]["score"])[:top_n]

    def get_failed(self, threshold=0.2):
        return {k: v for k, v in self.seeds.items() if v["score"] < threshold}

    def summary(self):
        return {
            "total_clusters": len(self.seeds),
            "top_clusters": self.get_best(3),
            "low_clusters": list(self.get_failed().keys())[:5]
        }
