
import threading
import time
from utils import seed_archive
import importlib
from core.spiraforce_engine import SpiraforceEngine
from core.wordlist_loader import load_wordlist_from_txt
from core.target_hit_logger import log_target_hit
from core.wordlist_tuner import update_dynamic_wordlist
from memory_graph import save_memory_entry
from resonance_navigator import ResonanceNavigator

class ClusterControllerResonant:
    def __init__(self, coin="ethereum", target_address=None, wordlist_path=None, cluster_strategy="resonant", num_cells=5, depth=1):
        self.coin = coin
        self.target_address = target_address.lower()
        self.plugin = importlib.import_module(f"coins.{coin}")
        self.wordlist = load_wordlist_from_txt(wordlist_path) if wordlist_path else None
        self.cluster_strategy = cluster_strategy
        self.num_cells = num_cells
        self.depth = depth
        self.running = False
        self.navigator = ResonanceNavigator()

    def _cell_worker(self, cell_id, strategy):
        engine = SpiraforceEngine(wordlist=self.wordlist)
        index = self.navigator.get_next_hotspot_index() or cell_id * 1000
        while self.running:
            seed = engine.generate_seed(index)
            seed_archive.store_seed(seed, source='cluster_controller')
            mutated = engine.mutate_seed(seed, self.depth)
            address = self.plugin.generate_address(mutated).lower()

            if address == self.target_address:
                log_target_hit(mutated, address, self.coin)
                update_dynamic_wordlist(mutated, depth=self.depth)
                save_memory_entry(seed, mutated, address, index, cluster_id=strategy)
                print(f"[MATCH] Cell-{cell_id} ({strategy}): {mutated} -> {address}")
                break

            index += 1
            time.sleep(0.1)

    def run_cluster(self):
        print(f"[CLUSTER] Resonante Bohrung auf Ziel: {self.target_address}")
        self.running = True
        threads = []

        for i in range(self.num_cells):
            strategy = self.cluster_strategy
            t = threading.Thread(target=self._cell_worker, args=(i+1, strategy), daemon=True)
            threads.append(t)
            t.start()

        for t in threads:
            t.join()