
from hive_controller import HiveController
from seed_modulator import SeedModulator
from scorpio_bridge import ScorpioBridge
from resonance_return import ResonanceReturn
from resonance_heatmap import ResonanceHeatmap
from collective_field import CollectiveField
import time
import json
import numpy as np
import hashlib

class SpiralSwarmCell:
    def __init__(self, wordlist, name="SWARM-CELL", steps=60):
        self.name = name
        self.wordlist = wordlist
        self.hive = HiveController(steps=steps)
        self.path = self.hive.generate_offset_spiral()
        self.modulator = SeedModulator(wordlist)
        self.bridge = ScorpioBridge()
        self.returner = ResonanceReturn()
        self.heatmap = ResonanceHeatmap()
        self.field = CollectiveField()
        self.control_file = f"control_{self.name}.json"
        self._init_control()

    def _init_control(self):
        with open(self.control_file, "w") as f:
            json.dump({"stop": False}, f)

    def _check_stop(self):
        try:
            with open(self.control_file, "r") as f:
                return json.load(f).get("stop", False)
        except:
            return False

    def run(self):
        print(f"[{self.name}] startet Hive-gesteuerte Spiralbohrung.")
        base = self.modulator.vector_to_seed(np.zeros(12))
        for i, vec in enumerate(self.path):
            if self._check_stop():
                print(f"[{self.name}] Stoppsignal empfangen.")
                break
            seed = self.modulator.mutate_seed_from_vector(base, vec)
            address = self._address_from_seed(seed)
            balance = self.bridge.check_seed(address)
            self.heatmap.log_address(address, balance)
            self.field.log_point(seed, vec, address, balance)
            if balance and balance > 0:
                self.returner.log_hit(seed, address, balance)
            time.sleep(0.1)

    def _address_from_seed(self, seed):
        return hashlib.sha256(seed.encode()).hexdigest()[:34]
