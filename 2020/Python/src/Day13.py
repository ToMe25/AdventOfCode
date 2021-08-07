import InputReader


def main():
    departing = 0
    busIds = []
    busses = {}
    fastest = 0;
    for line in InputReader.readInputFileLines(13):
        if departing == 0:
            departing = int(line)
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
