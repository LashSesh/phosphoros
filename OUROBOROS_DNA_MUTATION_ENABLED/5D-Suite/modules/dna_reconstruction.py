import hashlib
import numpy as np

class DNAReconstructor:
    def __init__(self):
        self.genbank = {}

    def extract_dna(self, seed):
        h = hashlib.sha256(seed.encode()).hexdigest()
        return [int(h[i:i+2], 16)/255.0 for i in range(0, 100, 2)]

    def store(self, zelle_id, seed):
        self.genbank[zelle_id] = self.extract_dna(seed)

    def compare(self, id1, id2):
        d1 = self.genbank.get(id1)
        d2 = self.genbank.get(id2)
        if not d1 or not d2:
            return 0.0
        sim = 1.0 - np.linalg.norm(np.array(d1) - np.array(d2)) / np.sqrt(len(d1))
        return max(0.0, min(sim, 1.0))

    def closest_match(self, reference_id):
        reference = self.genbank.get(reference_id)
        if not reference:
            return None, 0.0
        best, score = None, 0.0
        for other_id, dna in self.genbank.items():
            if other_id == reference_id:
                continue
            sim = self.compare(reference_id, other_id)
            if sim > score:
                best, score = other_id, sim
        return best, score
