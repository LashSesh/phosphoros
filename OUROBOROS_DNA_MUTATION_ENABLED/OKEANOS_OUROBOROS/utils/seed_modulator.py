
import numpy as np
import hashlib
import random

class SeedModulator:
    def __init__(self, wordlist):
        self.wordlist = wordlist
        self.word_count = len(wordlist)

    def vector_to_seed(self, vec):
        # Normiere und mappe Vektor auf Wörter aus Wordlist
        normed = (vec - np.min(vec)) / (np.max(vec) - np.min(vec) + 1e-6)
        indices = (normed * self.word_count).astype(int)
        indices = np.clip(indices, 0, self.word_count - 1)
        words = [self.wordlist[i] for i in indices[:12]]
        return ' '.join(words)

    def mutate_seed_from_vector(self, base_seed, vec_delta):
        base_words = base_seed.strip().split()
        base_vec = self.seed_to_index_vector(base_words)
        mutated = np.clip(base_vec + vec_delta[:12], 0, self.word_count - 1).astype(int)
        new_words = [self.wordlist[i] for i in mutated]
        return ' '.join(new_words)

    def seed_to_index_vector(self, words):
        indices = []
        for w in words:
            try:
                indices.append(self.wordlist.index(w))
            except:
                hashed = hashlib.sha256(w.encode()).hexdigest()
                idx = int(hashed[:6], 16) % self.word_count
                indices.append(idx)
        return np.array(indices, dtype=np.float32)
