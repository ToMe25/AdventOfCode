import os
import re

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

    inputFile = open(input, 'r')
    found = []
    for line in inputFile.readlines():
        line = re.sub("[FL]", "0", re.sub("[BR]", "1", line))

        found.append(int(line[:7], 2) * 8 + int(line[-4:], 2))

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
