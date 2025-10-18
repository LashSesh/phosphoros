import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
import hashlib

def project_seed_to_xyz(seed: str):
    h = hashlib.sha256(seed.encode()).hexdigest()
    return (
        int(h[0:2], 16) / 255,
        int(h[2:4], 16) / 255,
        int(h[4:6], 16) / 255
    )

def visualize_fabric_links(links):
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    for s1, s2, dist in links:
        x1, y1, z1 = project_seed_to_xyz(s1)
        x2, y2, z2 = project_seed_to_xyz(s2)

        ax.plot([x1, x2], [y1, y2], [z1, z2], alpha=0.6)

    ax.set_title("μcotop – Spektrale Seed-Topologie")
    plt.show()
