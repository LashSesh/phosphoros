
import os
import uuid
import json
import random
from concurrent.futures import ThreadPoolExecutor
from utils.scorp_center import ScorpioBalancer

SAVE_PATH = "data/zellen.json"

class GabrielZelle:
    def __init__(self, typ, id=None, iterationen=0, status="aktiv", letzte_aktion=""):
        self.id = id or str(uuid.uuid4())[:8]
        self.typ = typ
        self.iterationen = iterationen
        self.status = status
        self.letzte_aktion = letzte_aktion

    def tick(self):
        self.iterationen += 1
        if self.typ == "Navigator":
            self.letzte_aktion = f"Seed exploriert: {self._generate_seed()}"
        elif self.typ == "Mutator":
            try:
                from streamlit import session_state as st_sess
                pool = st_sess.cell_manager.get_resonanzpool()
                if pool:
                    seed = "-".join(random.choices(pool, k=3))
                    self.letzte_aktion = f"Gelerntes Seed erzeugt: {seed}"
                else:
                    original = self._generate_seed()
                    mutated = original[::-1]
                    self.letzte_aktion = f"Mutierte Seed: {original} → {mutated}"
            except:
                self.letzte_aktion = "Fehler bei Lernverstärkung"
        elif self.typ == "Watcher":
            from modules.echolot_parser import parse_seed
            seed = self._generate_seed()
            matches = parse_seed(seed)
            if matches:
                self.letzte_aktion = f"Pattern erkannt: {', '.join(matches)}"
            else:
                self.letzte_aktion = "Keine relevanten Muster"
        elif self.typ == "Collector":
            self.letzte_aktion = f"Seed gespeichert: {self._generate_seed()}"
        elif self.typ == "Supervisor":
            from streamlit import session_state as st_sess
            try:
                cm = st_sess.cell_manager
                cluster_count = len(cm.clusters)
                watcher_count = sum(1 for z in cm.zellen if z.typ == "Watcher")
                if cluster_count > 2 and watcher_count < 20:
                    cm.batch_add_zellen("Watcher", 5)
                    self.letzte_aktion = "5 Watcher erzeugt (Cluster ↑)"
                else:
                    cm.tick_parallel()
                    self.letzte_aktion = "Globaler Tick aller Zellen"
            except:
                self.letzte_aktion = "Supervisor konnte nicht zugreifen"

    def _generate_seed(self):
        wörter = ["alpha", "bravo", "charlie", "delta", "echo", "foxtrot"]
        return "-".join(random.choices(wörter, k=3))

    def to_dict(self):
        return {
            "id": self.id,
            "typ": self.typ,
            "iterationen": self.iterationen,
            "status": self.status,
            "letzte_aktion": self.letzte_aktion
        }

    @staticmethod
    def from_dict(d):
        return GabrielZelle(
            typ=d["typ"],
            id=d.get("id"),
            iterationen=d.get("iterationen", 0),
            status=d.get("status", "aktiv"),
            letzte_aktion=d.get("letzte_aktion", "")
        )

class CellManager:
    def __init__(self):
        self.zellen = []
        self.clusters = []
        self.load_zellen()

    def add_zelle(self, zelltyp):
        self.zellen.append(GabrielZelle(zelltyp))
        self.save_zellen()

    def batch_add_zellen(self, zelltyp, anzahl):
        for _ in range(anzahl):
            self.add_zelle(zelltyp)

    def remove_zelle(self, zellen_id):
        self.zellen = [z for z in self.zellen if z.id != zellen_id]
        self.save_zellen()

    def tick_all(self):
        for z in self.zellen:
            z.tick()
        self.save_zellen()

    def tick_parallel(self):
        balancer = ScorpioBalancer(slots=4)
        verteilt = balancer.distribute_quadrupol(self.zellen)

        def tick_gruppe(gruppe):
            for z in gruppe:
                z.tick()

        with ThreadPoolExecutor(max_workers=4) as ex:
            futures = [ex.submit(tick_gruppe, grp) for grp in verteilt]
            for f in futures:
                f.result()

        self.save_zellen()

    def save_zellen(self):
        with open(SAVE_PATH, "w", encoding="utf-8") as f:
            json.dump([z.to_dict() for z in self.zellen], f, indent=2)

    def load_zellen(self):
        if os.path.exists(SAVE_PATH):
            with open(SAVE_PATH, "r", encoding="utf-8") as f:
                try:
                    daten = json.load(f)
                    self.zellen = [GabrielZelle.from_dict(d) for d in daten]
                except:
                    self.zellen = []

    def get_clusters(self):
        return [{
            "ID": c.id,
            "Typ": c.typ,
            "Mitglieder": len(c.mitglieder),
            "Durchschnitt": round(c.durchschnitt_resonanz, 2) if c.durchschnitt_resonanz else "–",
            "Iterationen": c.iterationen
        } for c in self.clusters]

    def get_resonanzpool(self):
        pfad = os.path.join("data", "memory_graph.json")
        if not os.path.exists(pfad):
            return []
        with open(pfad, "r", encoding="utf-8") as f:
            try:
                daten = json.load(f)
                seeds = [d["seed"] for d in daten if d.get("score", 0) > 0.9]
                wörter = []
                for s in seeds:
                    wörter += s.split("-")
                return list(set(wörter))
            except:
                return []
