import os

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

    inputFile = open(input, 'r')
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
