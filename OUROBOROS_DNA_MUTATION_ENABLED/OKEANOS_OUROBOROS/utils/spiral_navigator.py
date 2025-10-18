
import numpy as np

class SpiralNavigator:
    def __init__(self, steps=100, a=1.0, b=0.2):
        self.steps = steps
        self.a = a
        self.b = b

    def generate_spiral_path(self):
        # Erzeuge logarithmische Spirale im 2D-Raum (r = a * e^(b * theta))
        theta = np.linspace(0, 4 * np.pi, self.steps)
        r = self.a * np.exp(self.b * theta)
        x = r * np.cos(theta)
        y = r * np.sin(theta)
        coords = np.stack((x, y), axis=-1)
        return coords
