import os
import sys
import re


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
        inf = os.path.join(inf, "Day16.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')

    ownTicket = []
    tickets = []
    inputValidators = {}
    validator = lambda i, range: (i >= range[0] and i <= range[1]) or (i >= range[2] and i <= range[3])
    your = False
    for line in inputFile.readlines():
        if len(line) < 3:
            continue

        if line.startswith("your"):
            your = True
            continue
        elif line.startswith("nearby"):
            your = False
            continue

        if not your and len(ownTicket) == 0:
            inputRange = [int(val) for val in re.split("-| or ", line[line.index(":") + 2:-1])]
            inputValidators[line[:line.index(":")]] = inputRange
        elif your:
            ownTicket = [int(field) for field in line[:-1].split(",")]
        else:
            ticket = [int(field) for field in line[:-1].split(",")]
            tickets.append(ticket)

    inputFile.close()

    invalid = 0
    invalidTickets = []
    for ticket in tickets:
        for field in ticket:
            if len([0 for inputRange in inputValidators.values() if validator(field, inputRange)]) == 0:
                invalid += field
                invalidTickets.append(ticket)

    print(f"Found an error rate of {invalid}.")

    for ticket in invalidTickets:
        tickets.remove(ticket)

    possiblePositions = {}
    for field in inputValidators:
        for i in range(len(ownTicket)):
            if not validator(ownTicket[i], inputValidators[field]):
                continue
            elif not all(validator(ticket[i], inputValidators[field]) for ticket in tickets):
                continue
            else:
                positions = set()
                if field in possiblePositions:
                    positions = possiblePositions[field]

                positions.add(i)
                possiblePositions[field] = positions

    fields = [None] * len(ownTicket)
    while None in fields:
        for i in range(len(ownTicket)):
            if [i in v for v in possiblePositions.values()].count(True) == 1:
                fields[i] = next(field for field in possiblePositions if i in possiblePositions[field])
                for field in possiblePositions:
                    if i in possiblePositions[field]:
                        possiblePositions[field].remove(i)

        for field in [field for field in possiblePositions if len(possiblePositions[field]) == 1]:
            if len(possiblePositions[field]) == 0:
                continue

            pos = next(iter(possiblePositions[field]))
            fields[pos] = field
            for field in possiblePositions:
                if pos in possiblePositions[field]:
                    possiblePositions[field].remove(pos)

    result = 1
    for i in range(len(ownTicket)):
        if fields[i].startswith("departure"):
            result *= ownTicket[i]

    print(f"The part 2 result is {result}.")


if __name__ == '__main__':
    main()
