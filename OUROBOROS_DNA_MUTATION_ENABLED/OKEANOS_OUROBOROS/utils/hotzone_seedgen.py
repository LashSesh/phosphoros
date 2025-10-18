
import hashlib
import random

class HotzoneSeedGenerator:
    def __init__(self, wordlist, prefix_targets=None):
        self.wordlist = wordlist
        self.prefix_targets = prefix_targets or []

    def generate_seed_for_zone(self, prefix_length=4, max_attempts=10000):
        for _ in range(max_attempts):
            seed_words = random.sample(self.wordlist, 12)
            seed_phrase = ' '.join(seed_words)
            address = self._seed_to_address(seed_phrase)
            prefix = address[:prefix_length]
            if prefix in self.prefix_targets:
                return seed_phrase, address
        return None, None

    def _seed_to_address(self, seed):
        return hashlib.sha256(seed.encode()).hexdigest()[:34]
