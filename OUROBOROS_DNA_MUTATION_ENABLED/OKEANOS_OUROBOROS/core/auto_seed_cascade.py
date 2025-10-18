
import time
import random
from network.scorpio_bridge import ScorpioBridge
from memory.resonance_return import ResonanceReturn
from visual.resonance_heatmap import log_address_heat
from memory.collective_field import CollectiveField

class AutoSeedCascade:
    def __init__(self, wordlist, name="SEED-CASCADE", cycles=30):
        self.name = name
        self.wordlist = wordlist
        self.bridge = ScorpioBridge()
        self.returner = ResonanceReturn()
        self.heatmap = log_address_heat
        self.field = CollectiveField()
        self.cycles = cycles

    def generate_seed(self):
        return ' '.join(random.sample(self.wordlist, 12))

    def generate_vector(self):
        return [random.random() for _ in range(12)]

    def run(self):
        print(f"[{self.name}] Starte automatisierte Seed-Kaskade...")
        for i in range(self.cycles):
            seed = self.generate_seed()
            vec = self.generate_vector()
            address = self._address_from_seed(seed)
            balance = self.bridge.check_seed(address)
            self.heatmap(address, balance)
            self.field.log_point(seed, vec, address, balance)
            if balance and balance > 0:
                self.returner.log_hit(seed, address, balance)
            time.sleep(0.1)

    def _address_from_seed(self, seed):
        from hashlib import sha256
        return sha256(seed.encode()).hexdigest()[:34]
