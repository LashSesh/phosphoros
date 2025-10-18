
import random

class GabrielCell:
    def __init__(self, cell_id):
        self.cell_id = cell_id
        self.memory = []
        self.success_count = 0
        self.failure_count = 0

    def evaluate_seed(self, seed, resonance_score):
        if resonance_score > 0.7:
            self.memory.append(seed)
            self.success_count += 1
        else:
            self.failure_count += 1

    def mutate_seed(self, seed):
        if self.memory and self.success_count > self.failure_count:
            base = random.choice(self.memory)
            mutated = base[:len(base)//2] + seed[len(seed)//2:]  # Kreuzung
        else:
            mutated = ''.join(random.choice("abcdefghijklmnopqrstuvwxyz0123456789") for _ in range(len(seed)))
        return mutated
