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

    map = read(input)
    print(f"Part one hit {checkSlope(map, 3)} trees.")

    slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
    result = 1
    for slope in slopes:
        hits = checkSlope(map, slope[0], slope[1])
        print(f"Slope Right {slope[0]}, Down {slope[1]} hit {hits} trees.")
        result *= hits

    print(f"Part two result is {result}.")

def checkSlope(trees, right, down = 1):
    treesHit = 0
    x = 0
    for y in range(down, len(trees), down):
        x = (x + right) % len(trees[y])
        if trees[y][x]:
            treesHit += 1

    return treesHit

def read(input):
    file = open(input, 'r')
    trees = []
    for line in file.readlines():
        row = []
        for c in line:
            if c != '\n':
                row.append(c == '#')

        trees.append(row)

    return trees

if __name__ == '__main__':
    main()
