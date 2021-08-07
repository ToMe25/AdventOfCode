package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day5 {

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
			input = new File(input, "Day5.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		List<Integer> found = new ArrayList<>();
		for (String line : Files.readAllLines(input.toPath())) {
			line = line.replaceAll("[FL]", "0").replaceAll("[RB]", "1");

			found.add(Integer.parseInt(line, 2));
		}

		Collections.sort(found);
		System.out.println("Highest found seat id is " + found.get(found.size() - 1) + ".");

		int last = 0;
		for (int seat : found) {
			if (seat == last + 2) {
				last = seat - 1;
				break;
			}
			last = seat;
		}

		System.out.println("Your seat has id " + last + ".");
	}
}
