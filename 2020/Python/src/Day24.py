import InputReader


class Hexagon:
    coordLookup = {}

    def __init__(self, x, y):
        """Initializes a new Hexagon at the given position.
        
        Also looks up the neighboring Hexagons.
        
        Parameters
        ----------
        x: int
            The x coordinate of the new Hexagon.
        y: int
            The y coordinate of the new Hexagon.
        """
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
        """Toggles whether the Hexagon is black.
        
        Also creates the neighboring Hexagons if they don't already exist.
        
        Returns
        -------
        bool
            True if the Hexagon is black.
        """
        self.getOrCreateEast()
        self.getOrCreateSouthEast()
        self.getOrCreateSouthWest()
        self.getOrCreateWest()
        self.getOrCreateNorthWest()
        self.getOrCreateNorthEast()
        self.black = not self.black
        return self.black

    def hasEast(self):
        """Checks whether this Hexagon has a neighbor to the east.
        
        Returns
        -------
        bool
            True if it has.
        """
        return self.east != None

    def getEast(self):
        """Gets the Hexagon east of this one.
        
        Returns
        -------
        Hexagon
            The Hexagon east of this one.
        """
        return self.east

    def getOrCreateEast(self):
        """Gets the Hexagon east of this one, or creates it if it is missing.
        
        Returns
        -------
        Hexagon
            The Hexagon east of this one.
        """
        if not self.hasEast():
            self.setEast(Hexagon(self.x + 1, self.y))

        return self.getEast()

    def setEast(self, east):
        """Sets the Hexagon east of this one.
        
        Also tells that Hexagon that this is the one west of it.
        
        Parameters
        ----------
        east: Hexagon
            The Hexagon east of this.
        """
        if east == None:
            return

        self.east = east

        if not east.hasWest():
            east.setWest(self)

    def hasSouthEast(self):
        """Checks whether this Hexagon has a neighbor to the south east.
        
        Returns
        -------
        bool
            True if it has.
        """
        return self.southEast != None

    def getSouthEast(self):
        """Gets the Hexagon south east of this one.
        
        Returns
        -------
        Hexagon
            The Hexagon south east of this one.
        """
        return self.southEast

    def getOrCreateSouthEast(self):
        """Gets the Hexagon south east of this one, or creates it if it is missing.
        
        Returns
        -------
        Hexagon
            The Hexagon south east of this one.
        """
        if not self.hasSouthEast():
            self.setSouthEast(Hexagon(self.x + 0.5, self.y + 1))

        return self.getSouthEast()

    def setSouthEast(self, southEast):
        """Sets the Hexagon south east of this one.
        
        Also tells that Hexagon that this is the one north west of it.
        
        Parameters
        ----------
        southEast: Hexagon
            The Hexagon south east of this.
        """
        if southEast == None:
            return

        self.southEast = southEast

        if not southEast.hasNorthWest():
            southEast.setNorthWest(self)

    def hasSouthWest(self):
        """Checks whether this Hexagon has a neighbor to the south west.
        
        Returns
        -------
        bool
            True if it has.
        """
        return self.southWest != None

    def getSouthWest(self):
        """Gets the Hexagon south west of this one.
        
        Returns
        -------
        Hexagon
            The Hexagon south west of this one.
        """
        return self.southWest

    def getOrCreateSouthWest(self):
        """Gets the Hexagon south west of this one, or creates it if it is missing.
        
        Returns
        -------
        Hexagon
            The Hexagon south west of this one.
        """
        if not self.hasSouthWest():
            self.setSouthWest(Hexagon(self.x - 0.5, self.y + 1))

        return self.getSouthWest()

    def setSouthWest(self, southWest):
        """Sets the Hexagon south west of this one.
        
        Also tells that Hexagon that this is the one north east of it.
        
        Parameters
        ----------
        southWest: Hexagon
            The Hexagon south west of this.
        """
        if southWest == None:
            return

        self.southWest = southWest;

        if not southWest.hasNorthEast():
            southWest.setNorthEast(self)

    def hasWest(self):
        """Checks whether this Hexagon has a neighbor to the west.
        
        Returns
        -------
        bool
            True if it has.
        """
        return self.west != None

    def getWest(self):
        """Gets the Hexagon west of this one.
        
        Returns
        -------
        Hexagon
            The Hexagon west of this one.
        """
        return self.west

    def getOrCreateWest(self):
        """Gets the Hexagon west of this one, or creates it if it is missing.
        
        Returns
        -------
        Hexagon
            The Hexagon west of this one.
        """
        if not self.hasWest():
            self.setWest(Hexagon(self.x - 1, self.y))

        return self.getWest()

    def setWest(self, west):
        """Sets the Hexagon west of this one.
        
        Also tells that Hexagon that this is the one east of it.
        
        Parameters
        ----------
        west: Hexagon
            The Hexagon west of this.
        """
        if west == None:
            return

        self.west = west

        if not west.hasEast():
            west.setEast(self)

    def hasNorthWest(self):
        """Checks whether this Hexagon has a neighbor to the north west.
        
        Returns
        -------
        bool
            True if it has.
        """
        return self.northWest != None

    def getNorthWest(self):
        """Gets the Hexagon north west of this one.
        
        Returns
        -------
        Hexagon
            The Hexagon north west of this one.
        """
        return self.northWest

    def getOrCreateNorthWest(self):
        """Gets the Hexagon north west of this one, or creates it if it is missing.
        
        Returns
        -------
        Hexagon
            The Hexagon north west of this one.
        """
        if not self.hasNorthWest():
            self.setNorthWest(Hexagon(self.x - 0.5, self.y - 1))

        return self.getNorthWest()

    def setNorthWest(self, northWest):
        """Sets the Hexagon north west of this one.
        
        Also tells that Hexagon that this is the one south east of it.
        
        Parameters
        ----------
        northWest: Hexagon
            The Hexagon north west of this.
        """
        if northWest == None:
            return

        self.northWest = northWest

        if not northWest.hasSouthEast():
            northWest.setSouthEast(self)

    def hasNorthEast(self):
        """Checks whether this Hexagon has a neighbor to the northeast.
        
        Returns
        -------
        bool
            True if it has.
        """
        return self.northEast != None

    def getNorthEast(self):
        """Gets the Hexagon north east of this one.
        
        Returns
        -------
        Hexagon
            The Hexagon north east of this one.
        """
        return self.northEast

    def getOrCreateNorthEast(self):
        """Gets the Hexagon north east of this one, or creates it if it is missing.
        
        Returns
        -------
        Hexagon
            The Hexagon north east of this one.
        """
        if not self.hasNorthEast():
            self.setNorthEast(Hexagon(self.x + 0.5, self.y - 1))

        return self.getNorthEast()

    def setNorthEast(self, northEast):
        """Sets the Hexagon north east of this one.
        
        Also tells that Hexagon that this is the one south west of it.
        
        Parameters
        ----------
        northEast: Hexagon
            The Hexagon north east of this.
        """
        if northEast == None:
            return

        self.northEast = northEast

        if not northEast.hasSouthWest():
            northEast.setSouthWest(self)

    def check(self):
        """Checks whether this Hexagon should toggle being black.
        
        If a Hexagon is black it becomes white if either none or more then
        2 of its neighbors are also black.
        If a Hexagon is white it becomes black if 2 of its neighbors are
        black.
        
        Returns
        -------
        bool
            Whether this Hexagon should change its color.
        """
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
        """Makes all Hexagons check if they should change their color, and if yes do so.
        
        Returns
        -------
        int
            The number of black Hexagons after the change.
        """
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
