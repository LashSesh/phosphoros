
import json
import os

HEATMAP_PATH = "data/heatmap.json"

def log_address_heat(address, balance):
    os.makedirs(os.path.dirname(HEATMAP_PATH), exist_ok=True)

    data = {}
    if os.path.exists(HEATMAP_PATH):
        with open(HEATMAP_PATH, "r") as f:
            try:
                data = json.load(f)
            except json.JSONDecodeError:
                data = {}

    prefix = address[:6]
    if prefix not in data:
        data[prefix] = {"count": 0, "sum": 0.0}

    data[prefix]["count"] += 1
    data[prefix]["sum"] += balance or 0.0

    with open(HEATMAP_PATH, "w") as f:
        json.dump(data, f, indent=2)
