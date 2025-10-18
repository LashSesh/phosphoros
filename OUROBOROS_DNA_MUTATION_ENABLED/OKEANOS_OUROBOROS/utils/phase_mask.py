import random

class PhaseMask:
    def apply(self, packet):
        mask = ''.join(random.sample(packet, len(packet)))
        return mask