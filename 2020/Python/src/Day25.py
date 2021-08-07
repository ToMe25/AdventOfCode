import InputReader


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
    keys = InputReader.readInputFileNumbers(25)
    subjectNumber = 7

    encryptionKey = getKey(keys[1], getLoopSize(keys[0], subjectNumber))
    print(f"The encryption key is {encryptionKey}.")


if __name__ == '__main__':
    main()
