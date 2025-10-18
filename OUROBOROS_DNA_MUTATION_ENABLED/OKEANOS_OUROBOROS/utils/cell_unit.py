
from hotzone_seedgen import HotzoneSeedGenerator
from scorpio_bridge import ScorpioBridge
from resonance_return import ResonanceReturn
from resonance_heatmap import ResonanceHeatmap
import time
import json
import os

class CellUnit:
    def __init__(self, wordlist, target_prefix, name="CELL-X", max_cycles=50):
        self.name = name
        self.target_prefix = target_prefix
        self.wordlist = wordlist
        self.max_cycles = max_cycles
        self.generator = HotzoneSeedGenerator(wordlist, prefix_targets=[target_prefix])
        self.bridge = ScorpioBridge()
        self.res_return = ResonanceReturn()
        self.heatmap = ResonanceHeatmap()
        self.status_file = f"status_{self.name}.json"
        self.control_file = f"control_{self.name}.json"
        self._init_control()

    def _init_control(self):
        with open(self.control_file, "w") as f:
            json.dump({"stop": False}, f)

    def _check_stop(self):
        try:
            with open(self.control_file, "r") as f:
                data = json.load(f)
            return data.get("stop", False)
        except:
            return False

    def _log_status(self, i, seed, address, balance):
        status = {
            "name": self.name,
            "cycle": i,
            "max_cycles": self.max_cycles,
            "prefix": self.target_prefix,
            "seed": seed,
            "address": address,
            "balance": balance
        }
        with open(self.status_file, "w") as f:
            json.dump(status, f, indent=2)

    def run(self):
        print(f"[{self.name}] Aktiviert – Zielzone: {self.target_prefix}")
        for i in range(self.max_cycles):
            if self._check_stop():
                print(f"[{self.name}] Stoppsignal empfangen.")
                break
            seed, address = self.generator.generate_seed_for_zone()
            if seed and address:
                balance = self.bridge.check_seed(address)
                self.heatmap.log_address(address, balance)
                self._log_status(i, seed, address, balance)
                if balance and balance > 0:
                    self.res_return.log_hit(seed, address, balance)
            time.sleep(0.1)
