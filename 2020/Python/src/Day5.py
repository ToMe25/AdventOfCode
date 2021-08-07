import InputReader
import re


def main():
    found = []
    for line in InputReader.readInputFileLines(5):
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
