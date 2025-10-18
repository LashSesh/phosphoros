
import json
import os
from collections import defaultdict

class ResonanceHeatmap:
    def __init__(self, heatmap_file="heatmap.json"):
        self.heatmap_file = heatmap_file
        if not os.path.exists(self.heatmap_file):
            with open(self.heatmap_file, 'w') as f:
                json.dump({}, f)

    def _get_prefix(self, address, length=4):
        return address[:length]

    def log_address(self, address, balance):
        prefix = self._get_prefix(address)
        with open(self.heatmap_file, 'r+') as f:
            data = json.load(f)
            if prefix not in data:
                data[prefix] = {"count": 0, "hits": []}
            data[prefix]["count"] += 1
            data[prefix]["hits"].append({"address": address, "balance": balance})
            f.seek(0)
            json.dump(data, f, indent=2)

        print(f"[HEATMAP] +1 in zone {prefix} → {address} (Balance: {balance})")
