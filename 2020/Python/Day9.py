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
        inf = os.path.join(inf, "Day9.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    i = 0
    last = []
    invalid = 0
    for line in inputFile.readlines():
        number = int(line[:-1])
        if i > 25:
            found = False;
            for n in range(1, 26):
                if last[i - n] != number / 2:
                    for j in range(1, 26):
                        if last[i - j] == number - last[i - n]:
                            found = True
                            break

            if not found:
                print(f"Couldn't find number pair for {number}.")
                invalid = number
        last.append(number)
        i += 1

    for nr in range(len(last)):
        min = last[nr]
        max = last[nr]
        sum = 0
        i = 0
        while sum < invalid:
            if nr + i >= len(last) or last[nr + i] == invalid:
                break
            if last[nr + i] < min:
                min = last[nr + i]
            elif last[nr + i] > max:
                max = last[nr + i]
            sum += last[nr + i]
            i += 1

        if sum == invalid:
            print(f"Found contiguous set adding up to invalid: [min: {min}, max: {max}, weakness: {min + max}].")
            break


if __name__ == '__main__':
    main()
