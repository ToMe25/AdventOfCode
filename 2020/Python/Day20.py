import os
import math
import sys


class Tile:

    def __init__(self, pixels):
        self.pixels = pixels
        self.top = pixels[0]
        self.bottom = pixels[len(pixels) - 1]
        self.left = [row[0] for row in pixels]
        self.right = [row[len(row) - 1] for row in pixels]

    def rotate(self):
        return Tile([[self.pixels[len(self.pixels) - 1 - j][i] for j in range(len(self.pixels))] for i in range(len(self.pixels))])

    def mirror(self):
        return Tile([self.pixels[len(self.pixels) - 1 - i].copy() for i in range(len(self.pixels))])

    def fits(self, t):
        if self.top == t.bottom:
            return 1
        elif self.left == t.right:
            return 2
        elif self.bottom == t.top:
            return 3
        elif self.right == t.left:
            return 4
        else:
            return 0

    def fitsRotated(self, t):
        for i in range(4):
            fit = self.fits(t)
            if fit != 0:
                return fit, i

            t = t.rotate()

        return 0, 0

    def fitsMirrored(self, t):
        fit, rot = self.fitsRotated(t)
        if fit != 0:
            return fit, rot, 0

        fit, rot = self.fitsRotated(t.mirror())
        if fit != 0:
            return fit, rot, 1

        return 0, 0, 0


def countWaves(image, monster):
    waves = 0
    for _ in range(4):
        monsterFound = False
        monsters = []
        for y in range(len(image)):
            for x in range(len(image[y])):
                if y < len(image) - len(monster) and x < len(image[y]) - len(monster[0]):
                    error = False
                    for i in range(len(monster)):
                        for j in range(len(monster[i])):
                            if monster[i][j] and not image[y + i][x + j]:
                                error = True
                                break

                        if error:
                            break

                    if not error:
                        monsterFound = True
                        monsters.append([y, x])

                if image[y][x]:
                    monsterHit = False
                    for monsterPos in monsters:
                        if (y >= monsterPos[0] and x >= monsterPos[1] and y - monsterPos[0] < len(monster)
                                    and x - monsterPos[1] < len(monster[y - monsterPos[0]])
                                    and monster[y - monsterPos[0]][x - monsterPos[1]]):
                            monsterHit = True
                            break

                    if not monsterHit:
                        waves += 1

        if not monsterFound:
            waves = 0
            copy = [image.copy() for _ in image]
            for i in range(len(image)):
                for j in range(len(image[i])):
                    copy[j][len(image[i]) - 1 - i] = image[i][j]

            image = copy
        else:
            break

    return waves


def main():
    inf = os.path.dirname(os.getcwd())
    inf = os.path.join(inf, "input")
    if not os.path.exists(inf) or os.path.isdir(inf):
        print(inf + " does not exist!")
        inf += ".txt"
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print(inf + " does not exist!")
        inf = os.path.join(os.path.dirname(inf), "input")
        inf = os.path.join(inf, "Day20.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    lines = open(inf, 'r').readlines()
    tiles = {}
    for i in range(0, len(lines), 12):
        tile = [[lines[i + j + 1][k] == '#' for k in range(len(lines[i + j + 1]) - 1)] for j in range(10)]
        tiles[int(lines[i][5:9])] = Tile(tile)

    grid = [[0 for j in range(math.ceil(math.sqrt(len(tiles))))] for i in range(math.ceil(math.sqrt(len(tiles))))]
    for id in tiles.keys():
        put = False
        while not put:
            topMatches = False;
            bottomMatches = False;
            leftMatches = False;
            rightMatches = False;
            for i in tiles.keys():
                if id == i:
                    continue

                match, _, _ = tiles[id].fitsMirrored(tiles[i])
                if match == 1:
                    topMatches = True
                elif match == 2:
                    leftMatches = True
                elif match == 3:
                    bottomMatches = True
                elif match == 4:
                    rightMatches = True

                if topMatches and bottomMatches and leftMatches and rightMatches:
                    break

            if not topMatches and not leftMatches:
                if grid[0][0] == 0:
                    grid[0][0] = id
                    put = True
            elif (leftMatches or bottomMatches) and (bottomMatches or rightMatches) and (rightMatches or topMatches):
                put = True

            if not put:
                tiles[id] = tiles[id].rotate()

        if grid[0][0] != 0:
            break

    for i in range(len(grid) * len(grid) - 1):
        if grid[int((i + 1) / len(grid))][(i + 1) % len(grid)] != 0 and (int(i / len(grid)) == len(grid) - 1 or grid[int(i / len(grid) + 1)][i % len(grid)] != 0):
            continue

        last = tiles[grid[int(i / len(grid))][i % len(grid)]]
        for id in tiles.keys():
            side, rotation, mirror = last.fitsMirrored(tiles[id])
            if side == 3:
                grid[int(i / len(grid) + 1)][i % len(grid)] = id
            elif side == 4:
                grid[int((i + 1) / len(grid))][(i + 1) % len(grid)] = id
            else:
                continue

            rotated = tiles[id]

            if mirror == 1:
                rotated = rotated.mirror()

            for _ in range(rotation):
                rotated = rotated.rotate()

            tiles[id] = rotated

    part1sum = grid[0][0];
    part1sum *= grid[0][len(grid[0]) - 1];
    part1sum *= grid[len(grid) - 1][0];
    part1sum *= grid[len(grid) - 1][len(grid[len(grid) - 1]) - 1];
    print(f"The sum of the ids from the four corners is {part1sum}.");

    image = [[None] * (len(grid) * 8) for i in range(len(grid[0]) * 8)]
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            tile = tiles[grid[i][j]]
            for k in range(8):
                for l in range(8):
                    image[i * 8 + k][j * 8 + l] = tile.pixels[k + 1][l + 1]

    MONSTER = ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "]
    monster = [[MONSTER[i][j] == '#' for j in range(len(MONSTER[i]))] for i in range(len(MONSTER))]

    waves = countWaves(image, monster)

    print(f"The roughness of the water is {waves}.")


if __name__ == '__main__':
    main()
