class K3ResonanceAnalyzer:
    def __init__(self):
        self.patterns = {}

    def observe(self, fabric_links):
        """
        Beobachtet Verbindungen aus dem Fabric-Modul und erkennt dominante Muster.
        """
        stats = {}
        for a, b, dist in fabric_links:
            for seed in (a, b):
                prefix = seed[:6]
                stats[prefix] = stats.get(prefix, 0) + 1

        # Nur signifikante Muster merken
        self.patterns = {k: v for k, v in stats.items() if v >= 2}

    def report(self):
        """
        Gibt eine textuelle Zusammenfassung der wichtigsten Resonanzkerne zurück.
        """
        if not self.patterns:
            return ["Keine dominanten Muster erkannt."]
        return [f"Cluster {k} → {v}x Resonanzpunkte" for k, v in self.patterns.items()]
