import InputReader


def main():
    inputs = InputReader.readInputFileNumbers(1)
    double = False
    triple = False
    for nr in inputs:
        if not double and 2020 - nr in inputs:
            print("Found matching two number pair!")
            print(f"The numbers are {nr} and {2020 - nr}, and the result is {nr * (2020 - nr)}.")
            double = True

        if not triple:
            for i in inputs:
                if 2020 - nr - i in inputs:
                    print("Found matching three number pair!")
                    print(f"The numbers are {nr}, {i}, and {2020 - nr - i}, and the result is {nr * i * (2020 - nr - i)}.")
                    triple = True
                    break

        if double and triple:
            return

    print("Could not find a matching number pair!")


if __name__ == '__main__':
    main()
