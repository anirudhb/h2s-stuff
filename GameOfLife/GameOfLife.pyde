WIDTH = 1024
HEIGHT = 1024
rectSize = 16

from Grid import Grid
from Color import Color
from Button import Button
from Simulation import SimGrid
from sys import stdout
from copy import deepcopy
printable = '\'0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!"#$%&\\\'()*+,-./:;<=>?@[\\\\]^_`{|}~ \t\n\r\x0b\x0c\''

### BEGIN CLASSES ###


grid = Grid(WIDTH/rectSize, HEIGHT/rectSize, rect, fill, rectSize)

COPY_SIZE = 100

def setup():
    global runButton, playButton, grid
    size(WIDTH, HEIGHT, P2D)
    background(0)
    smooth(8)
    runButton = Button("Generation", 52)
    runButton.setupXY(0, 0)
    playButton = Button("Serialize", 52)
    playButton.setupXY(300, 0)
    
    selectInput("Select file to open:", "openFileSelected")

def openFileSelected(f):
    global grid
    with open(f.getAbsolutePath(), "r") as f:
        grid.apply_serialized(f.read())

runButton = None
playButton = None

def draw():
    global lastx, lasty, eraseLast
    background(255)
    fill(color(0,255,0))
    if grid is not None:
        grid.draw()
    # Draw overlaid buttons
    overlay()
    overlayMouse()

# If true, makes the buttons semi-transparent so you can see the cells underneath.
beAlpha = False

def mouseDragged():
    global beAlpha
    x, y = grid.coordFromMouse(mouseX, mouseY)
    if x > grid.width-1 or x < 0 or y > grid.height-1 or y < 0:
        pass
    else:
        if mouseButton == LEFT:
            grid[x][y] = Color.cell
        elif mouseButton == RIGHT:
            grid[x][y] = Color.default
    beAlpha = True

def mouseReleased():
    global beAlpha
    beAlpha = False

serialized = ""

def mousePressed():
    if runButton.clicked(mouseX, mouseY):
        s = grid.to_simgrid()
        s.sim_gen()
        grid.from_simgrid(s)
    if playButton.clicked(mouseX, mouseY):
        global serialized
        serialized = grid.serialize()
        selectOutput("Select a file to save your grid to:", "gotFile")

def gotFile(f):
    with open(f.getAbsolutePath(), "w") as ff:
        ff.write(serialized)
    print("Written serialized grid!")

def overlay():
    global beAlpha
    # Run button
    if beAlpha:
        runButton.a = .5
        playButton.a = .5
    else:
        runButton.a = 1
        playButton.a = 1
    runButton.draw()
    playButton.draw()

def overlayMouse():
    x, y = grid.coordFromMouse(mouseX, mouseY)
    x = rectSize*x
    y = rectSize*y
    fill(color(Color.cell.r, Color.cell.g, Color.cell.b, .5*255))
    rect(x, y, rectSize, rectSize)