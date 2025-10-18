
from resonant_generator import ResonantSeedGenerator
from scorpio_bridge import ScorpioBridge
from resonance_return import ResonanceReturn
from resonance_heatmap import ResonanceHeatmap
from collective_field import CollectiveField
import time

class AutoSeedCascade:
    def __init__(self, wordlist, name="SEED-CASCADE", cycles=30):
        self.name = name
        self.generator = ResonantSeedGenerator(wordlist)
        self.bridge = ScorpioBridge()
        self.returner = ResonanceReturn()
        self.heatmap = ResonanceHeatmap()
        self.field = CollectiveField()
        self.cycles = cycles

    def run(self):
        print(f"[{self.name}] Autobohrung gestartet.")
        for i in range(self.cycles):
            seed, vec = self.generator.generate_resonant_seed()
            if not seed:
                print("[WARNUNG] Keine Seed-Vektoren im Feld.")
                break
            address = self._address_from_seed(seed)
            balance = self.bridge.check_seed(address)
            self.heatmap.log_address(address, balance)
            self.field.log_point(seed, vec, address, balance)
            if balance and balance > 0:
                self.returner.log_hit(seed, address, balance)
            time.sleep(0.1)

    def _address_from_seed(self, seed):
        import hashlib
        return hashlib.sha256(seed.encode()).hexdigest()[:34]
