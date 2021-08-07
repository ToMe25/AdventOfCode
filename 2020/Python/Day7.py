import os
import sys


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
        inf = os.path.join(inf, "Day7.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    bags = {}
    for line in inputFile.readlines():
        contained = {}
        for bag in line[line.index("tain ") + 5:].split(", "):
            if "no other" in bag:
                break
            contained[bag[2:bag.index(" bag")]] = int(bag[0])

        bags[line[:line.index(" bag")]] = contained

    goldContaining = 0
    for bag in bags.keys():
        if checkBag(bags, bag, "shiny gold"):
            goldContaining += 1

    print(f"Found {goldContaining} bags that contain a \"shiny gold\" bag.")

    print(f"A single \"shiny gold\" bag contains {countBags(bags, 'shiny gold') - 1} bags.")


def checkBag(bags, bag, lookingFor):
    for check in bags[bag].keys():
        if check == lookingFor:
            return True
        elif checkBag(bags, check, lookingFor):
            return True
    return False


def countBags(bags, bag):
    count = 1
    for check in bags[bag].keys():
        count += countBags(bags, check) * bags[bag][check]

    return count


if __name__ == '__main__':
    main()
