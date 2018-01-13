WIDTH = 1000
HEIGHT = 1000
rectSize = 20

### BEGIN CLASSES ###
class Color:
    
    def __init__(self, r, g, b):
        self.r = r
        self.g = g
        self.b = b
    
    def toColor(self):
        return color(self.r, self.g, self.b)

Color.default = Color(0,0,0)

class Grid:
    def __init__(self, width, height, _rect, _fill, sqrSize):
        self.grid = []
        self.width = width
        self.height = height
        for i in range(height):
            row = []
            for j in range(width):
                row.append(Color.default)
            self.grid.append(row)
        self._setupDraw(_rect, _fill, sqrSize)
    
    def _setupDraw(self, _rect, _fill, _sqrSize):
        self._rect = _rect
        self._fill = _fill
        self._sqrSize = _sqrSize
    
    def _reset_background(self):
        self._rect(0, 0, width*self._sqrSize, height*self._sqrSize)
    
    def draw(self):
        self._reset_background()
        self._draw()
    
    def _draw(self):
        for (rowN, row) in enumerate(self.grid):
            for (itemN, item) in enumerate(row):
                self._fill(item.toColor())
                self._rect(rowN*self._sqrSize, itemN*self._sqrSize, self._sqrSize, self._sqrSize)
    
    def coordFromMouse(self, mx, my):
        return (int(mx/self._sqrSize), int(my/self._sqrSize))
    
    def resetColors(self):
        self.grid = []
        for i in range(self.height):
            row = []
            for j in range(self.width):
                row.append(Color.default)
            self.grid.append(row)
    
    def __getitem__(self, x):
        return self.grid[x]
    
    def __setitem__(self, x, y):
        self.grid[x] = y


grid = Grid(WIDTH/rectSize, HEIGHT/rectSize, rect, fill, rectSize)


def setup():
    size(WIDTH, HEIGHT)
    background(0)
    smooth(8)

def draw():
    global lastx, lasty, eraseLast
    background(255)
    fill(color(0,255,0))
    grid.draw()

def mouseDragged():
    eraseLast = False
    x, y = grid.coordFromMouse(mouseX, mouseY)
    if mouseButton == LEFT:
        grid[x][y] = Color(0,255,0)
    elif mouseButton == RIGHT:
        grid[x][y] = Color(0,0,0)