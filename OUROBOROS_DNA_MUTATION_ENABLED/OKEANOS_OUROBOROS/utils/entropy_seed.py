import random
import time

class EntropySeed:
    def initialize(self):
        seed = time.time() + random.SystemRandom().random()
        random.seed(seed)