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
    i = 0
    last = []
    invalid = 0
    for line in inputFile.readlines():
        number = int(line[:-1])
        if i > 25:
            found = False;
            for n in range(1, 26):
                if last[i - n] != number / 2:
                    for j in range(1, 26):
                        if last[i - j] == number - last[i - n]:
                            found = True
                            break

            if not found:
                print(f"Couldn't find number pair for {number}.")
                invalid = number
        last.append(number)
        i += 1

    for nr in range(len(last)):
        min = last[nr]
        max = last[nr]
        sum = 0
        i = 0
        while sum < invalid:
            if nr + i >= len(last) or last[nr + i] == invalid:
                break
            if last[nr + i] < min:
                min = last[nr + i]
            elif last[nr + i] > max:
                max = last[nr + i]
            sum += last[nr + i]
            i += 1

        if sum == invalid:
            print(f"Found contiguous set adding up to invalid: [min: {min}, max: {max}, weakness: {min + max}].")
            break

if __name__ == '__main__':
    main()
