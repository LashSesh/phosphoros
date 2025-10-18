import time
from collections import defaultdict

class PulsTracker:
    def __init__(self):
        self.timeline = []
        self.activity_log = defaultdict(list)

    def register_event(self, seed, cell_type):
        timestamp = time.time()
        self.timeline.append((timestamp, seed, cell_type))
        self.activity_log[cell_type].append(timestamp)

    def get_pulse_summary(self):
        if not self.timeline:
            return "Kein Zellpuls registriert."

        latest = self.timeline[-5:]
        summary = ["Letzte Zellaktionen:"]
        for ts, seed, cell in latest:
            summary.append(f"{time.strftime('%H:%M:%S', time.localtime(ts))} | {cell}: {seed[:10]}...")
        return "\n".join(summary)

    def detect_frequency_anomalies(self):
        """
        Gibt an, ob in letzter Zeit Zellen ungewöhnlich schnell/langsam reagierten
        """
        issues = []
        for cell_type, stamps in self.activity_log.items():
            if len(stamps) < 2:
                continue
            deltas = [stamps[i] - stamps[i-1] for i in range(1, len(stamps))]
            avg = sum(deltas) / len(deltas)
            if avg < 1.0:
                issues.append(f"{cell_type}: ungewöhnlich hohe Frequenz")
            elif avg > 15:
                issues.append(f"{cell_type}: Aktivität sehr gering")
        return issues
