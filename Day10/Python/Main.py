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
    adapters = []
    for line in inputFile.readlines():
        adapters.append(int(line[:-1]))

    adapters.sort()

    one = 0
    three = 1
    possibleSolutions = 1
    futureSolutions = list()
    for i in range(3):
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
