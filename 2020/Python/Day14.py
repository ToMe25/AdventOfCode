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
        inf = os.path.join(inf, "Day14.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    zeroMask = 0
    oneMask = 0
    memory = {}
    floatingZeroMasks = []
    floatingOneMasks = []
    memory2 = {}
    for line in inputFile.readlines():
        if line[:4] == "mask":
            zeroMask = int(line[7:].replace('X', '1'), 2)
            oneMask = int(line[7:].replace('X', '0'), 2)

            floatingZeroMasks.clear()
            floatingOneMasks.clear()
            masksf = [""]
            for c in line[7:]:
                if c == 'X':
                    for i in range(len(masksf)):
                        masksf.append(masksf[i] + '0')
                        masksf[i] += '1'
                elif c == '1':
                    for i in range(len(masksf)):
                        masksf[i] += '1'
                elif c == '0':
                    for i in range(len(masksf)):
                        masksf[i] += 'X'

            for mask in masksf:
                floatingZeroMasks.append(int(mask.replace('X', '1'), 2))
                floatingOneMasks.append(int(mask.replace('X', '0'), 2))
        elif line[:3] == "mem":
            key = int(line[4:line.index(']')])
            value = int(line[line.index('=') + 2:])
            value &= zeroMask
            value |= oneMask
            memory[key] = value

            value = int(line[line.index('=') + 2:])
            for i in range(len(floatingZeroMasks)):
                key = int(line[4:line.index(']')])
                key &= floatingZeroMasks[i]
                key |= floatingOneMasks[i]
                memory2[key] = value

    inputFile.close()

    values = 0
    for value in memory.values():
        values += value

    print(f"All memory values for part 1 add up to {values}.")

    values = 0
    for value in memory2.values():
        values += value

    print(f"All memory values for part 2 add up to {values}.")


if __name__ == '__main__':
    main()
