class SimGrid:
    def __init__(self, grid):
        self.grid = grid
    def sim_gen(self):
        new_grid = []
        for (rn, row) in enumerate(self.grid):
            r = []
            for (cn, cell) in enumerate(row):
                neighbors = sum(self.neighbors(rn, cn))
                #print(self.neighbors(rn, cn))
                if cell:
                    if neighbors < 2:
                        r.append(0)
                    elif neighbors == 2 or neighbors == 3:
                        r.append(1)
                    elif neighbors > 3:
                        r.append(0)
                elif not cell:
                    if neighbors == 3:
                        r.append(1)
                    else:
                        r.append(0)
                else:
                    r.append(0)
            new_grid.append(r)
        self.grid = new_grid
    
    def neighbors(self, row, col):
        morphedGrid = WrapList()
        for r in self.grid:
            morphedGrid.append(WrapList(r))
            #print("Length of row:",len(WrapList(r)))
        #print("Length of morphed grid:",len(morphedGrid))
        #print("Rows:",row)
        #print("Columns:",col)
        topleft = morphedGrid[row-1][col-1]
        top = morphedGrid[row-1][col]
        topright = morphedGrid[row-1][col+1]
        left = morphedGrid[row][col-1]
        right = morphedGrid[row][col+1]
        bottomleft = morphedGrid[row+1][col-1]
        bottom = morphedGrid[row+1][col]
        bottomright = morphedGrid[row+1][col+1]
        return [topleft, top, topright, left, right, bottomleft, bottom, bottomright]

class WrapList(list):
    def __getitem__(self, index):
        # If the index is negative, return 0
        #print("WrapList array being accessed. Length:",len(self))
        #print("WrapList array access:",index)
        # If the index is greater than the list's length-1, return 0
        if index > len(self)-1:
            return super(WrapList, self).__getitem__(index-len(self))
        return super(WrapList, self).__getitem__(index)