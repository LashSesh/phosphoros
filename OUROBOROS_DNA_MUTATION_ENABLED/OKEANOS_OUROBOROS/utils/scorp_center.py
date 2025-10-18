
from collections import defaultdict

class ScorpCenter:
    def __init__(self):
        self.phase_clock = 0
        self.ilus = []

    def update_phase(self):
        self.phase_clock += 1

class ScorpioBalancer:
    def __init__(self, slots=4):
        self.phase = ScorpCenter().phase_clock
        self.slots = slots
        self.load_slots = [[] for _ in range(slots)]

    def distribute(self, zellen):
        self.load_slots = [[] for _ in range(self.slots)]
        if not zellen:
            return self.load_slots
        for i, z in enumerate(zellen):
            self.load_slots[i % self.slots].append(z)
        return self.load_slots

    def distribute_by_type(self, zellen):
        grouped = defaultdict(list)
        for z in zellen:
            grouped[z.typ].append(z)
        flat = []
        for group in grouped.values():
            flat.extend(group)
        return self.distribute(flat)

    def distribute_quadrupol(self, zellen):
        phase = ScorpCenter().phase_clock % 2
        aktive_typen = ["Navigator", "Collector"] if phase == 0 else ["Mutator", "Watcher"]
        selektiert = [z for z in zellen if z.typ in aktive_typen]
        return self.distribute(selektiert)
