import os
import os.path as path
import sys


# Gets the input file to read for the given day.
def getInputFile(day):
    inf = path.dirname(os.getcwd())
    inf = path.join(inf, "input")
    if not path.exists(inf) or not path.isfile(inf):
        print(inf + " does not exist!")
        inf += ".txt"
        print(f"trying {inf} instead.")

    if not path.exists(inf) or not path.isfile(inf):
        print(inf + " does not exist!")
        inf = path.join(path.dirname(path.dirname(os.getcwd())), "input")
        inf = path.join(inf, f"Day{day}.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or not os.path.isfile(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        sys.exit(1)
    else:
        print(f"Using input file {inf}.")

    return inf


# Reads all the lines from the input file as a list of strings.
def readInputFileLines(day):
    inf = getInputFile(day)
    lines = []
    inputFile = open(inf, 'r')
    for line in inputFile.readlines():
        lines.append(line.strip())

    inputFile.close()
    return lines


# Reads all the lines from the input file as strings and parses them to ints.
def readInputFileNumbers(day):
    numbers = []
    for line in readInputFileLines(day):
        numbers.append(int(line))

    return numbers
