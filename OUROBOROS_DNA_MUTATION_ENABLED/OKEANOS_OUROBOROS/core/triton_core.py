
import json
import random
from core.auto_seed_cascade import AutoSeedCascade
from memory.pattern_memory import PatternMemory
from memory.collective_field import CollectiveField

class TRITONCore:
    def __init__(self, wordlist, name="TRITON", mode="auto", cycles=40):
        self.name = name
        self.mode = mode
        self.cycles = cycles
        self.wordlist = wordlist
        self.memory = PatternMemory()
        self.field = CollectiveField()
        self.signature_path = f"{self.name}_signature.json"

    def run(self):
        print(f"[{self.name}] Initialisiert in Modus: {self.mode}")
        if self.mode == "auto":
            self.run_auto()
        elif self.mode == "memory":
            self.run_memory()
        elif self.mode == "hybrid":
            self.run_hybrid()
        else:
            print("[Fehler] Unbekannter Modus:", self.mode)

    def run_auto(self):
        cascade = AutoSeedCascade(self.wordlist, name=f"{self.name}-AUTO", cycles=self.cycles)
        cascade.run()
        self._update_signature()

    def run_memory(self):
        cell_ids = self._get_known_cells()
        if not cell_ids:
            print("[Memory] Keine Zellpfade vorhanden.")
            return
        cell = random.choice(cell_ids)
        pattern = self.memory.repeat_path(cell, mutation=0.2)
        if pattern is None:
            print("[Memory] Leeres Muster.")
            return
        self._simulate_pattern_drill(pattern, origin=cell)

    def run_hybrid(self):
        cells = self._get_known_cells()
        if len(cells) < 2:
            print("[Hybrid] Zu wenige Musterpfade.")
            return
        a, b = random.sample(cells, 2)
        pattern = self.memory.combine_paths(a, b, mutation=0.15)
        self._simulate_pattern_drill(pattern, origin=f"{a}+{b}")

    def _simulate_pattern_drill(self, pattern, origin="UNKNOWN"):
        from crypto.seed_crypto import SeedCrypto
        from network.scorpio_bridge import ScorpioBridge
        from visual.resonance_heatmap import log_address_heat

        crypto = SeedCrypto()
        bridge = ScorpioBridge()

        for vec in pattern:
            seed = ''.join(random.choices(self.wordlist, k=12))
            address = crypto.seed_to_address(seed)["address"]
            balance = bridge.check_seed(address)
            log_address_heat(address, balance)
            self.field.log_point(seed, vec, address, balance)
            if balance and balance > 0:
                from memory.resonance_return import ResonanceReturn
                ResonanceReturn().log_hit(seed, address, balance)

    def _get_known_cells(self):
        try:
            from memory.path_recorder import PathRecorder
            recorder = PathRecorder()
            data = recorder.load_all()
            return list(set([d["cell"] for d in data]))
        except:
            return []

    def _update_signature(self):
        sig = {
            "identity": self.name,
            "mode": self.mode,
            "recorded_vectors": len(self.field.load_vectors()),
            "state": "active"
        }
        with open(self.signature_path, "w") as f:
            json.dump(sig, f, indent=2)
