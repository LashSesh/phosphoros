import time

class ResonanceOverlay:
    def __init__(self):
        self.entries = []

    def log(self, source, action, detail=""):
        timestamp = time.strftime("%H:%M:%S", time.localtime())
        self.entries.append(f"[{timestamp}] {source}: {action} {detail}")

    def get_overlay(self, limit=10):
        return "\n".join(self.entries[-limit:]) if self.entries else "Keine Systemaktivität aufgezeichnet."
