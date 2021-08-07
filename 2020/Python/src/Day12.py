import InputReader


def main():
    north = 0
    west = 0
    waypointNorth = 1;
    waypointWest = -10;
    shipNorth = 0;
    shipWest = 0;
    rotation = 1
    for line in InputReader.readInputFileLines(12):
        value = int(line[1:])
        if line[0] == 'N':
            north += value
            waypointNorth += value
        elif line[0] == 'S':
            north -= value
            waypointNorth -= value
        elif line[0] == 'E':
            west -= value
            waypointWest -= value
        elif line[0] == 'W':
            west += value
            waypointWest += value
        elif line[0] == 'L':
            value /= 90
            rotation = (rotation - value) % 4
            oldWest = waypointWest
            if value == 1:
                waypointWest = waypointNorth
                waypointNorth = -oldWest
            elif value == 2:
                waypointWest *= -1
                waypointNorth *= -1
            elif value == 3:
                waypointWest = -waypointNorth
                waypointNorth = oldWest
        elif line[0] == 'R':
            value /= 90
            rotation = (rotation + value) % 4
            oldWest = waypointWest
            if value == 1:
                waypointWest = -waypointNorth
                waypointNorth = oldWest
            elif value == 2:
                waypointWest *= -1
                waypointNorth *= -1
            elif value == 3:
                waypointWest = waypointNorth
                waypointNorth = -oldWest
        elif line[0] == 'F':
            if rotation == 0:
                north += value
            elif rotation == 1:
                west -= value
            elif rotation == 2:
                north -= value
            elif rotation == 3:
                west += value

            shipNorth += waypointNorth * value
            shipWest += waypointWest * value

    print(f"The Manhattan distance from the ferrys starting position to its part 1 position is {abs(north) + abs(west)}.")

    print(f"The Manhattan distance from the ferrys starting position to its part 2 position is {abs(shipNorth) + abs(shipWest)}.")


if __name__ == '__main__':
    main()
