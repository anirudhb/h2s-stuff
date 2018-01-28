# Simple button class that handles clicks (if propagated)
# Draws rect, calculates font character height and width
class Button:
    padding = 10 # pixels
    def __init__(self, txt, fontSize=12, a=1):
        self.text = txt
        self.fontSize = fontSize
        self.a = a
        self.setupFont()
        self.calcSize()
    def setupXY(self, x, y):
        self.x = x
        self.y = y
    def setupFont(self):
        # Setup fonts
        # Use Arial
        font = createFont("Arial", self.fontSize)
        textFont(font)
        textAlign(LEFT, TOP)
    def calcSize(self):
        self.width = 4*self.padding + textWidth(self.text)
        self.height = 4*self.padding + self.fontSize + textAscent() + textDescent()
    def draw(self):
        fill(color(255,255,255,self.a*255))
        rect(self.x, self.y, self.width, self.height)
        self.setupFont()
        fill(0)
        text(self.text, self.x + 2*self.padding, self.y + 2*self.padding)
    def clicked(self, mx, my):
        return (mx >= self.x and mx <= self.x + self.width \
            and my >= self.y and my <= self.y + self.height)