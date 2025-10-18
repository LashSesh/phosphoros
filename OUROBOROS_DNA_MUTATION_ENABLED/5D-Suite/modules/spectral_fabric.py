import hashlib
import numpy as np

class SpectralFabric:
    def __init__(self):
        self.seed_vectors = []

    def encode_seed(self, seed: str) -> np.ndarray:
        # Primitive "geometrische Projektion" aus Hashstruktur
        hashed = hashlib.sha256(seed.encode()).hexdigest()
        numeric = [int(hashed[i:i+2], 16)/255.0 for i in range(0, len(hashed), 2)]
        return np.array(numeric)

    def add_seed(self, seed: str):
        vec = self.encode_seed(seed)
        self.seed_vectors.append((seed, vec))

    def build_fabric(self):
        if len(self.seed_vectors) < 2:
            return []

        fabric = []
        for i, (seed_a, vec_a) in enumerate(self.seed_vectors):
            for j, (seed_b, vec_b) in enumerate(self.seed_vectors):
                if i >= j:
                    continue
                dist = np.linalg.norm(vec_a - vec_b)
                if dist < 1.2:  # Schwelle für "Resonanz-Fäden"
                    fabric.append((seed_a, seed_b, round(dist, 4)))
        return fabric



    def export_projected_seeds(self, count=5):
        """
        Erzeugt plausible, geometrisch induzierte Seeds zur Rückgabe an Zellnetzwerk.
        """
        hints = self.get_projection_hint(top_k=count)
        full_seeds = ["recon_" + h for h in hints]
        return full_seeds

    def get_projection_hint(self, top_k=3):
        # Erzeuge neue Vektoren aus Mittelwert nahegelegener Seeds
        if len(self.seed_vectors) < 3:
            return []

        vectors = [vec for _, vec in self.seed_vectors]
        avg = np.mean(vectors, axis=0)

        hints = []
        for i in range(top_k):
            perturb = avg + np.random.normal(0, 0.03, size=avg.shape)
            perturbed = "".join([f"{int(v*255)%16:x}" for v in perturb[:32]])
            hints.append(perturbed)
        return hints
