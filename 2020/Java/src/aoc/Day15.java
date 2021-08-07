package aoc;

import java.io.IOException;

public abstract class Day15 {

	public static void main(String[] args) throws IOException {
		int lastNumber = 0;
		int[] lastNumbers = new int[30000000];
		int nr = 0;
		for (String number : InputReader.readInputFileLines(15).get(0).split(",")) {
			lastNumber = Integer.parseInt(number);
			nr++;
			lastNumbers[lastNumber] = nr;
		}

		while (nr < 30000000) {
			int num = 0;
			if (lastNumbers[lastNumber] != 0)
				num = nr - lastNumbers[lastNumber];

			lastNumbers[lastNumber] = nr;
			lastNumber = num;
			nr++;

			if (nr == 2020) {
				System.out.println("The 2020th number is " + lastNumber + ".");
			}
		}

		System.out.println("The 30000000th number is " + lastNumber + ".");
	}
}
