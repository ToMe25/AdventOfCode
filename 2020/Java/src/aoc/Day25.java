package aoc;

import java.io.IOException;
import java.util.List;

public abstract class Day25 {

	public static void main(String[] args) throws IOException {
		List<Long> keys = InputReader.readInputFileNumbers(25);
		final int subjectNumber = 7;

		final long encryptionKey = getKey(keys.get(1), getLoopSize(keys.get(0), subjectNumber));
		System.out.printf("The encryption key is %d.", encryptionKey);
	}

	public static long getKey(long subjectNumber, int loopSize) {
		long value = 1;
		for (int i = 0; i < loopSize; i++) {
			value = value * subjectNumber % 20201227;
		}
		return value;
	}

	public static int getLoopSize(long key, int subjectNumber) {
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
