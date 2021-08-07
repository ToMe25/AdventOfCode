package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

public class Day15 {

	public static void main(String[] args) throws IOException {
		File input = new File("input").getAbsoluteFile();
		if (!input.exists() || input.isDirectory()) {
			System.out.println(input.getPath() + " does not exist!");
			input = new File(input.getPath() + ".txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.out.println(input.getPath() + " does not exist!");
			input = new File(input.getParentFile().getParent(), "input");
			input = new File(input, "Day15.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		int lastNumber = 0;
		int[] lastNumbers = new int[30000000];
		int nr = 0;
		for (String number : Files.readAllLines(input.toPath()).get(0).split(",")) {
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
