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
    instructions = inputFile.readlines()
    inputFile.close()

    executed = []
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
