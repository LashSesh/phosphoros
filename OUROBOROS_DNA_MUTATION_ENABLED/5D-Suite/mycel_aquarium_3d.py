from PyQt6.QtWidgets import QApplication, QMainWindow
from PyQt6.QtOpenGLWidgets import QOpenGLWidget
from PyQt6.QtCore import Qt, QTimer
from OpenGL.GL import *
from OpenGL.GLU import *
import sys
import random
import math

class MycelAquarium3D(QOpenGLWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setMinimumSize(800, 600)
        self.zellen = self.generate_zellen(30)
        self.angle = 0

        self.timer = QTimer(self)
        self.timer.timeout.connect(self.update_animation)
        self.timer.start(30)

    def generate_zellen(self, count):
        return [
            {
                "x": random.uniform(-5, 5),
                "y": random.uniform(-5, 5),
                "z": random.uniform(-5, 5),
                "r": random.random(),
                "g": random.random(),
                "b": random.random()
            } for _ in range(count)
        ]

    def update_animation(self):
        self.angle += 0.5
        self.update()

    def initializeGL(self):
        glClearColor(0.05, 0.05, 0.1, 1)
        glEnable(GL_DEPTH_TEST)
        glEnable(GL_BLEND)
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)

    def resizeGL(self, w, h):
        glViewport(0, 0, w, h)
        glMatrixMode(GL_PROJECTION)
        glLoadIdentity()
        gluPerspective(45, w / h if h != 0 else 1, 0.1, 50.0)
        glMatrixMode(GL_MODELVIEW)

    def paintGL(self):
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        glLoadIdentity()
        glTranslatef(0.0, 0.0, -20)
        glRotatef(self.angle, 1, 1, 0)

        for z in self.zellen:
            glPushMatrix()
            glTranslatef(z["x"], z["y"], z["z"])
            glColor4f(z["r"], z["g"], z["b"], 0.9)
            quad = gluNewQuadric()
            gluSphere(quad, 0.4, 16, 16)
            glPopMatrix()

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("OKEANOS – MycelAquarium 3D")
        self.setCentralWidget(MycelAquarium3D())

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec())
