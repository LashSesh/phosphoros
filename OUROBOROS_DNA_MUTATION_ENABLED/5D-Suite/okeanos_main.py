from PyQt6.QtWidgets import QApplication, QMainWindow, QTabWidget, QWidget, QLabel, QVBoxLayout
import sys

class OkeanosTab(QWidget):
    def __init__(self, title):
        super().__init__()
        layout = QVBoxLayout()
        label = QLabel(f"Modul: {title}")
        label.setStyleSheet("font-size: 18px; color: #CCCCCC;")
        layout.addWidget(label)
        self.setLayout(layout)

class OkeanosMain(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("OKEANOS PyQt6 Interface")
        self.setMinimumSize(1024, 768)
        self.setStyleSheet("background-color: #1e1e1e;")

        self.tabs = QTabWidget()
        self.tabs.setTabPosition(QTabWidget.TabPosition.North)
        self.tabs.setStyleSheet("QTabBar::tab { height: 30px; width: 180px; color: white; }")

        self.modules = [
            "Zellfarm",
            "RID / Michael",
            "Reflexmatrix",
            "ScorpioGraph",
            "ScorpioVision",
            "μcotop",
            "Einstellungen",
            "Strukturwissen"
        ]

        for module in self.modules:
            self.tabs.addTab(OkeanosTab(module), module)

        self.setCentralWidget(self.tabs)

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = OkeanosMain()
    window.show()
    sys.exit(app.exec())
