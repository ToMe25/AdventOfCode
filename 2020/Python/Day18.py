import os
import sys
import re


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
        inf = os.path.join(inf, "Day18.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    globalResult = 0
    globalResult2 = 0
    for line in open(inf, 'r').readlines():
        line = line[:-1]
        globalResult += evaluate(line)
        globalResult2 += evaluate2(line)

    print(f"The result of all equation results for part 1 added together is {globalResult}.")
    print(f"The result of all equation results for part 2 added together is {globalResult2}.")


def evaluate(equation):
    result = 0
    operation = '+'
    i = 0
    while i < len(equation):
        c = equation[i]
        if c == ' ':
            i += 1
            continue
        elif c == '(':
            open = 0
            for j in range(i, len(equation)):
                if equation[j] == '(':
                    open += 1
                elif equation[j] == ')':
                    open -= 1

                if open == 0:
                    break

            if operation == '+':
                result += evaluate(equation[i + 1:j])
            elif operation == '*':
                result *= evaluate(equation[i + 1:j])

            i = j
        elif c.isdigit():
            if operation == '+':
                result += int(c)
            elif operation == '*':
                result *= int(c)
        else:
            operation = c
        i += 1

    return result


def evaluate2(equation):
    open = 0
    while equation.find("(") != -1:
        start = equation.find("(")
        for i in range(start, len(equation)):
            if equation[i] == '(':
                open += 1
            elif equation[i] == ')':
                open -= 1

            if (open == 0):
                break

        equation = equation.replace(equation[start:i + 1], str(evaluate2(equation[start + 1:i])))

    while equation.find('+') != -1:
        index = equation.find('+')
        equation = re.sub("\d+ \+ \d+", str(int(re.findall("\d+", equation[:index])[-1]) + int(re.search("\d+", equation[index:]).group())), equation, 1)

    while equation.find('*') != -1:
        index = equation.find('*')
        equation = re.sub("\d+ \* \d+", str(int(re.findall("\d+", equation[:index])[-1]) * int(re.search("\d+", equation[index:]).group())), equation, 1)

    return int(equation)


if __name__ == '__main__':
    main()
