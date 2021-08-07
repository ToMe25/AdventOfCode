package aoc;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public abstract class Day5 {

	public static void main(String[] args) throws IOException {
		List<String> lines = InputReader.readInputFileLines(5);
		int[] found = new int[lines.size()];
		for (int i = 0; i < found.length; i++) {
			String line = lines.get(i).replaceAll("[FL]", "0").replaceAll("[RB]", "1");
			found[i] = Integer.parseInt(line, 2);
		}

		Arrays.sort(found);
		System.out.println("Highest found seat id is " + found[found.length - 1] + ".");

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
