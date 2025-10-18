
import os
import json
import random

MEMORY_GRAPH_PATH = "data/memory_graph.json"
DYNAMIC_WORDLIST_PATH = "data/dynamic_wordlist.txt"

class AutoSeedGenerator:
    def __init__(self, seed_length=12):
        self.seed_length = seed_length
        self.memory = self.load_memory()
        self.words = self.load_wordlist()
        self.core_pool = self.extract_core_words()

    def load_memory(self):
        if os.path.exists(MEMORY_GRAPH_PATH):
            with open(MEMORY_GRAPH_PATH, "r", encoding="utf-8") as f:
                return json.load(f)
        return []

    def load_wordlist(self):
        if os.path.exists(DYNAMIC_WORDLIST_PATH):
            with open(DYNAMIC_WORDLIST_PATH, "r", encoding="utf-8") as f:
                return list(set([line.strip() for line in f if line.strip()]))
        return []

    def extract_core_words(self):
        freq = {}
        for entry in self.memory:
            for word in entry["mutated_seed"].split():
                freq[word] = freq.get(word, 0) + 1
        # Nimm die 50 häufigsten Wörter als resonantes Zentrum
        sorted_words = sorted(freq.items(), key=lambda x: x[1], reverse=True)
        return [w for w, _ in sorted_words[:50]] + self.words

    def generate(self, num_variants=10):
        generated = []
        for _ in range(num_variants):
            seed = " ".join(random.sample(self.core_pool, self.seed_length))
            generated.append(seed)
        return generated
