import hashlib
import random
import string

class DNAMutator:
    def __init__(self):
        self.population = []

    def mutate_dna(self, dna, strength=0.05):
        # Mutiert eine DNA (Liste aus floats 0.0-1.0)
        return [min(1.0, max(0.0, v + random.uniform(-strength, strength))) for v in dna]

    def dna_to_seed(self, dna):
        # Einfacher Seed-Rebuilder: Wandelt DNA zurück in hex-ähnlichen Seed
        return ''.join(random.choice(string.ascii_lowercase + string.digits) for _ in range(64))

    def generate_population(self, base_dna, count=10):
        self.population.clear()
        for _ in range(count):
            mutated = self.mutate_dna(base_dna)
            seed = self.dna_to_seed(mutated)
            self.population.append(seed)

    def get_mutants(self):
        return self.population
