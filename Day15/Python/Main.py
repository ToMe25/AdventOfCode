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
    lastNumber = 0;
    lastNumbers = [0]*30000000
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
