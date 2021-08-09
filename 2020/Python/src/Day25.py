import InputReader


def main():
    keys = InputReader.readInputFileNumbers(25)
    subjectNumber = 7

    encryptionKey = getKey(keys[1], getLoopSize(keys[0], subjectNumber))
    print(f"The encryption key is {encryptionKey}.")


def getKey(subjectNumber, loopSize):
    """Generates the encryption key from the subject number and the loop size.
    
    The encryption key starts as 1.
    Then the following actions get executed loop size times:
    
    * multiply the previous value by the subject number
    * set the encryption key to the remainder from dividing the previous
      result by 20201227
    
    Parameters
    ----------
    subjectNumber: int
        The subject number to use in the calculation.
    loopSize: int
        How many times to execute the calculation.
    
    Returns
    -------
    int
        The resulting encryption key.
    """
    value = 1
    for _ in range(loopSize):
        value = value * subjectNumber % 20201227
    return value


def getLoopSize(key, subjectNumber):
    """Reverse engineers the loop size from the given key and subject number.
    
    This is done by continually dividing the key by the subject number
    until the result matches 1.
    If the result has decimal digits 20201227 gets added to the previous
    key, before it is divided again.
    By counting the divisions the loop size can be determined.
    
    Parameters
    ----------
    key: int
        The key to get the loop size for.
    subjectNumber: int
        The subject number used to generate the key.
    
    Returns
    -------
    int
        The loop size used to generate the given key.
    """
    loopSize = 0
    while key != 1:
        newKey = key / subjectNumber
        while newKey % 1 != 0:
            key += 20201227
            newKey = key / subjectNumber

        key = newKey
        loopSize += 1

    return loopSize


if __name__ == '__main__':
    main()
