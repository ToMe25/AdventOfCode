import os

class Cup:

    def __init__(self, label):
        self.label = label

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

    line = open(input, 'r').readline()
    cups = [None]*(len(line) - 1)
    min_cup = -1
    max_cup = -1
    for i in range(len(cups)):
        cups[i] = int(str(line[i]))
        if cups[i] < min_cup or min_cup == -1:
            min_cup = cups[i]
        if cups[i] > max_cup or max_cup == -1:
            max_cup = cups[i]

    cups_initial = cups.copy()

    picked = [None]*3
    for i in range(100):
        for j in range(len(picked)):
            picked[j] = cups[(i + j + 1) % len(cups)]

        target = cups[i % len(cups)] - 1
        if target < min_cup:
            target = max_cup

        destination = cups.index(target)
        while target in picked:
            target -= 1
            if target < min_cup:
                target = max_cup
            destination = cups.index(target)

        if destination > i % len(cups):
            for k in range(i % len(cups) + 1, destination - len(picked) + 1):
                cups[k % len(cups)] = cups[(k + len(picked)) % len(cups)]
        else:
            for k in range(i % len(cups) + 1, len(cups) + destination - len(picked) + 1):
                cups[k % len(cups)] = cups[(k + len(picked)) % len(cups)];
                cups[(k + len(picked)) % len(cups)] = -1;

        destination = cups.index(target)
        for k in range(len(picked)):
            cups[(destination + k + 1) % len(cups)] = picked[k];

    result = ""
    start = cups.index(1)
    for i in range(1, len(cups)):
        result += str(cups[(i + start) % len(cups)])
    print(f"The part 1 result is {result}.")

    cups2 = [None]*1000000
    startCup = Cup(cups_initial[0])
    cups2[cups_initial[0] - 1] = startCup
    current = previous = startCup

    for i in range(2, len(cups2) + 1):
        if i <= len(cups_initial):
            current = Cup(cups_initial[i - 1])
        else:
            current = Cup(i)

        previous.next = current
        previous = current
        cups2[current.label - 1] = current

    current.next = startCup

    current = startCup
    picked2 = [None]*3
    for i in range(10000000):
        picked2[0] = current
        for j in range(len(picked2)):
            picked2[j] = picked2[max(j - 1, 0)].next

        if current.label > 1:
            target = cups2[current.label - 2]
        else:
            target = cups2[len(cups2) - 1]

        while target in picked2:
            if current.label > 1:
                target = cups2[target.label - 2]
            else:
                target = cups2[len(cups2) - 1]

        current.next = picked2[len(picked2) - 1].next
        oldNext = target.next
        target.next = picked2[0]
        picked2[len(picked2) - 1].next = oldNext

        current = current.next

    result2 = cups2[0].next.label * cups2[0].next.next.label;
    print(f"The part 2 result is {result2}.");

if __name__ == '__main__':
    main()
