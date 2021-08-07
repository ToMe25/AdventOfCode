import os

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
    input = os.path.dirname(os.getcwd())
    input = os.path.join(input, "input")
    if not os.path.exists(input):
        print(input + " does not exist!")
        input = os.path.join(os.path.dirname(input), "input.txt")
        print(f"trying {input} instead.")
        if not os.path.exists(input):
            print(input + " does not exist!")
            return;

    file = open(input, 'r')
    subjectNumber = 7
    key1 = int(file.readline())
    key2 = int(file.readline())
    file.close()

    encryptionKey = getKey(key2, getLoopSize(key1, subjectNumber))
    print(f"The encryption key is {encryptionKey}.")

if __name__ == '__main__':
    main()
