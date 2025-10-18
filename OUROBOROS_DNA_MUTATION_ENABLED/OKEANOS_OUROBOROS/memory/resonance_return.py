
import json
import os
from datetime import datetime

HIT_LOG_PATH = "data/resonance_hits.json"

class ResonanceReturn:
    def __init__(self):
        os.makedirs(os.path.dirname(HIT_LOG_PATH), exist_ok=True)

    def log_hit(self, seed, address, balance):
        hit = {
            "timestamp": datetime.utcnow().isoformat(),
            "seed": seed,
            "address": address,
            "balance": balance
        }

        data = []
        if os.path.exists(HIT_LOG_PATH):
            with open(HIT_LOG_PATH, "r") as f:
                try:
                    data = json.load(f)
                except json.JSONDecodeError:
                    data = []

        data.append(hit)

        with open(HIT_LOG_PATH, "w") as f:
            json.dump(data, f, indent=2)
