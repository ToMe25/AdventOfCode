import os

def main():
    input = os.path.dirname(os.getcwd())
    input = os.path.join(input, "input")
    if not os.path.exists(input):
        print(input + " does not exist!")
        input = os.path.join(os.path.dirname(input), "input.txt")
        if not os.path.exists(input):
            print(input + " does not exist!")
            return;

    inputFile = open(input, 'r')
    correctPwds1 = 0
    correctPwds2 = 0
    for line in inputFile.readlines():
        inputs = line.split(' ')
        range = inputs[0].split('-')
        min = int(range[0])
        max = int(range[1])
        occurrences = inputs[2].count(inputs[1][0])
        if occurrences >= min and occurrences <= max:
            correctPwds1 += 1

        if (inputs[2][min - 1] == inputs[1][0]) != (inputs[2][max - 1] == inputs[1][0]):
            correctPwds2 += 1

    print(f"Found {correctPwds1} correct step 1 passwords.")
    print(f"Found {correctPwds2} correct step 2 passwords.")

if __name__ == '__main__':
    main()