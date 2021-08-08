import os
import os.path as path
import sys


def getInputFile(day):
    """Gets the input file to read for the given day.
    
    Parameters
    ----------
    day: int
        The day for which to read the input file.
    
    Returns
    -------
    str
        The path of the file to read for this days puzzle.
    """
    inf = path.dirname(os.getcwd())
    inf = path.join(inf, "input")
    if not path.exists(inf) or not path.isfile(inf):
        print(inf + " does not exist!")
        inf += ".txt"
        print(f"trying {inf} instead.")

    if not path.exists(inf) or not path.isfile(inf):
        print(inf + " does not exist!")
        inf = path.join(path.dirname(path.dirname(os.getcwd())), "input")
        inf = path.join(inf, f"Day{day}.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or not os.path.isfile(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        sys.exit(1)
    else:
        print(f"Using input file {inf}.")

    return inf


def readInputFileLines(day):
    """Reads all the lines from the input file as a list of strings.
    
    Parameters
    ----------
    day: int
        The day for which to read the input file.
    
    Returns
    -------
    list of str
        A list containing a string for each line of the input file.
    """
    inf = getInputFile(day)
    lines = []
    inputFile = open(inf, 'r')
    for line in inputFile.readlines():
        lines.append(line.strip())

    inputFile.close()
    return lines


def readInputFileNumbers(day):
    """Reads all the lines from the input file as strings and parses them to ints.
    
    Parameters
    ----------
    day: int
        The day for which to read the input file.
    
    Returns
    -------
    list of int
        A list containing an int for each line of the input file.
    """
    numbers = []
    for line in readInputFileLines(day):
        numbers.append(int(line))

    return numbers
