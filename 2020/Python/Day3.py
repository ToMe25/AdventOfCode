import os
import sys


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
        inf = os.path.join(inf, "Day3.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    map = read(inf)
    print(f"Part one hit {checkSlope(map, 3)} trees.")

    slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
    result = 1
    for slope in slopes:
        hits = checkSlope(map, slope[0], slope[1])
        print(f"Slope Right {slope[0]}, Down {slope[1]} hit {hits} trees.")
        result *= hits

    print(f"Part two result is {result}.")


def checkSlope(trees, right, down=1):
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
