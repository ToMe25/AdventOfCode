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
