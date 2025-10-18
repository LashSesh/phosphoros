
import time
from threading import Thread
from streamlit.runtime.scriptrunner import add_script_run_ctx
from modules.cell_manager import CellManager

class OuroborosEngine:
    def __init__(self, manager: CellManager):
        self.manager = manager
        self.takt_phase = 0
        self.running = False
        self.thread = None

    def start(self):
        if not self.running:
            self.running = True
            self.thread = Thread(target=self._run_loop)
            add_script_run_ctx(self.thread)
            self.thread.start()

    def stop(self):
        self.running = False

    def _run_loop(self):
        while self.running:
            self.takt_phase += 1
            if self.takt_phase % 3 == 0:
                self._tick_supervisor()
            elif self.takt_phase % 2 == 0:
                self._tick_helix_b()
            else:
                self._tick_helix_a()
            self.manager.save_zellen()
            time.sleep(5)

    def _tick_helix_a(self):
        for z in self.manager.zellen:
            if z.typ in ["Navigator", "Collector"]:
                z.tick()

    def _tick_helix_b(self):
        for z in self.manager.zellen:
            if z.typ in ["Mutator", "Watcher"]:
                z.tick()

    def _tick_supervisor(self):
        for z in self.manager.zellen:
            if z.typ == "Supervisor":
                z.tick()
