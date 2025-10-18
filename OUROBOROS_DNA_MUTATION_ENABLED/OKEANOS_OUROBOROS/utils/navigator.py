
import json

class Navigator:
    def __init__(self, heatmap_file="heatmap.json"):
        self.heatmap_file = heatmap_file

    def get_hot_zones(self, top_n=3):
        try:
            with open(self.heatmap_file, 'r') as f:
                data = json.load(f)

            sorted_zones = sorted(
                data.items(),
                key=lambda item: item[1]['count'],
                reverse=True
            )

            return [zone[0] for zone in sorted_zones[:top_n]]

        except Exception as e:
            print(f"[NAVIGATOR ERROR] {e}")
            return []

    def focus_on_zone(self, prefix):
        print(f"[NAVIGATOR] Fokussiere auf Präfix-Zone: {prefix}")
        # Hier kann später z. B. ein spezialisierter Seed-Generator implementiert werden
