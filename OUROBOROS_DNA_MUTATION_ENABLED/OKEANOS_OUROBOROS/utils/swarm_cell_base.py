memory = memory_field.MemoryField()


import random
from utils import seed_archive, memory_field

class BaseCell:
    def __init__(self, cell_id):
        self.cell_id = cell_id
        self.memory = []

    def evaluate_seed(self, seed, resonance_score):
        pass

    def act(self, seed):
        pass

class NavigatorCell(BaseCell):
    def evaluate_seed(self, seed, resonance_score):
        if resonance_score < 0.3:
            self.memory.append(seed)
        memory.add(seed, resonance_score, "Mutator")

    def act(self, seed):
        return ''.join(random.choice("abcdef0123456789") for _ in range(len(seed)))

class MutatorCell(BaseCell):
    def evaluate_seed(self, seed, resonance_score):
        if resonance_score > 0.6:
            self.memory.append(seed)
        memory.add(seed, resonance_score, "Mutator")

    def act(self, seed):
        base = random.choice(self.memory) if self.memory else seed
        return base[:len(seed)//2] + ''.join(random.choice("xyz789") for _ in range(len(seed)//2))

class WatcherCell(BaseCell):
    def evaluate_seed(self, seed, resonance_score):
        if resonance_score < 0.1:
            self.memory.append(seed_archive.classify_prefix(seed))

    def act(self, seed):
        # Blockiere häufige Fehlschlag-Präfixe
        prefix = seed[:3]
        if prefix in self.memory:
            return None
        return seed

class CollectorCell(BaseCell):
    def evaluate_seed(self, seed, resonance_score):
        if resonance_score > 0.75:
            self.memory.append(seed)
        memory.add(seed, resonance_score, "Mutator")
            seed_archive.store_seed(seed, source="CollectorCell", result="hit")
            memory.add(seed, resonance_score, "Collector")

    def act(self, seed):
        return seed

class SupervisorCell(BaseCell):
    def __init__(self, cell_id):
        super().__init__(cell_id)
        self.activation_map = {"Navigator": True, "Mutator": True, "Watcher": True, "Collector": True}

    def evaluate_performance(self, cell_reports):
        # Simpler Supervisor: aktiviert/deaktiviert Zelltypen je nach Erfolg
        for cell_type, stats in cell_reports.items():
            success = stats.get("success", 0)
            fail = stats.get("fail", 0)
            ratio = success / (success + fail + 1e-5)
            self.activation_map[cell_type] = ratio > 0.2  # deactivate if success ratio too low

    def is_active(self, cell_type):
        return self.activation_map.get(cell_type, True)
