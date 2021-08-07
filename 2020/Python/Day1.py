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
        inf = os.path.join(inf, "Day1.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    inputs = []
    double = False
    triple = False
    for line in inputFile.readlines():
        nr = int(line)
        if not double and 2020 - nr in inputs:
            print("Found matching two number pair!")
            print(f"The numbers are {nr} and {2020 - nr}, and the result is {nr * (2020 - nr)}.")
            double = True

        if not triple:
            for i in inputs:
                if 2020 - nr - i in inputs:
                    print("Found matching three number pair!")
                    print(f"The numbers are {nr}, {i}, and {2020 - nr - i}, and the result is {nr * i * (2020 - nr - i)}.")
                    triple = True
                    break

        inputs.append(nr)
        if double and triple:
            return

    print("Could not find a matching number pair!")


if __name__ == '__main__':
    main()
