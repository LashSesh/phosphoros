import matplotlib.pyplot as plt
import networkx as nx
import hashlib
import numpy as np
import random

class HybridMap:
    def __init__(self):
        self.G = nx.Graph()
        self.signaturen = {}

    def encode_seed(self, seed):
        h = hashlib.sha256(seed.encode()).hexdigest()
        return [int(h[i:i+2], 16)/255.0 for i in range(0, len(h), 2)]

    def build_graph(self, zellen):
        self.G.clear()
        self.signaturen.clear()
        for i, z in enumerate(zellen):
            node_id = f"Z{i}"
            self.G.add_node(node_id, type=z["type"])
            if z.get("seed"):
                self.signaturen[node_id] = self.encode_seed(z["seed"])
        ids = list(self.G.nodes())
        for i in range(len(ids)):
            for j in range(i+1, len(ids)):
                if random.random() < 0.15:
                    self.G.add_edge(ids[i], ids[j])

    def plot_hybrid(self):
        pos = nx.spring_layout(self.G, seed=42)
        types = nx.get_node_attributes(self.G, 'type')
        colors = {'NavigatorCell': 'skyblue', 'MutatorCell': 'orange', 
                  'WatcherCell': 'red', 'CollectorCell': 'green', 
                  'SupervisorCell': 'purple'}

        plt.figure(figsize=(14, 8))
        for node in self.G.nodes():
            x, y = pos[node]
            vec = self.signaturen.get(node)
            if vec:
                spectrum = np.array(vec[:20])
                offset = np.linspace(-0.1, 0.1, len(spectrum))
                plt.plot(x + offset, y + spectrum * 0.1, alpha=0.6, color='gray')
            plt.scatter(x, y, color=colors.get(types.get(node), 'gray'), s=250, edgecolors='black')
            plt.text(x, y+0.03, node, fontsize=8, ha='center')
        for edge in self.G.edges():
            x0, y0 = pos[edge[0]]
            x1, y1 = pos[edge[1]]
            plt.plot([x0, x1], [y0, y1], color='lightgray', alpha=0.5)
        plt.title("Hybridkarte – Zellnetzwerk + Spektrale Fäden")
        plt.axis('off')
        plt.show()
