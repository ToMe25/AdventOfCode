import os
import sys
import re


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
        inf = os.path.join(inf, "Day5.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    found = []
    for line in inputFile.readlines():
        line = re.sub("[FL]", "0", re.sub("[BR]", "1", line))

        found.append(int(line, 2))

    found.sort()
    print(f"Highest found seat id is {found[-1]}.")

    last = 0
    for seat in found:
        if seat == last + 2:
            last = seat - 1
            break
        last = seat

    print(f"Your seat has id {last}.")

if __name__ == '__main__':
    main()
