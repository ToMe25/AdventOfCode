import InputReader


def main():
    inputs = InputReader.readInputFileNumbers(1)
    double = False
    triple = False
    for nr in inputs:
        if not double and 2020 - nr in inputs:
            print("Found matching two number pair!")
            print("The numbers are %d and %d, and the result is %d." % (nr, 2020 - nr, nr * (2020 - nr)))
            double = True

        if not triple:
            for i in inputs:
                if 2020 - nr - i in inputs:
                    print("Found matching three number pair!")
                    print("The numbers are %d, %d, and %d, and the result is %d." % (nr, i, 2020 - nr -i, nr * i * (2020 - nr - i)))
                    triple = True
                    break

        if double and triple:
            return

    print("Could not find a matching number pair!")


if __name__ == '__main__':
    main()
