import InputReader


def main():
    instructions = InputReader.readInputFileLines(8)

    result = runCode(instructions.copy(), [], 0, 0, False)
    print(f"The acc value was {result[0]} just before executing the first instruction a second time.")

    result = runCode(instructions.copy(), [], 0, 0, True)
    if result[1] == 1:
        print(f"The acc value was {result[0]} after executing everything.")
    else:
        print("Failed to fully execute the code.")


def runCode(instructions, executed, current, acc, change):
    """Executes the given list of instructions.
    
    Each instruction has an integer value to be used for its function, this
    can be positive or negative.
    Supported instructions:
    
    * acc: adds its integer value to the accumulator value
    * jmp: jumps the amount of instructions defined by its integer value
      forward
    * nop: does nothing
    
    Terminates when the first instruction would be executed a second time,
    or the instruction to execute is after the last instruction.
    
    Parameters
    ----------
    instructions: list of str
        The list of instructions to execute.
    executed: list of int
        A list containing the positions of instructions to be considered
        executed already.
    acc: int
        The starting accumulator value.
    change: bool
        Whether a single jmp or nop instruction should be changed to make
        the program terminate by trying to execute an instruction after the
        last one, rather then executing an earlier one twice.
    
    Returns
    -------
    list of int
        An integer list containing the current accumulator value, and an
        exit code. This exit code is 1 if the program terminated by trying
        to execute an instruction after the last one. If it tried to
        execute an instruction twice the exit code is 0.
    """
    while current < len(instructions):
        inst = instructions[current][:3]
        add = 1
        if inst == "acc":
            acc += int(instructions[current][4:])
        elif inst == "jmp":
            add = int(instructions[current][4:])
            if change:
                instructions[current] = "nop" + instructions[current][3:]
                result = runCode(instructions, executed.copy(), current, acc, False)
                if result[1] == 1:
                    return result
        elif change:
            instructions[current] = "jmp" + instructions[current][3:]
            result = runCode(instructions, executed.copy(), current, acc, False)
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
