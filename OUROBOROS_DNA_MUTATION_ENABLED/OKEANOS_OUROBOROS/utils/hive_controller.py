
import numpy as np
from collective_field import CollectiveField
from spiral_navigator import SpiralNavigator

class HiveController:
    def __init__(self, steps=60, spread=1.0):
        self.field = CollectiveField()
        self.steps = steps
        self.spread = spread

    def suggest_path_offset(self):
        vectors = self.field.load_vectors()
        if vectors.shape[0] == 0:
            return np.zeros(12)  # Leeres Feld = Ursprung
        mean_vector = np.mean(vectors, axis=0)
        perturb = np.random.normal(scale=self.spread, size=12)
        return mean_vector + perturb

    def generate_offset_spiral(self):
        offset = self.suggest_path_offset()
        nav = SpiralNavigator(steps=self.steps)
        path2d = nav.generate_spiral_path()
        path12d = [offset + np.tile(p, 6)[:12] for p in path2d]
        return path12d
