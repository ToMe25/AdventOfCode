package aoc;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public abstract class Day10 {

	public static void main(String[] args) throws IOException {
		List<Long> adapters = InputReader.readInputFileNumbers(10);
		Collections.sort(adapters);

		int one = 0;
		int three = 1;
		long possibleSolutions = 1;
		List<Long> futureSolutions = new ArrayList<>();
		for (int i = 0; i < 3; i++) {
			futureSolutions.add((long) 0);
		}

		long joltage = 0;
		for (long adapter : adapters) {
			long lowest = adapter - joltage;
			if (adapters.contains(joltage + 2) && lowest < 2) {
				futureSolutions.set(1, futureSolutions.get(1) + possibleSolutions);
			}
			if (adapters.contains(joltage + 3) && lowest < 3) {
				futureSolutions.set(2, futureSolutions.get(2) + possibleSolutions);
			}
			possibleSolutions += futureSolutions.get(0);
			futureSolutions.remove(0);
			futureSolutions.add((long) 0);
			if (adapter - joltage == 1) {
				one++;
			} else if (adapter - joltage == 3) {
				three++;
			}
			joltage = adapter;
		}

		System.out.printf(
				"Found %d one jolt differences and %d three jolt differences, getting a step one result of %d.%n", one,
				three, one * three);

		System.out.println("Found " + possibleSolutions + " valid solutions.");
	}
}
