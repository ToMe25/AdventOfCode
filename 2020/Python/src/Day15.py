import InputReader


def main():
    lastNumber = 0;
    lastNumbers = [0] * 30000000
    nr = 0
    for number in InputReader.readInputFileLines(15)[0].split(','):
        lastNumber = int(number);
        nr += 1
        lastNumbers[lastNumber] = nr

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
