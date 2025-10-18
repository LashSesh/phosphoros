
import threading
import time
import importlib
from core.spiraforce_engine import SpiraforceEngine
from core.wordlist_loader import load_wordlist_from_txt
from core.target_hit_logger import log_target_hit
from core.wordlist_tuner import update_dynamic_wordlist
from memory_graph import save_memory_entry
from resonance_navigator import ResonanceNavigator
from auto_seed_generator import AutoSeedGenerator

class ClusterControllerResonantAuto:
    def __init__(self, coin="ethereum", target_address=None, wordlist_path=None, num_cells=5, depth=1):
        self.coin = coin
        self.target_address = target_address.lower()
        self.plugin = importlib.import_module(f"coins.{coin}")
        self.wordlist = load_wordlist_from_txt(wordlist_path) if wordlist_path else None
        self.num_cells = num_cells
        self.depth = depth
        self.running = False
        self.navigator = ResonanceNavigator()
        self.autogen = AutoSeedGenerator()

    def _cell_worker(self, cell_id):
        engine = SpiraforceEngine(wordlist=self.wordlist)
        index = self.navigator.get_next_hotspot_index() or cell_id * 1000
        fallback_seeds = self.autogen.generate(10)
        fallback_index = 0

        while self.running:
            if fallback_index < len(fallback_seeds):
                mutated = fallback_seeds[fallback_index]
                fallback_index += 1
            else:
                seed = engine.generate_seed(index)
                mutated = engine.mutate_seed(seed, self.depth)
                index += 1

            address = self.plugin.generate_address(mutated).lower()

            if address == self.target_address:
                log_target_hit(mutated, address, self.coin)
                update_dynamic_wordlist(mutated, depth=self.depth)
                save_memory_entry(mutated, mutated, address, index, cluster_id="auto_seed")
                print(f"[MATCH] Cell-{cell_id} (auto): {mutated} -> {address}")
                break

            time.sleep(0.1)

    def run_cluster(self):
        print(f"[CLUSTER] Auto-Seed-Bohrung auf Ziel: {self.target_address}")
        self.running = True
        threads = []

        for i in range(self.num_cells):
            t = threading.Thread(target=self._cell_worker, args=(i+1,), daemon=True)
            threads.append(t)
            t.start()

        for t in threads:
            t.join()
