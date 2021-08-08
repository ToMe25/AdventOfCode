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

	/**
	 * Generates the encryption key from the subject number and the loop size.
	 * 
	 * <p>
	 * The encryption key starts as 1.<br>
	 * Then the following actions get executed loop size times:
	 * <ul>
	 * <li>multiply the previous value by the subject number</li>
	 * <li>set the encryption key to the remainder from dividing the previous result
	 * by 20201227</li>
	 * </ul>
	 * 
	 * @param subjectNumber the subject number to use in the calculation.
	 * @param loopSize      how many times to execute the calculation.
	 * @return the resulting encryption key.
	 */
	public static long getKey(long subjectNumber, int loopSize) {
		long value = 1;
		for (int i = 0; i < loopSize; i++) {
			value = value * subjectNumber % 20201227;
		}
		return value;
	}

	/**
	 * Reverse engineers the loop size from the given key and subject number.
	 * 
	 * <p>
	 * This is done by continually dividing the key by the subject number until the
	 * result matches 1.<br>
	 * If the result has decimal digits 20201227 gets added to the previous key,
	 * before it is divided again.<br>
	 * By counting the divisions the loop size can be determined.
	 * 
	 * @param key           the key to get the loop size for.
	 * @param subjectNumber the subject number used to generate the key.
	 * @return the loop size used to generate the given key.
	 */
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
