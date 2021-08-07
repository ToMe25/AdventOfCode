import os
import sys


def getLoopSize(key, subjectNumber):
    loopSize = 0
    while key != 1:
        newKey = key / subjectNumber
        while newKey % 1 != 0:
            key += 20201227
            newKey = key / subjectNumber

        key = newKey
        loopSize += 1

    return loopSize


def getKey(subjectNumber, loopSize):
    value = 1
    for _ in range(loopSize):
        value = value * subjectNumber % 20201227
    return value


def main():
    inf = os.path.dirname(os.getcwd())
    inf = os.path.join(inf, "input")
    if not os.path.exists(inf) or os.path.isdir(inf):
        print(inf + " does not exist!")
        inf += ".txt"
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print(inf + " does not exist!")
        inf = os.path.join(os.path.dirname(inf), "input")
        inf = os.path.join(inf, "Day25.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    file = open(inf, 'r')
    subjectNumber = 7
    key1 = int(file.readline())
    key2 = int(file.readline())
    file.close()

    encryptionKey = getKey(key2, getLoopSize(key1, subjectNumber))
    print(f"The encryption key is {encryptionKey}.")


if __name__ == '__main__':
    main()
