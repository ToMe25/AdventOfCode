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
    globalGroupFound = 0
    globalFound = 0
    groupFoundEvery = []
    groupFoundAny = []
    found = []
    newline = False
    first = True
    for b in inputFile.read():
        if b == '\n':
            if newline:
                globalFound += len(groupFoundAny)
                globalGroupFound += len(groupFoundEvery)
                first = True
            else:
                if first:
                    groupFoundAny = found.copy()
                    groupFoundEvery = found.copy()
                    first = False
                else:
                    for a in found:
                        if not a in groupFoundAny:
                            groupFoundAny.append(a)
                    for a in groupFoundAny:
                        if not a in found and a in groupFoundEvery:
                            groupFoundEvery.remove(a)

                found.clear()
                newline = True
        else:
            newline = False
            if not b in found:
                found.append(b)

    globalFound += len(groupFoundAny)
    globalGroupFound += len(groupFoundEvery)

    print(f"The sum of all group part 1 answers is {globalFound}.")
    print(f"The sum of all group part 2 answers is {globalGroupFound}.")

if __name__ == '__main__':
    main()
