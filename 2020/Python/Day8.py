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
        inf = os.path.join(inf, "Day8.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    instructions = inputFile.readlines()
    inputFile.close()

    result = checkCode(instructions.copy(), [], 0, 0, False)
    print(f"The acc value was {result[0]} just before executing the first instruction a second time.")

    result = checkCode(instructions.copy(), [], 0, 0, True)
    if result[1] == 1:
        print(f"The acc value was {result[0]} after executing everything.")
    else:
        print("Failed to fully execute the code.")


def checkCode(instructions, executed, current, acc, change):
    while current < len(instructions):
        inst = instructions[current][:3]
        add = 1
        if inst == "acc":
            acc += int(instructions[current][4:-1])
        elif inst == "jmp":
            add = int(instructions[current][4:-1])
            if change:
                instructions[current] = "nop" + instructions[current][3:]
                result = checkCode(instructions, executed.copy(), current, acc, False)
                if result[1] == 1:
                    return result
        elif change:
            instructions[current] = "jmp" + instructions[current][3:]
            result = checkCode(instructions, executed.copy(), current, acc, False)
            if result[1] == 1:
                return result

        if current + add in executed:
            return [acc, 0]
        else:
            executed.append(current)
            current += add

    return [acc, 1]


if __name__ == '__main__':
    main()
