package aoc;

import java.io.IOException;
import java.util.List;

public abstract class Day1 {

	public static void main(String[] args) throws IOException {
		List<Long> inputs = InputReader.readInputFileNumbers(1);
		boolean Double = false;
		boolean Triple = false;
		for (long nr : inputs) {
			if (!Double && inputs.contains(2020 - nr)) {
				System.out.println("Found matching two number pair!");
				System.out.printf("The numbers are %d and %d, and the result is %d.%n", nr, 2020 - nr,
						nr * (2020 - nr));
				Double = true;
			}

			if (!Triple) {
				for (long i : inputs) {
					if (inputs.contains(2020 - nr - i)) {
						System.out.println("Found matching three number pair!");
						System.out.printf("The numbers are %d, %d, and %d, and the result is %d.%n", nr, i,
								2020 - nr - i, nr * i * (2020 - nr - i));
						Triple = true;
						break;
					}
				}
			}

			if (Double && Triple) {
				return;
			}
		}

		System.err.println("Could not find a matching number pair!");
	}

}
