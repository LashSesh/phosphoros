from PyQt6.QtWidgets import QGraphicsScene, QGraphicsView, QGraphicsEllipseItem
from PyQt6.QtWidgets import QApplication, QWidget, QVBoxLayout
from PyQt6.QtGui import QColor, QBrush
from PyQt6.QtCore import Qt, QTimer
import random
import sys

class MycelAquarium(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("OKEANOS – MycelAquarium")
        self.setGeometry(100, 100, 800, 600)

        self.scene = QGraphicsScene()
        self.view = QGraphicsView(self.scene)
        self.view.setRenderHint(self.view.renderHints())

        layout = QVBoxLayout()
        layout.addWidget(self.view)
        self.setLayout(layout)

        self.zellen = []
        self.create_zellen(20)

        self.timer = QTimer()
        self.timer.timeout.connect(self.animate)
        self.timer.start(100)

    def create_zellen(self, count):
        for _ in range(count):
            x = random.randint(0, 700)
            y = random.randint(0, 500)
            zelle = QGraphicsEllipseItem(x, y, 20, 20)
            zelle.setBrush(QBrush(QColor(random.randint(100,255), 100, 255)))
            zelle.setOpacity(0.8)
            self.scene.addItem(zelle)
            self.zellen.append(zelle)

    def animate(self):
        for zelle in self.zellen:
            dx = random.choice([-1, 0, 1])
            dy = random.choice([-1, 0, 1])
            zelle.moveBy(dx, dy)

if __name__ == "__main__":
    app = QApplication(sys.argv)
    aquarium = MycelAquarium()
    aquarium.show()
    sys.exit(app.exec())
