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
        inf = os.path.join(inf, "Day15.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    lastNumber = 0;
    lastNumbers = [0] * 30000000
    nr = 0
    for number in inputFile.readlines()[0].split(','):
        lastNumber = int(number);
        nr += 1
        lastNumbers[lastNumber] = nr

    inputFile.close()

    while nr < 30000000:
        num = 0
        if lastNumbers[lastNumber] != 0:
            num = nr - lastNumbers[lastNumber]

        lastNumbers[lastNumber] = nr
        lastNumber = num
        nr += 1

        if nr == 2020:
            print(f"The 2020th number is {lastNumber}.")

    print(f"The 30000000th number is {lastNumber}.")


if __name__ == '__main__':
    main()
