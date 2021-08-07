import InputReader


def main():
    adapters = InputReader.readInputFileNumbers(10)
    adapters.sort()

    one = 0
    three = 1
    possibleSolutions = 1
    futureSolutions = list()
    for _ in range(3):
        futureSolutions.append(0)
    joltage = 0

    for adapter in adapters:
        lowest = adapter - joltage;
        if joltage + 2 in adapters and lowest < 2:
            futureSolutions[1] += possibleSolutions
        if joltage + 3 in adapters and lowest < 3:
            futureSolutions[2] += possibleSolutions
        possibleSolutions += futureSolutions[0]
        futureSolutions.remove(futureSolutions[0])
        futureSolutions.append(0)
        if adapter - joltage == 1:
            one += 1
        elif adapter - joltage == 3:
            three += 1
        joltage = adapter

    print(f"Found {one} one jolt differences and {three} three jolt differences, getting a step one result of {one * three}.")

    print(f"Found {possibleSolutions} valid solutions.")


if __name__ == '__main__':
    main()
