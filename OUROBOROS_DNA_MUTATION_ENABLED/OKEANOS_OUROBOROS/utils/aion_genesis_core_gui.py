
import sys
import os
import datetime
import numpy as np
import json
from config import GPT_MODE, OPENAI_API_KEY, LOCAL_API_URL

from PyQt6.QtWidgets import (
    QApplication, QWidget, QVBoxLayout, QHBoxLayout,
    QPushButton, QLabel, QTextEdit, QLineEdit, QSlider, QFileDialog
)
from PyQt6.QtCore import Qt, QTimer
from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg as FigureCanvas
from matplotlib.figure import Figure

from gabriel_cell import GabrielCell
import openai
import requests

if GPT_MODE == "openai":
    openai.api_key = OPENAI_API_KEY

OUTPUT_DIR = "genesis_output"
AGENT_DIR = "genesis_agents"
os.makedirs(OUTPUT_DIR, exist_ok=True)
os.makedirs(AGENT_DIR, exist_ok=True)

class GenesisCore(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("AION:Genesis Core v5 – Thron & Gedächtnis")
        self.setGeometry(100, 100, 1000, 800)

        self.cell = GabrielCell(node_count=10)
        self.init_ui()

        self.timer = QTimer()
        self.timer.timeout.connect(self.update_cell_plot)
        self.last_output_path = None

        # Thronachsen
        self.thronachsen = {
            "Intuition": 50,
            "Struktur": 50,
            "Dynamik": 50,
            "Stabilität": 50,
            "Fokus": 50,
            "Streuung": 50
        }

    def init_ui(self):
        layout = QVBoxLayout()
        self.setLayout(layout)

        self.input_line = QLineEdit()
        self.input_line.setPlaceholderText("Impulse eingeben...")
        self.input_line.returnPressed.connect(self.handle_input)
        layout.addWidget(self.input_line)

        self.output_area = QTextEdit()
        self.output_area.setReadOnly(True)
        layout.addWidget(self.output_area)

        # Zellaktivierung
        self.figure = Figure(figsize=(5, 1))
        self.canvas = FigureCanvas(self.figure)
        self.ax = self.figure.add_subplot(111)
        layout.addWidget(self.canvas)

        # Zellsteuerung
        button_layout = QHBoxLayout()
        self.btn_start = QPushButton("Zell-Aktivierung starten")
        self.btn_start.clicked.connect(self.start_cells)
        button_layout.addWidget(self.btn_start)

        self.btn_stop = QPushButton("Zell-Aktivierung stoppen")
        self.btn_stop.clicked.connect(self.stop_cells)
        button_layout.addWidget(self.btn_stop)

        self.btn_save = QPushButton("Zustand speichern")
        self.btn_save.clicked.connect(self.save_cell_state)
        button_layout.addWidget(self.btn_save)

        self.btn_load = QPushButton("Zustand laden")
        self.btn_load.clicked.connect(self.load_cell_state)
        button_layout.addWidget(self.btn_load)

        layout.addLayout(button_layout)

        # Thronachsen
        for name in self.thronachsen:
            slider = QSlider(Qt.Orientation.Horizontal)
            slider.setMinimum(0)
            slider.setMaximum(100)
            slider.setValue(50)
            slider.valueChanged.connect(lambda val, n=name: self.update_thronachse(n, val))
            layout.addWidget(QLabel(name))
            layout.addWidget(slider)

    def update_thronachse(self, name, value):
        self.thronachsen[name] = value
        self.output_area.append(f"[Thronachse {name} → {value}]")

    def start_cells(self):
        self.timer.start(100)

    def stop_cells(self):
        self.timer.stop()

    def update_cell_plot(self):
        input_vec = np.random.rand(self.cell.node_count)
        mod_factor = 1 + ((self.thronachsen["Dynamik"] - 50) / 50.0)
        input_vec *= mod_factor
        self.cell.propagate_signal(input_vec)
        self.cell.hebbian_update()
        self.ax.clear()
        self.ax.imshow(self.cell.activations.reshape(1, -1), cmap="inferno", aspect="auto")
        self.ax.set_xticks(range(self.cell.node_count))
        self.ax.set_yticks([])
        self.ax.set_title("GabrielCell Aktivierung")
        self.canvas.draw()

    def handle_input(self):
        prompt = self.input_line.text().strip()
        self.input_line.clear()
        if not prompt:
            return
        self.output_area.append(f"Du >> {prompt}")
        response = self.query_gpt(prompt)
        self.output_area.append(f"AION >> {response}\n")
        self.save_output(response)
        self.try_generate_agent(response)

    def query_gpt(self, prompt):
        instruction = (
            "Du bist AION:Genesis – ein resonanter Entwicklungsagent. "
            f"Aktive Thronachsen: {json.dumps(self.thronachsen)}. "
            "Antworte funktional, evolutionär, generativ. Erzeuge Code, wenn sinnvoll."
        )
        if GPT_MODE == "openai":
            try:
                response = openai.ChatCompletion.create(
                    model="gpt-4",
                    messages=[
                        {"role": "system", "content": instruction},
                        {"role": "user", "content": prompt}
                    ]
                )
                return response.choices[0].message["content"]
            except Exception as e:
                return f"Fehler bei OpenAI: {e}"
        elif GPT_MODE == "local":
            try:
                headers = {"Content-Type": "application/json"}
                data = {
                    "model": "local-model",
                    "messages": [
                        {"role": "system", "content": instruction},
                        {"role": "user", "content": prompt}
                    ]
                }
                response = requests.post(LOCAL_API_URL, headers=headers, data=json.dumps(data))
                result = response.json()
                return result["choices"][0]["message"]["content"]
            except Exception as e:
                return f"Fehler bei lokalem Modell: {e}"
        return "Ungültiger GPT-Modus."

    def save_output(self, content):
        timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
        filename = f"genesis_{timestamp}.txt"
        path = os.path.join(OUTPUT_DIR, filename)
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)
        self.last_output_path = path

    def try_generate_agent(self, content):
        if "```python" in content:
            try:
                code = content.split("```python")[1].split("```")[0].strip()
                timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
                filename = f"agent_{timestamp}.py"
                path = os.path.join(AGENT_DIR, filename)
                with open(path, "w", encoding="utf-8") as f:
                    f.write(code)
                self.output_area.append(f"[Auto-Agent erzeugt: {filename}]")
            except Exception as e:
                self.output_area.append(f"[Agentenerzeugung fehlgeschlagen: {e}]")

    def save_cell_state(self):
        filename, _ = QFileDialog.getSaveFileName(self, "Zellzustand speichern", ".", "JSON Files (*.json)")
        if filename:
            self.cell.save_state(filename)
            self.output_area.append(f"[Zellzustand gespeichert >> {filename}]")

    def load_cell_state(self):
        filename, _ = QFileDialog.getOpenFileName(self, "Zellzustand laden", ".", "JSON Files (*.json)")
        if filename:
            self.cell.load_state(filename)
            self.output_area.append(f"[Zellzustand geladen << {filename}]")

def main():
    app = QApplication(sys.argv)
    window = GenesisCore()
    window.show()
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
