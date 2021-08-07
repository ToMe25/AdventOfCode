package aoc;

import java.io.IOException;

public abstract class Day2 {

	public static void main(String[] args) throws IOException {
		int correctPwds1 = 0;
		int correctPwds2 = 0;
		for (String line : InputReader.readInputFileLines(2)) {
			String[] inputs = line.split(" ");
			String[] range = inputs[0].split("-");
			int min = Integer.parseInt(range[0]);
			int max = Integer.parseInt(range[1]);
			char requested = inputs[1].charAt(0);
			int occurrences = (int) inputs[2].chars().filter(c -> c == requested).count();
			if (occurrences >= min && occurrences <= max) {
				correctPwds1++;
			}

			if ((inputs[2].charAt(min - 1) == requested) != (inputs[2].charAt(max - 1) == requested)) {
				correctPwds2++;
			}
		}

		System.out.println("Found " + correctPwds1 + " correct step 1 passwords.");
		System.out.println("Found " + correctPwds2 + " correct step 2 passwords.");
	}

}
