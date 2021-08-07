import InputReader
import re


def main():
    lines = InputReader.readInputFileLines(19)
    rules = [None] * round(len(lines) / 2)
    ruleReplacements = {8: "( 42 )+", 11: "( 42 ){n}( 31 ){n}"}
    pattern = ""
    pattern2 = ""
    matches = 0
    matches2 = 0
    for line in lines:
        if ":" in line:
            value = line[line.index(":") + 1:] + " "
            if "|" in value:
                value = f"({value})"

            if "\"" in value:
                value = value.replace('"', '')

            rules[int(line[:line.index(":")])] = value
        elif pattern == "":
            pattern = pattern2 = rules[0]
            while True:
                toReplace = 0
                for nr in pattern.split(" "):
                    if nr.isdigit():
                        toReplace = int(nr)
                        break

                if toReplace == 0:
                    break

                pattern = re.sub(f" {toReplace} ", f" {rules[toReplace]} ", pattern)
                if toReplace in ruleReplacements:
                    pattern2 = re.sub(f" {toReplace} ", f" {ruleReplacements[toReplace]} ", pattern2)
                else:
                    pattern2 = re.sub(f" {toReplace} ", f" {rules[toReplace]} ", pattern2)

                for i in range(len(rules)):
                    if rules[i] != None and rules[i] != "":
                        rules[i] = re.sub(f" {toReplace} ", f" {rules[toReplace]} ", rules[i])

                for i in ruleReplacements.keys():
                    ruleReplacements[i] = re.sub(f" {toReplace} ", f" {rules[toReplace]} ", ruleReplacements[i])

            pattern = pattern.replace(' ', '')
            pattern2 = pattern2.replace(' ', '')
        else:
            if line == "":
                continue

            if re.fullmatch(pattern, line) != None:
                matches += 1

            for i in range(1, 10):
                if re.fullmatch(pattern2.replace('n', str(i)), line) != None:
                    matches2 += 1

    print(f"Found {matches} valid part 1 messages.")
    print(f"Found {matches2} valid part 2 messages.")


if __name__ == '__main__':
    main()
