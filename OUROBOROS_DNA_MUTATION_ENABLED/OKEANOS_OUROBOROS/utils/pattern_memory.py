
import numpy as np
import random
from path_recorder import PathRecorder

class PatternMemory:
    def __init__(self):
        self.recorder = PathRecorder()

    def get_cell_patterns(self, cell_name):
        path = self.recorder.get_path_by_cell(cell_name)
        vectors = [step["vector"] for step in path]
        return np.array(vectors) if vectors else np.zeros((0, 12))

    def repeat_path(self, cell_name, mutation=0.0):
        pattern = self.get_cell_patterns(cell_name)
        if pattern.shape[0] == 0:
            return []
        if mutation > 0:
            noise = np.random.normal(scale=mutation, size=pattern.shape)
            pattern = pattern + noise
        return pattern

    def combine_paths(self, cell_a, cell_b, mutation=0.1):
        pa = self.get_cell_patterns(cell_a)
        pb = self.get_cell_patterns(cell_b)
        if pa.shape[0] == 0 or pb.shape[0] == 0:
            return []
        mix_len = min(len(pa), len(pb))
        combined = (pa[:mix_len] + pb[:mix_len]) / 2.0
        noise = np.random.normal(scale=mutation, size=combined.shape)
        return combined + noise
