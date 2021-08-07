package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

public class Day2 {

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
			input = new File(input, "Day2.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		int correctPwds1 = 0;
		int correctPwds2 = 0;
		for (String line : Files.readAllLines(input.toPath())) {
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
