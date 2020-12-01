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

    inputFile = open(input, "r")
    lines = inputFile.readlines()
    inputs = []
    double = False
    triple = False
    for line in lines:
        nr = int(line)
        if not double and 2020 - nr in inputs:
            print("Found matching two number pair!")
            print(f"The numbers are {nr} and {2020 - nr}, and the result is {nr * (2020 - nr)}.")
            double = True

        if not triple:
            for i in inputs:
                if 2020 - nr - i in inputs:
                    print("Found matching three number pair!")
                    print(f"The numbers are {nr}, {i}, and {2020 - nr - i}, and the result is {nr * i * (2020 - nr - i)}.")
                    triple = True
                    break

        inputs.append(nr)
        if double and triple:
            return

    print("Could not find a matching number pair!")

if __name__ == '__main__':
    main()
