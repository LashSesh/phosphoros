
import json
import os
from phase_mask import PhaseMask

class ResonanceReturn:
    def __init__(self, log_file="resonance_hits.json"):
        self.log_file = log_file
        self.masker = PhaseMask()
        if not os.path.exists(self.log_file):
            with open(self.log_file, 'w') as f:
                json.dump([], f)

    def log_hit(self, seed, address, balance):
        hit = {
            "seed": self.masker.apply(seed),
            "address": address,
            "balance": balance
        }
        with open(self.log_file, 'r+') as f:
            data = json.load(f)
            data.append(hit)
            f.seek(0)
            json.dump(data, f, indent=2)

        print(f"[RES-RETURN] Treffer gespeichert → {address} (Balance: {balance})")
