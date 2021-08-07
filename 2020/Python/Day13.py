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
        inf = os.path.join(inf, "Day13.txt")
        print(f"trying {inf} instead.")

    if not os.path.exists(inf) or os.path.isdir(inf):
        print("None of the expected inputs exist!", file=sys.stderr)
        return
    else:
        print(f"Using input file {inf}.")

    inputFile = open(inf, 'r')
    departing = 0
    busIds = []
    busses = {}
    fastest = 0;
    for line in inputFile.readlines():
        if departing == 0:
            departing = int(line[:-1])
        else:
            for bus in line.split(','):
                if bus != "x":
                    b = int(bus)
                    busses[b] = b - departing % b
                    if fastest == 0 or busses[b] < busses[fastest]:
                        fastest = b
                    busIds.append(b)

                else:
                    busIds.append(0)

    print(f"Part one result is {fastest * busses[fastest]}.")

    time = 0
    step = 1
    current = -1
    while(True):
        bus = 0
        for i in range(current + 1, len(busIds)):
            bus = busIds[i]
            if bus == 0:
                continue
            elif (time + i) % bus != 0:
                bus = 0
                break;
            else:
                current = i
                step *= bus

        if bus == busIds[-1]:
            break
        else:
            time += step

    print(f"Part two timestamp is {time}.")


if __name__ == '__main__':
    main()
