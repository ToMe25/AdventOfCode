import InputReader


def main():
    globalGroupFound = 0
    globalFound = 0
    groupFoundEvery = []
    groupFoundAny = []
    found = []
    newline = False
    first = True
    for b in open(InputReader.getInputFile(6), 'r').read():
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
                    groupFoundEvery = list(set(groupFoundEvery) & set(found))

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
