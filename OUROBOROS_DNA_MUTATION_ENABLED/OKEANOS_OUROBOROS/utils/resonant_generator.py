
import numpy as np
from collective_field import CollectiveField
from seed_modulator import SeedModulator

class ResonantSeedGenerator:
    def __init__(self, wordlist, top_k=5, variation=0.3):
        self.field = CollectiveField()
        self.modulator = SeedModulator(wordlist)
        self.top_k = top_k
        self.variation = variation

    def generate_resonant_seed(self):
        vectors = self.field.load_vectors()
        if vectors.shape[0] == 0:
            return None, None
        center = self._find_resonance_core(vectors)
        perturbed = center + np.random.normal(scale=self.variation, size=12)
        seed = self.modulator.vector_to_seed(perturbed)
        return seed, perturbed

    def _find_resonance_core(self, vectors):
        from sklearn.cluster import KMeans
        kmeans = KMeans(n_clusters=self.top_k, n_init=5).fit(vectors)
        scores = np.bincount(kmeans.labels_)
        hot_cluster = np.argmax(scores)
        core = kmeans.cluster_centers_[hot_cluster]
        return core
