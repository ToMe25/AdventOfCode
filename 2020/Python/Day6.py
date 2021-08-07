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
        inf = os.path.join(inf, "Day6.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    globalGroupFound = 0
    globalFound = 0
    groupFoundEvery = []
    groupFoundAny = []
    found = []
    newline = False
    first = True
    for b in inputFile.read():
        if b == '\n':
            if newline:
                globalFound += len(groupFoundAny)
                globalGroupFound += len(groupFoundEvery)
                first = True
            else:
                if first:
                    groupFoundAny = found.copy()
                    groupFoundEvery = found.copy()
                    first = False
                else:
                    for a in found:
                        if not a in groupFoundAny:
                            groupFoundAny.append(a)
                    groupFoundEvery = list(set(groupFoundEvery) & set(found))

                found.clear()
                newline = True
        else:
            newline = False
            if not b in found:
                found.append(b)

    globalFound += len(groupFoundAny)
    globalGroupFound += len(groupFoundEvery)

    print(f"The sum of all group part 1 answers is {globalFound}.")
    print(f"The sum of all group part 2 answers is {globalGroupFound}.")


if __name__ == '__main__':
    main()
