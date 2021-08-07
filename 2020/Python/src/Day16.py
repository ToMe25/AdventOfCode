import InputReader
import re


def main():
    ownTicket = []
    tickets = []
    inputValidators = {}
    validator = lambda i, ran: (i >= ran[0] and i <= ran[1]) or (i >= ran[2] and i <= ran[3])
    your = False
    for line in InputReader.readInputFileLines(16):
        if len(line) < 3:
            continue

        if line.startswith("your"):
            your = True
            continue
        elif line.startswith("nearby"):
            your = False
            continue

        if not your and len(ownTicket) == 0:
            inputRange = [int(val) for val in re.split("-| or ", line[line.index(":") + 2:])]
            inputValidators[line[:line.index(":")]] = inputRange
        elif your:
            ownTicket = [int(field) for field in line.split(",")]
        else:
            ticket = [int(field) for field in line.split(",")]
            tickets.append(ticket)

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
