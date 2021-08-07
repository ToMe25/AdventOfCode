package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.HashSet;
import java.util.Set;

public class Day6 {

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
			input = new File(input, "Day6.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		int globalGroupCount = 0;
		int globalCount = 0;
		Set<Byte> groupFoundAny = new HashSet<>();
		Set<Byte> groupFoundEvery = new HashSet<>();
		Set<Byte> found = new HashSet<>();
		boolean newline = false;
		boolean first = true;
		for (byte b : Files.readAllBytes(input.toPath())) {
			if (b == '\n') {
				if (newline) {
					globalCount += groupFoundAny.size();
					globalGroupCount += groupFoundEvery.size();
					groupFoundAny.clear();
					groupFoundEvery.clear();
					first = true;
				} else {
					groupFoundAny.addAll(found);
					if (first) {
						groupFoundEvery.addAll(found);
						first = false;
					} else {
						groupFoundEvery.retainAll(found);
					}
					found.clear();
					newline = true;
				}
			} else if (b == '\r') {
			} else {
				newline = false;
				found.add(b);
			}
		}

		globalCount += groupFoundAny.size();
		globalGroupCount += groupFoundEvery.size();

		System.out.printf("The sum of all group part 1 answers is %d.%n", globalCount);
		System.out.printf("The sum of all group part 2 answers is %d.%n", globalGroupCount);
	}
}
