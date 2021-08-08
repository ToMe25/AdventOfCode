import InputReader


def main():
    bags = {}
    for line in InputReader.readInputFileLines(7):
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
    """Checks whether the given bag contains a bag of a given type.
    
    This is done by recursively iterating through the bags in the given bag.
    
    Parameters
    ----------
    bags: dict of str to dict of str to int
        The map of what bags contain what types of bags.
    bag: str
        The bag to check.
    lookingFor: str
        The bag to check for.
    
    Returns
    -------
    bool
        Whether or not the given bag contains the bag to look for.
    """
    for check in bags[bag].keys():
        if check == lookingFor:
            return True
        elif checkBag(bags, check, lookingFor):
            return True
    return False


def countBags(bags, bag):
    """Counts the number of bags in the given bag.
    
    Done by recursively iterating through its contents.
    
    Parameters
    ----------
    bags: dict of str to dict of str to int
        The map of what bags contain what other bags.
    bag: str
        The bag to check.
    
    Returns
    -------
    int
        The total bag count. Includes the start bag.
    """
    count = 1
    for check in bags[bag].keys():
        count += countBags(bags, check) * bags[bag][check]

    return count


if __name__ == '__main__':
    main()
