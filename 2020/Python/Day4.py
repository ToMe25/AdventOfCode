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
        inf = os.path.join(inf, "Day4.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    requiredFields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    validEyeColors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    validPassports1 = 0
    validPassports2 = 0
    fields = {}
    for line in inputFile.readlines():
        line = line.rstrip()
        if len(line) <= 1:
            invalid1 = False
            invalid2 = False
            for field in requiredFields:
                if field in fields:
                    if field == "byr":
                        byr = int(fields[field])
                        if byr < 1920 or byr > 2002:
                            invalid2 = True
                    elif field == "iyr":
                        iyr = int(fields[field])
                        if iyr < 2010 or iyr > 2020:
                            invalid2 = True
                    elif field == "eyr":
                        eyr = int(fields[field])
                        if eyr < 2020 or eyr > 2030:
                            invalid2 = True
                    elif field == "hgt":
                        hgt = fields[field]
                        if len(hgt) < 4:
                            invalid2 = True
                            continue
                        unit = hgt[-2:]
                        value = int(hgt[:-2])
                        if unit == "cm":
                            if value < 150 or value > 193:
                                invalid2 = True
                        elif unit == "in":
                            if value < 59 or value > 76:
                                invalid2 = True
                    elif field == "hcl":
                        hcl = fields[field]
                        if hcl[0] != '#' or len(hcl) != 7:
                            invalid2 = True
                            continue
                        try:
                            int(hcl[1:], 16)
                        except ValueError:
                            invalid2 = True
                    elif field == "ecl":
                        if not fields[field] in validEyeColors:
                            invalid2 = True
                    elif field == "pid":
                        pid = fields[field]
                        if len(pid) != 9:
                            invalid2 = True
                            continue
                        try:
                            int(pid)
                        except ValueError:
                            invalid2 = True
                            
                else:
                    invalid1 = invalid2 = True
                    break;

            if not invalid1:
                validPassports1 += 1
            if not invalid2:
                validPassports2 += 1
            fields.clear()
        else:
            for field in line.split(" "):
                fields[field.split(":")[0]] = field.split(":")[1]

    print(f"Step 1 found {validPassports1} valid passports.")
    print(f"Step 2 found {validPassports2} valid passports.")


if __name__ == '__main__':
    main()
