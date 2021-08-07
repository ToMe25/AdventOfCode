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
        inf = os.path.join(inf, "Day10.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    adapters = []
    for line in inputFile.readlines():
        adapters.append(int(line[:-1]))

    adapters.sort()

    one = 0
    three = 1
    possibleSolutions = 1
    futureSolutions = list()
    for _ in range(3):
        futureSolutions.append(0)
    joltage = 0
    for adapter in adapters:
        lowest = adapter - joltage;
        if joltage + 2 in adapters and lowest < 2:
            futureSolutions[1] += possibleSolutions
        if joltage + 3 in adapters and lowest < 3:
            futureSolutions[2] += possibleSolutions
        possibleSolutions += futureSolutions[0]
        futureSolutions.remove(futureSolutions[0])
        futureSolutions.append(0)
        if adapter - joltage == 1:
            one += 1
        elif adapter - joltage == 3:
            three += 1
        joltage = adapter

    print(f"Found {one} one jolt differences and {three} three jolt differences, getting a step one result of {one * three}.")

    print(f"Found {possibleSolutions} valid solutions.")


if __name__ == '__main__':
    main()
