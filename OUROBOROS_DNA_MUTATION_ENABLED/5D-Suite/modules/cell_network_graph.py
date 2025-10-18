import networkx as nx
import matplotlib.pyplot as plt
import random

class CellNetworkGraph:
    def __init__(self):
        self.G = nx.Graph()

    def simulate_graph(self, zellen):
        self.G.clear()
        for z in zellen:
            self.G.add_node(z["id"], type=z["type"], activity=z.get("activity", 0))
        # Zufällige Verbindungen simulieren (Clusterverknüpfung)
        ids = [z["id"] for z in zellen]
        for i in range(len(ids)):
            for j in range(i + 1, len(ids)):
                if random.random() < 0.15:
                    self.G.add_edge(ids[i], ids[j])

    def show(self):
        types = nx.get_node_attributes(self.G, 'type')
        colors = {'NavigatorCell': 'skyblue', 'MutatorCell': 'orange', 
                  'WatcherCell': 'red', 'CollectorCell': 'green', 
                  'SupervisorCell': 'purple'}
        node_colors = [colors.get(types[n], 'gray') for n in self.G.nodes()]

        plt.figure(figsize=(10, 7))
        nx.draw(self.G, with_labels=True, node_color=node_colors, node_size=600, font_size=8)
        plt.title("Zellnetzwerk – Struktur & Typus")
        plt.show()
