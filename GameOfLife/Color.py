class Color:
    
    def __init__(self, r, g, b):
        self.r = r
        self.g = g
        self.b = b
    
    def toColor(self):
        return color(self.r, self.g, self.b)

Color.default = Color(0,0,0)
Color.cell = Color(0,255,0)