
from cell_unit import CellUnit
from navigator import Navigator
import threading

class CellController:
    def __init__(self, wordlist, num_cells=3, cycles_per_cell=30):
        self.wordlist = wordlist
        self.num_cells = num_cells
        self.cycles_per_cell = cycles_per_cell
        self.navigator = Navigator()

    def launch_cells(self):
        hotzones = self.navigator.get_hot_zones(top_n=self.num_cells)
        threads = []
        for i, zone in enumerate(hotzones):
            cell = CellUnit(self.wordlist, target_prefix=zone, name=f"CELL-{i}", max_cycles=self.cycles_per_cell)
            t = threading.Thread(target=cell.run)
            threads.append(t)
            t.start()

        for t in threads:
            t.join()



# Beispiel für die Nutzung der GabrielMetaCell
meta_cell = gabriel_meta_cell.GabrielMetaCell(cell_id="meta01")

# Simulierter Seed-Test (dieser Block sollte später mit realem Seedtest gekoppelt werden)
test_seed = "bc1qxyzabc123"
resonance_score = 0.8  # simuliertes Ergebnis
meta_cell.evaluate_seed(test_seed, resonance_score)
mutated_seed = meta_cell.mutate_seed(test_seed)
print(f"[MetaCell] Mutated Seed: {mutated_seed}")
