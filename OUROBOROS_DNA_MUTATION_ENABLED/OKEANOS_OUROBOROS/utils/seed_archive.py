
import os
import json
import hashlib

SEED_DB_PATH = "seed_archive.json"

def classify_entropy(seed: str) -> str:
    length = len(seed)
    if length < 16:
        return "low"
    elif length < 32:
        return "medium"
    else:
        return "high"

def classify_prefix(seed: str) -> str:
    if seed.startswith("1"):
        return "legacy"
    elif seed.startswith("3"):
        return "p2sh"
    elif seed.startswith("bc1"):
        return "segwit"
    else:
        return "unknown"

def store_seed(seed: str, source: str = "unknown", result: str = "unchecked"):
    seed_hash = hashlib.sha256(seed.encode()).hexdigest()
    record = {
        "seed": seed,
        "source": source,
        "result": result,
        "entropy_class": classify_entropy(seed),
        "prefix_class": classify_prefix(seed),
        "hash": seed_hash
    }

    if os.path.exists(SEED_DB_PATH):
        with open(SEED_DB_PATH, "r") as f:
            db = json.load(f)
    else:
        db = {}

    db[seed_hash] = record

    with open(SEED_DB_PATH, "w") as f:
        json.dump(db, f, indent=4)

def load_seed_database():
    if os.path.exists(SEED_DB_PATH):
        with open(SEED_DB_PATH, "r") as f:
            return json.load(f)
    return {}
