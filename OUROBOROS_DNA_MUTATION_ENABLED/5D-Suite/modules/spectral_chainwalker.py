import hashlib
import numpy as np

class SpectralChainWalker:
    def __init__(self):
        self.reference_vectors = []

    def encode(self, seed: str) -> np.ndarray:
        h = hashlib.sha256(seed.encode()).hexdigest()
        numeric = [int(h[i:i+2], 16)/255.0 for i in range(0, len(h), 2)]
        return np.array(numeric)

    def load_reference_seeds(self, seed_list):
        """
        Lädt bekannte Seeds in den Referenzraum (MemoryField o.ä.)
        """
        self.reference_vectors = [(s, self.encode(s)) for s in seed_list]

    def extrapolate_from_fragment(self, fragment: str, max_out=5):
        """
        Versucht Seeds zu finden, die zu einer Fragmentstruktur passen
        """
        frag_vector = self.encode(fragment)
        candidates = []

        for s, vec in self.reference_vectors:
            score = np.linalg.norm(frag_vector - vec)
            candidates.append((s, score))

        # Beste Übereinstimmungen zurückgeben
        candidates.sort(key=lambda x: x[1])
        return [s for s, _ in candidates[:max_out]]
