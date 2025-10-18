
import threading
import time
import importlib
from memory.memory_graph import save_memory_entry
from memory.navigator import Navigator

class ClusterControllerResonant:
    def __init__(self, coin="ethereum", target_address=None, wordlist=None, cluster_strategy="resonant", num_cells=5, depth=1):
        self.coin = coin
        self.target_address = target_address.lower()
        self.plugin = importlib.import_module(f"coins.{coin}")
        self.wordlist = wordlist
        self.cluster_strategy = cluster_strategy
        self.num_cells = num_cells
        self.depth = depth
        self.running = False
        self.navigator = Navigator()

    def _cell_worker(self, cell_id, strategy):
        engine = self.plugin.Engine(self.wordlist)
        index = self.navigator.get_next_hotspot_index() or cell_id * 1000
        while self.running:
            seed = engine.generate_seed(index)
            mutated = engine.mutate_seed(seed, self.depth)
            address = self.plugin.generate_address(mutated).lower()

            if address == self.target_address:
                save_memory_entry(seed, mutated, address, index, cluster_id=strategy)
                print(f"[MATCH] Cell-{cell_id} ({strategy}): {mutated} -> {address}")
                break

            index += 1
            time.sleep(0.1)

    def run_cluster(self):
        print(f"[CLUSTER] Suche nach Ziel: {self.target_address}")
        self.running = True
        threads = []

        for i in range(self.num_cells):
            strategy = self.cluster_strategy
            t = threading.Thread(target=self._cell_worker, args=(i+1, strategy), daemon=True)
            threads.append(t)
            t.start()

        for t in threads:
            t.join()
