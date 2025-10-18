
import json
import os
from datetime import datetime

MEMORY_GRAPH_PATH = "data/memory_graph.json"

def save_memory_entry(seed, mutated_seed, address, index, cluster_id=None):
    os.makedirs(os.path.dirname(MEMORY_GRAPH_PATH), exist_ok=True)
    entry = {
        "timestamp": datetime.utcnow().isoformat(),
        "index": index,
        "original_seed": seed,
        "mutated_seed": mutated_seed,
        "address": address,
        "cluster_id": cluster_id or "default"
    }

    data = []
    if os.path.exists(MEMORY_GRAPH_PATH):
        with open(MEMORY_GRAPH_PATH, "r", encoding="utf-8") as f:
            try:
                data = json.load(f)
            except json.JSONDecodeError:
                data = []

    data.append(entry)

    with open(MEMORY_GRAPH_PATH, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)
