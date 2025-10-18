
import json
import os

FIELD_LOG_PATH = "data/collective_field.json"

class CollectiveField:
    def __init__(self):
        os.makedirs(os.path.dirname(FIELD_LOG_PATH), exist_ok=True)

    def log_point(self, seed, vector, address, balance):
        point = {
            "seed": seed,
            "vector": vector,
            "address": address,
            "balance": balance
        }

        data = []
        if os.path.exists(FIELD_LOG_PATH):
            with open(FIELD_LOG_PATH, "r") as f:
                try:
                    data = json.load(f)
                except json.JSONDecodeError:
                    data = []

        data.append(point)

        with open(FIELD_LOG_PATH, "w") as f:
            json.dump(data, f, indent=2)

    def load_vectors(self):
        if not os.path.exists(FIELD_LOG_PATH):
            return []
        with open(FIELD_LOG_PATH, "r") as f:
            try:
                data = json.load(f)
                return [d["vector"] for d in data if "vector" in d]
            except json.JSONDecodeError:
                return []
