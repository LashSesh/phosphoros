
import json
import os
from collections import defaultdict

MEMORY_GRAPH_PATH = "data/memory_graph.json"

class ResonanceNavigator:
    def __init__(self):
        self.hotspots = self.extract_hotspots()

    def extract_hotspots(self, bucket_size=100, min_hits=3):
        if not os.path.exists(MEMORY_GRAPH_PATH):
            return []

        with open(MEMORY_GRAPH_PATH, "r", encoding="utf-8") as f:
            memory = json.load(f)

        freq = defaultdict(int)
        for entry in memory:
            bucket = int(entry["index"]) // bucket_size
            key = (entry["cluster_id"], bucket)
            freq[key] += 1

        # Filtere Hotspots mit hoher Trefferfrequenz
        hotspots = [bucket * bucket_size for (cluster, bucket), count in freq.items() if count >= min_hits]
        return sorted(set(hotspots))

    def get_next_hotspot_index(self):
        if not self.hotspots:
            return None
        return self.hotspots.pop(0)
