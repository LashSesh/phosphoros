
from utils import swarm_cell_base

class SwarmController:
    def __init__(self):
        self.cells = {
            "Navigator": [],
            "Mutator": [],
            "Watcher": [],
            "Collector": [],
            "Supervisor": []
        }

    def create_cells(self, cell_type, count):
        cell_class = getattr(swarm_cell_base, f"{cell_type}Cell", None)
        if not cell_class:
            return 0
        for i in range(count):
            cell_id = f"{cell_type[:3]}_{len(self.cells[cell_type]) + 1}"
            self.cells[cell_type].append(cell_class(cell_id))
        return len(self.cells[cell_type])

    def clear_all(self):
        for key in self.cells:
            self.cells[key] = []

    def get_summary(self):
        return {key: len(self.cells[key]) for key in self.cells}


    def delete_cell(self, cell_type, index):
        if cell_type in self.cells and 0 <= index < len(self.cells[cell_type]):
            del self.cells[cell_type][index]
            return True
        return False
    