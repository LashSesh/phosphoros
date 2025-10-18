
import numpy as np
import json

class GabrielCell:
    def __init__(self, node_count=10):
        self.node_count = node_count
        self.weights = np.random.rand(node_count, node_count) * 0.01
        self.activations = np.zeros(node_count)

    def propagate_signal(self, input_vector):
        self.activations = np.tanh(np.dot(self.weights, input_vector))
        return self.activations

    def hebbian_update(self, learning_rate=0.01):
        delta_w = learning_rate * np.outer(self.activations, self.activations)
        self.weights += delta_w - 0.001 * self.weights

    def save_state(self, filename):
        state = {
            "weights": self.weights.tolist(),
            "activations": self.activations.tolist()
        }
        with open(filename, "w") as f:
            json.dump(state, f)

    def load_state(self, filename):
        with open(filename, "r") as f:
            state = json.load(f)
            self.weights = np.array(state["weights"])
            self.activations = np.array(state["activations"])



class GabrielCell:
    def __init__(self, cell_id):
        self.cell_id = cell_id
        self.memory = []  # gespeicherte Seeds mit Resonanz
        self.success_count = 0
        self.failure_count = 0

    def evaluate_seed(self, seed, resonance_score):
        if resonance_score > 0.7:
            self.memory.append(seed)
            self.success_count += 1
        else:
            self.failure_count += 1

    def mutate_seed(self, seed):
        # einfaches Beispiel: bei Erfolgshistorie ähnliche Seeds bevorzugen
        import random
        if self.memory and self.success_count > self.failure_count:
            base = random.choice(self.memory)
            mutated = base[:len(base)//2] + seed[len(seed)//2:]  # Kreuzung
        else:
            mutated = ''.join(random.choice("abcdefghijklmnopqrstuvwxyz0123456789") for _ in range(len(seed)))
        return mutated
