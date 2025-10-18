
import uuid
import time

class MetaCluster:
    def __init__(self, zelltyp, mitglieder):
        self.id = str(uuid.uuid4())[:6]
        self.typ = zelltyp
        self.mitglieder = mitglieder  # Liste von Zell-IDs
        self.entstehung = time.time()
        self.iterationen = 0
        self.durchschnitt_resonanz = None

    def update(self, resonanzen):
        self.iterationen += 1
        if resonanzen:
            self.durchschnitt_resonanz = sum(resonanzen) / len(resonanzen)
