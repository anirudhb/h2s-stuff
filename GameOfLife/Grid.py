from Color import Color
from Simulation import SimGrid

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
    
    def serialize(self):
        # Serialize efficiently; use 3 characters per cell
        # Metadata: ASCII number seperated by comma (width, height, sqrsize)
        # Metadata on new line
        metadata = ",".join([str(self.width), str(self.height)]) + "\n"
        serialized = ""
        sgrid = self.to_simgrid()
        for row in sgrid.grid:
            for r in row:
                serialized += chr(r)
        return metadata + serialized
    
    def from_simgrid(self, sgrid):
        #print sgrid.grid
        for ix in range(self.height):
            for i in range(self.width):
                if sgrid.grid[ix][i]:
                    self.grid[ix][i] = Color.cell
                else:
                    self.grid[ix][i] = Color.default
    
    def apply_serialized(self, s):
        metadata, ser = s.split("\n")
        w, h = map(int, metadata.split(","))
        for ix in range(h):
            for i in range(w):
                index = h*ix+i
                if ord(ser[index]) == 1:
                    self[ix][i] = Color.cell
                else:
                    self[ix][i] = Color.default
    
    def to_simgrid(self):
        l = []
        for r in self:
            row = []
            for c in r:
                if c != Color.cell:
                    row.append(0)
                else:
                    row.append(1)
            l.append(row)
        return SimGrid(l)
    
    def __getitem__(self, x):
        return self.grid[x]
    
    def __setitem__(self, x, y):
        self.grid[x] = y