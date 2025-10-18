
import random
from utils import seed_archive

class GabrielMetaCell:
    def __init__(self, cell_id):
        self.cell_id = cell_id
        self.inner_memory = []
        self.meta_memory = {"success": 0, "failure": 0}
        self.mutation_bias = "neutral"

    def evaluate_seed(self, seed, resonance_score):
        if resonance_score > 0.7:
            self.inner_memory.append(seed)
            self.meta_memory["success"] += 1
            seed_archive.store_seed(seed, source="GabrielMetaCell", result="hit")
        else:
            self.meta_memory["failure"] += 1
            seed_archive.store_seed(seed, source="GabrielMetaCell", result="miss")

        self.adjust_bias()

    def adjust_bias(self):
        total = self.meta_memory["success"] + self.meta_memory["failure"]
        if total >= 5:
            ratio = self.meta_memory["success"] / total
            if ratio > 0.6:
                self.mutation_bias = "exploit"
            elif ratio < 0.3:
                self.mutation_bias = "explore"
            else:
                self.mutation_bias = "neutral"

    def mutate_seed(self, base_seed):
        charset = "abcdefghijklmnopqrstuvwxyz0123456789"
        if self.mutation_bias == "exploit" and self.inner_memory:
            base = random.choice(self.inner_memory)
            part = base[:len(base)//2]
            mutated = part + ''.join(random.choice(charset) for _ in range(len(base_seed)//2))
        elif self.mutation_bias == "explore":
            mutated = ''.join(random.choice(charset) for _ in range(len(base_seed)))
        else:
            mutated = base_seed[:2] + ''.join(random.choice(charset) for _ in range(len(base_seed)-2))
        return mutated
