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
        inf = os.path.join(inf, "Day2.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    correctPwds1 = 0
    correctPwds2 = 0
    for line in inputFile.readlines():
        inputs = line.split(' ')
        range = inputs[0].split('-')
        min = int(range[0])
        max = int(range[1])
        occurrences = inputs[2].count(inputs[1][0])
        if min <= occurrences <= max:
            correctPwds1 += 1

        if (inputs[2][min - 1] == inputs[1][0]) != (inputs[2][max - 1] == inputs[1][0]):
            correctPwds2 += 1

    print(f"Found {correctPwds1} correct step 1 passwords.")
    print(f"Found {correctPwds2} correct step 2 passwords.")


if __name__ == '__main__':
    main()
