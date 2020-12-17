import os

def main():
    input = os.path.dirname(os.getcwd())
    input = os.path.join(input, "input")
    if not os.path.exists(input):
        print(input + " does not exist!")
        input = os.path.join(os.path.dirname(input), "input.txt")
        print(f"trying {input} instead.")
        if not os.path.exists(input):
            print(input + " does not exist!")
            return;

    cycles = 6
    lines = open(input, 'r').readlines()
    dimension1 = [[[False]*(cycles * 2 + len(lines[0]) - 1) for j in range(cycles * 2 + len(lines))] for i in range(cycles * 2 + 1)]
    dimension2 = [[[False]*(cycles * 2 + len(lines[0]) - 1) for j in range(cycles * 2 + len(lines))] for i in range(cycles * 2 + 1)]
    dimension3 = [[[[False]*(cycles * 2 + len(lines[0]) - 1) for k in range(cycles * 2 + len(lines))] for j in range(cycles * 2 + 1)] for i in range(cycles * 2 + 1)]
    dimension4 = [[[[False]*(cycles * 2 + len(lines[0]) - 1) for k in range(cycles * 2 + len(lines))] for j in range(cycles * 2 + 1)] for i in range(cycles * 2 + 1)]
    for y in range(len(lines)):
        line = lines[y]
        for x in range(len(lines[y]) - 1):
            dimension1[cycles][cycles + y][cycles + x] = dimension2[cycles][cycles + y][cycles + x] = lines[y][x] == '#'
            dimension3[cycles][cycles][cycles + y][cycles + x] = dimension4[cycles][cycles][cycles + y][cycles + x] = lines[y][x] == '#'

    for i in range(cycles):
        if i % 2 == 0:
            simulate3D(dimension1, dimension2)
        else:
            simulate3D(dimension2, dimension1)

    active = 0
    for z in range(len(dimension1)):
        for y in range(len(dimension1[z])):
            for x in range(len(dimension1[z][y])):
                if cycles % 2 == 0 and dimension2[z][y][x]:
                    active += 1
                elif cycles % 2 == 1 and dimension1[z][y][x]:
                    active += 1

    print(f"Found {active} active cubes in 3D space.")

    for i in range(cycles):
        if i % 2 == 0:
            simulate4D(dimension3, dimension4)
        else:
            simulate4D(dimension4, dimension3)

    active = 0
    for w in range(len(dimension3)):
        for z in range(len(dimension3[w])):
            for y in range(len(dimension3[w][z])):
                for x in range(len(dimension3[w][z][y])):
                    if cycles % 2 == 0 and dimension4[w][z][y][x]:
                        active += 1
                    elif cycles % 2 == 1 and dimension3[w][z][y][x]:
                        active += 1

    print(f"Found {active} active cubes in 4D space.")

def simulate3D(current, last):
    for z in range(len(current)):
        for y in range(len(current[z])):
            for x in range(len(current[z][y])):
                current[z][y][x] = check3D(last, x, y, z)

def check3D(last, x, y, z):
    activeNeighbors = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            for k in range(-1, 2):
                if z + i < 0 or z + i >= len(last) or y + j < 0 or y + j >= len(last[z + i]) or x + k < 0 or x + k >= len(last[z + i][y + j]):
                    continue
                elif i == 0 and j == 0 and k == 0:
                    continue
                elif last[z + i][y + j][x + k]:
                    activeNeighbors += 1
                    if activeNeighbors > 3:
                        return False

    if last[z][y][x]:
        return activeNeighbors == 2 or activeNeighbors == 3
    else:
        return activeNeighbors == 3

def simulate4D(current, last):
    for w in range(len(current)):
        for z in range(len(current[w])):
            for y in range(len(current[w][z])):
                for x in range(len(current[w][z][y])):
                    current[w][z][y][x] = check4D(last, w, x, y, z)

def check4D(last, w, x, y, z):
    activeNeighbors = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            for k in range(-1, 2):
                for l in range(-1, 2):
                    if w + i < 0 or w + i >= len(last) or z + j < 0 or z + j >= len(last[w + i]):
                        continue
                    elif y + k < 0 or y + k >= len(last[w + i][z + j]) or x + l < 0 or x + l >= len(last[w + i][z + j][y + k]):
                        continue
                    elif i == 0 and j == 0 and k == 0 and l == 0:
                        continue
                    elif last[w + i][z + j][y + k][x + l]:
                        activeNeighbors += 1
                        if activeNeighbors > 3:
                            return False

    if last[w][z][y][x]:
        return activeNeighbors == 2 or activeNeighbors == 3
    else:
        return activeNeighbors == 3

if __name__ == '__main__':
    main()
