from PyQt6.QtWidgets import (
    QApplication, QMainWindow, QTabWidget, QWidget, QLabel, QVBoxLayout,
    QPushButton, QTextEdit, QLineEdit, QFileDialog
)
from PyQt6.QtOpenGLWidgets import QOpenGLWidget
from mycel_aquarium_3d import MycelAquarium3D
import sys
import matplotlib.pyplot as plt
from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg as FigureCanvas
import numpy as np

class ZellfarmTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        label = QLabel("Zellfarm-Steuerung hier integrieren.")
        label.setStyleSheet("font-size: 16px; color: #CCCCCC;")
        layout.addWidget(label)
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

class μcotopTab(MycelAquarium3D):
    def __init__(self):
        super().__init__()

class RIDTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        self.status = QLabel("Michael (K3-Agent) Status: INAKTIV")
        self.status.setStyleSheet("color: #00FF88; font-weight: bold; font-size: 16px;")
        self.prompt = QLineEdit()
        self.prompt.setPlaceholderText("Optionaler Prompt für Michael...")
        self.output = QTextEdit()
        self.output.setReadOnly(True)
        self.output.setStyleSheet("background-color: #222222; color: #EEEEEE;")
        self.toggle = QPushButton("K3-Agent starten")
        self.toggle.clicked.connect(self.toggle_agent)
        layout.addWidget(self.status)
        layout.addWidget(self.prompt)
        layout.addWidget(self.toggle)
        layout.addWidget(self.output)
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

    def toggle_agent(self):
        self.status.setText("Michael (K3-Agent) Status: AKTIV")
        self.output.append("K3-Agent aktiviert... (Simulation)")

class ReflexTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        label = QLabel("Reflexmatrix: Triggerstatus")
        label.setStyleSheet("font-size: 16px; color: #CCCCCC;")
        self.log = QTextEdit()
        self.log.setReadOnly(True)
        self.log.append("System bereit...")
        layout.addWidget(label)
        layout.addWidget(self.log)
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

class ScorpioGraphTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        self.figure, self.ax = plt.subplots()
        self.canvas = FigureCanvas(self.figure)
        layout.addWidget(self.canvas)
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)
        self.plot_data()

    def plot_data(self):
        t = np.linspace(0, 2 * np.pi, 500)
        for i in range(6):
            offset = i * 0.25
            signal = np.sin(t + offset)
            style = '--' if i in [1, 4] else ':' if i == 3 else '-'
            self.ax.plot(t, signal, style, label=f"Zelle {i+1}")
        self.ax.set_title("ScorpioGraph: Zellpulsnetz")
        self.ax.legend()
        self.canvas.draw()

class ScorpioVisionTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        label = QLabel("ScorpioVision: Echtzeit-Puffer")
        label.setStyleSheet("font-size: 16px; color: #CCCCCC;")
        buffer_log = QTextEdit()
        buffer_log.setReadOnly(True)
        buffer_log.append("Cluster-Puffer online.
Verknüpfungen aktiv...")
        layout.addWidget(label)
        layout.addWidget(buffer_log)
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

class SettingsTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        layout.addWidget(QLabel("Design & Theme Auswahl (Platzhalter)"))
        layout.addWidget(QLabel("Chain-Auswahl & Systemoptionen"))
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

class StructureTab(QWidget):
    def __init__(self):
        super().__init__()
        layout = QVBoxLayout()
        label = QLabel("Strukturwissen: 5D-Wissenseinbettung")
        label.setStyleSheet("font-size: 16px; color: #CCCCCC;")
        text = QTextEdit()
        text.setReadOnly(True)
        text.setText("Hier erscheint die Strukturtheorie aus deinen 5D-Arbeiten...")
        layout.addWidget(label)
        layout.addWidget(text)
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

from modules.spectral_fabric import SpectralFabric
from modules.k3_agent_core import K3ResonanceAnalyzer
from modules.cotop_render import visualize_fabric_links
from modules.spectral_chainwalker import SpectralChainWalker
from modules.pulse_tracker import PulsTracker

class SpectrographTab(QWidget):
    def __init__(self):
        super().__init__()
        self.fabric = SpectralFabric()

        layout = QVBoxLayout()
        self.label = QLabel("HolyG – 5D Spektograph")
        self.label.setStyleSheet("font-size: 16px; color: #FFCC66;")
        layout.addWidget(self.label)

        self.seed_input = QLineEdit()
        self.seed_input.setPlaceholderText("Seed eingeben...")
        layout.addWidget(self.seed_input)

        self.add_button = QPushButton("Seed hinzufügen")
        self.add_button.clicked.connect(self.add_seed)
        layout.addWidget(self.add_button)

        self.fabric_output = QTextEdit()
        self.fabric_output.setReadOnly(True)
        layout.addWidget(self.fabric_output)

        self.build_btn = QPushButton("Fabric aufbauen")
        self.build_btn.clicked.connect(self.show_fabric)
        layout.addWidget(self.build_btn)

        self.project_btn = QPushButton("Projektions-Hinweise erzeugen")
        self.project_btn.clicked.connect(self.show_hints)
        layout.addWidget(self.project_btn)
        self.k3_btn = QPushButton("K3 Musteranalyse")
        self.k3_btn.clicked.connect(self.k3_analyse)
        layout.addWidget(self.k3_btn)
        self.cotop_btn = QPushButton("μcotop: Fadenstruktur zeigen")
        self.cotop_btn.clicked.connect(self.show_cotop)
        layout.addWidget(self.cotop_btn)
        self.fragment_input = QLineEdit()
        self.fragment_input.setPlaceholderText("Fragment oder Teilseed...")
        layout.addWidget(self.fragment_input)

        self.chainwalker_btn = QPushButton("Chainwalker starten")
        self.chainwalker_btn.clicked.connect(self.run_chainwalker)
        layout.addWidget(self.chainwalker_btn)
        self.pulse_btn = QPushButton("Pulsstatus anzeigen")
        self.pulse_btn.clicked.connect(self.show_pulse)
        layout.addWidget(self.pulse_btn)
    
    
    
    
        self.inject_btn = QPushButton("Seeds an Zellnetzwerk zurückgeben")
        self.inject_btn.clicked.connect(self.inject_seeds)
        layout.addWidget(self.inject_btn)


        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

    def add_seed(self):
        seed = self.seed_input.text()
        if seed:
            self.fabric.add_seed(seed)
            self.fabric_output.append(f"+ Seed hinzugefügt.")

    def show_fabric(self):
        links = self.fabric.build_fabric()
        self.fabric_output.append("\n--- Verbindungen ---")
        for s1, s2, dist in links:
            self.fabric_output.append(f"{s1[:6]} ↔ {s2[:6]} (Δ={dist})")

    def show_hints(self):
        hints = self.fabric.get_projection_hint()
        self.fabric_output.append("\n--- Projektionen ---")
        for h in hints:
            self.fabric_output.append(h)

        label.setStyleSheet("font-size: 16px; color: #FFCC66;")
        output = QTextEdit()
        output.setReadOnly(True)
        output.append("Spektrale Analyse der Informationsgeometrie vorbereitet...")
        layout.addWidget(label)
        layout.addWidget(output)
        self.k3 = K3ResonanceAnalyzer()
        self.chainwalker = SpectralChainWalker()
        self.puls = PulsTracker()
        self.setLayout(layout)

class OkeanosMain(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("OKEANOS PyQt6 Interface – COMPLETE")
        self.setMinimumSize(1024, 768)
        self.setStyleSheet("background-color: #1e1e1e;")
        self.tabs = QTabWidget()
        self.tabs.setTabPosition(QTabWidget.TabPosition.North)
        self.tabs.setStyleSheet("QTabBar::tab { height: 30px; width: 180px; color: white; }")

        self.tabs.addTab(ZellfarmTab(), "Zellfarm")
        self.tabs.addTab(RIDTab(), "RID / Michael")
        self.tabs.addTab(ReflexTab(), "Reflexmatrix")
        self.tabs.addTab(ScorpioGraphTab(), "ScorpioGraph")
        self.tabs.addTab(ScorpioVisionTab(), "ScorpioVision")
        self.tabs.addTab(μcotopTab(), "μcotop")
        self.tabs.addTab(SettingsTab(), "Einstellungen")
        self.tabs.addTab(StructureTab(), "Strukturwissen")
        self.tabs.addTab(SpectrographTab(), "5D Spektograph")

        self.setCentralWidget(self.tabs)

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = OkeanosMain()
    window.show()
    sys.exit(app.exec())


    def inject_seeds(self):
        seeds = self.fabric.export_projected_seeds()
        self.fabric_output.append("\n--- Rückführung ---")
        for s in seeds:
            self.fabric_output.append(f"→ {s} an Zellnetzwerk übergeben (simuliert)")


    def k3_analyse(self):
        links = self.fabric.build_fabric()
        self.k3.observe(links)
        report = self.k3.report()
        self.fabric_output.append("\n--- K3 Musteranalyse ---")
        for line in report:
            self.fabric_output.append(line)
    

    def show_cotop(self):
        links = self.fabric.build_fabric()
        if links:
            visualize_fabric_links(links)
        else:
            self.fabric_output.append("Keine Fäden zur Visualisierung.")
    

    def run_chainwalker(self):
        frag = self.fragment_input.text()
        if not frag or len(frag) < 8:
            self.fabric_output.append("Fragment zu kurz für Analyse.")
            return
        seed_data = [s for s, _ in self.fabric.seed_vectors]
        self.chainwalker.load_reference_seeds(seed_data)
        results = self.chainwalker.extrapolate_from_fragment(frag)
        self.fabric_output.append("\n--- Chainwalker Projektion ---")
        for r in results:
            self.fabric_output.append(f"> {r}")
    

    def show_pulse(self):
        self.fabric_output.append("\n--- Zellpuls ---")
        self.fabric_output.append(self.puls.get_pulse_summary())
        anom = self.puls.detect_frequency_anomalies()
        if anom:
            self.fabric_output.append("\n--- Warnungen ---")
            for a in anom:
                self.fabric_output.append(a)
    