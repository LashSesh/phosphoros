
import hashlib
import numpy as np

class SeedEmbedder:
    def __init__(self, word_to_index=None):
        self.word_to_index = word_to_index or {}

    def embed_seed(self, seed_phrase):
        words = seed_phrase.strip().split()
        if len(words) != 12:
            raise ValueError("Seed muss aus genau 12 Wörtern bestehen.")
        vector = []
        for word in words:
            index = self._word_index(word)
            vector.append(index)
        return np.array(vector, dtype=np.float32)

    def _word_index(self, word):
        if word in self.word_to_index:
            return self.word_to_index[word]
        else:
            # Hash-basiertes Fallback für unbekannte Wörter
            hashed = hashlib.sha256(word.encode()).hexdigest()
            return int(hashed[:6], 16) % 2048  # Simulierter Index im Seed-Space
