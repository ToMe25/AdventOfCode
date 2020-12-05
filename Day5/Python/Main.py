import os

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
    highest = 0
    found = []
    for line in inputFile.readlines():
        row = int(line[:7].replace("F", "0").replace("B", "1"), 2)
        column = int(line[-4:].replace("L", "0").replace("R", "1"), 2)
        seatId = row * 8 + column
        found.append(seatId)
        if seatId > highest:
            highest = seatId

    print(f"Highest found seat id is {highest}.")

    found.sort()
    last = 0
    for seat in found:
        if seat == last + 2:
            last = seat - 1
            break
        last = seat

    print(f"Your seat has id {last}.")

if __name__ == '__main__':
    main()
