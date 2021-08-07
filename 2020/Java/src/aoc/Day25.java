package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.List;

public class Day25 {

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
			input = new File(input, "Day25.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		List<String> lines = Files.readAllLines(input.toPath());
		final int subjectNumber = 7;
		final int key1 = Integer.parseInt(lines.get(0));
		final int key2 = Integer.parseInt(lines.get(1));

		final long encryptionKey = getKey(key2, getLoopSize(key1, subjectNumber));
		System.out.printf("The encryption key is %d.", encryptionKey);
	}

	public static long getKey(int subjectNumber, int loopSize) {
		long value = 1;
		for (int i = 0; i < loopSize; i++) {
			value = value * subjectNumber % 20201227;
		}
		return value;
	}

	public static int getLoopSize(int key, int subjectNumber) {
		int loopSize = 0;
		while (key != 1) {
			double newKey = key / (double) subjectNumber;
			while (newKey % 1 != 0) {
				key += 20201227;
				newKey = key / (double) subjectNumber;
			}
			key = (int) newKey;
			loopSize++;
		}
		return loopSize;
	}
}
