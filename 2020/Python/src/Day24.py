import InputReader


class Hexagon:
    coordLookup = {}

    def __init__(self, x, y):
        self.black = False
        self.x = x
        self.y = y
        self.coordLookup[(x, y)] = self;
        self.east = self.southEast = self.southWest = self.west = self.northWest = self.northEast = None
        self.setEast(self.coordLookup.get((x + 1, y), None))
        self.setSouthEast(self.coordLookup.get((x + 0.5, y + 1), None))
        self.setSouthWest(self.coordLookup.get((x - 0.5, y + 1), None))
        self.setWest(self.coordLookup.get((x - 1, y), None))
        self.setNorthWest(self.coordLookup.get((x - 0.5, y - 1), None))
        self.setNorthEast(self.coordLookup.get((x + 0.5, y - 1), None))

    def toggle(self):
        self.getOrCreateEast()
        self.getOrCreateSouthEast()
        self.getOrCreateSouthWest()
        self.getOrCreateWest()
        self.getOrCreateNorthWest()
        self.getOrCreateNorthEast()
        self.black = not self.black
        return self.black

    def hasEast(self):
        return self.east != None

    def getEast(self):
        return self.east

    def getOrCreateEast(self):
        if not self.hasEast():
            self.setEast(Hexagon(self.x + 1, self.y))

        return self.getEast()

    def setEast(self, east):
        if east == None:
            return

        self.east = east

        if not east.hasWest():
            east.setWest(self)

    def hasSouthEast(self):
        return self.southEast != None

    def getSouthEast(self):
        return self.southEast

    def getOrCreateSouthEast(self):
        if not self.hasSouthEast():
            self.setSouthEast(Hexagon(self.x + 0.5, self.y + 1))

        return self.getSouthEast()

    def setSouthEast(self, southEast):
        if southEast == None:
            return

        self.southEast = southEast

        if not southEast.hasNorthWest():
            southEast.setNorthWest(self)

    def hasSouthWest(self):
        return self.southWest != None

    def getSouthWest(self):
        return self.southWest

    def getOrCreateSouthWest(self):
        if not self.hasSouthWest():
            self.setSouthWest(Hexagon(self.x - 0.5, self.y + 1))

        return self.getSouthWest()

    def setSouthWest(self, southWest):
        if southWest == None:
            return

        self.southWest = southWest;

        if not southWest.hasNorthEast():
            southWest.setNorthEast(self)

    def hasWest(self):
        return self.west != None

    def getWest(self):
        return self.west

    def getOrCreateWest(self):
        if not self.hasWest():
            self.setWest(Hexagon(self.x - 1, self.y))

        return self.getWest()

    def setWest(self, west):
        if west == None:
            return

        self.west = west

        if not west.hasEast():
            west.setEast(self)

    def hasNorthWest(self):
        return self.northWest != None

    def getNorthWest(self):
        return self.northWest

    def getOrCreateNorthWest(self):
        if not self.hasNorthWest():
            self.setNorthWest(Hexagon(self.x - 0.5, self.y - 1))

        return self.getNorthWest()

    def setNorthWest(self, northWest):
        if northWest == None:
            return

        self.northWest = northWest

        if not northWest.hasSouthEast():
            northWest.setSouthEast(self)

    def hasNorthEast(self):
        return self.northEast != None

    def getNorthEast(self):
        return self.northEast

    def getOrCreateNorthEast(self):
        if not self.hasNorthEast():
            self.setNorthEast(Hexagon(self.x + 0.5, self.y - 1))

        return self.getNorthEast()

    def setNorthEast(self, northEast):
        if northEast == None:
            return

        self.northEast = northEast

        if not northEast.hasSouthWest():
            northEast.setSouthWest(self)

    def check(self):
        blackNeightbors = 0
        if self.hasEast():
            if self.east.black:
                blackNeightbors += 1
        if self.hasSouthEast():
            if self.southEast.black:
                blackNeightbors += 1
        if self.hasSouthWest():
            if self.southWest.black:
                blackNeightbors += 1
        if self.hasWest():
            if self.west.black:
                blackNeightbors += 1
        if self.hasNorthWest():
            if self.northWest.black:
                blackNeightbors += 1
        if self.hasNorthEast():
            if self.northEast.black:
                blackNeightbors += 1

        if self.black:
            return blackNeightbors == 0 or blackNeightbors > 2
        else:
            return blackNeightbors == 2

    @staticmethod
    def updateAll():
        marked = []
        for hex in Hexagon.coordLookup.values():
            if hex.check():
                marked.append(hex)

        for hex in marked:
            hex.toggle()

        black = 0
        for hex in Hexagon.coordLookup.values():
            if hex.black:
                black += 1

        return black


def main():
    center = Hexagon(0, 0)
    black = 0
    for line in InputReader.readInputFileLines(24):
        current = center
        i = 0
        while i < len(line):
            c = line[i]
            if c == 'e':
                current = current.getOrCreateEast()
            elif c == 's':
                i += 1
                if line[i] == 'e':
                    current = current.getOrCreateSouthEast()
                else:
                    current = current.getOrCreateSouthWest()
            elif c == 'w':
                current = current.getOrCreateWest()
            elif c == 'n':
                i += 1
                if line[i] == 'w':
                    current = current.getOrCreateNorthWest()
                else:
                    current = current.getOrCreateNorthEast()

            i += 1
        if current.toggle():
            black += 1
        else:
            black -= 1

    print(f"After part 1 there are {black} black tiles.")

    for _ in range(100):
        black2 = Hexagon.updateAll()

    print(f"After part 2 there are {black2} black tiles.")


if __name__ == '__main__':
    main()
