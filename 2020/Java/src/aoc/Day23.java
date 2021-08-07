package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

public class Day23 {

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
			input = new File(input, "Day23.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		String line = Files.readAllLines(input.toPath()).get(0);
		byte[] cups = new byte[line.length()];
		byte[] cups_initial = new byte[line.length()];
		byte min_cup = -1;
		byte max_cup = -1;
		for (int i = 0; i < cups.length; i++) {
			cups[i] = cups_initial[i] = (byte) Integer.parseInt(new String(new char[] { line.charAt(i) }));
			if (cups[i] < min_cup || min_cup == -1) {
				min_cup = cups[i];
			}
			if (cups[i] > max_cup || max_cup == -1) {
				max_cup = cups[i];
			}
		}

		byte[] picked = new byte[3];
		for (int i = 0; i < 100; i++) {
			for (int j = 0; j < picked.length; j++) {
				picked[j] = cups[(i + j + 1) % cups.length];
			}

			byte target = (byte) (cups[i % cups.length] - 1);
			if (target < min_cup) {
				target = max_cup;
			}

			int destination = findInArray(cups, target);
			while (findInArray(picked, target) != -1) {
				target--;
				if (target < min_cup) {
					target = max_cup;
				}
				destination = findInArray(cups, target);
			}

			if (destination > i % cups.length) {
				for (int k = i % cups.length + 1; k < destination - picked.length + 1; k++) {
					cups[k % cups.length] = cups[(k + picked.length) % cups.length];
				}
			} else {
				for (int k = i % cups.length + 1; k < cups.length + destination - picked.length + 1; k++) {
					cups[k % cups.length] = cups[(k + picked.length) % cups.length];
					cups[(k + picked.length) % cups.length] = -1;
				}
			}

			destination = findInArray(cups, target);
			for (int k = 0; k < picked.length; k++) {
				cups[(destination + k + 1) % cups.length] = picked[k];
			}
		}

		String result = "";
		int start = findInArray(cups, (byte) 1);
		for (int i = 1; i < cups.length; i++) {
			result += cups[(i + start) % cups.length];
		}
		System.out.println("The part 1 result is " + result + ".");

		Cup[] cups2 = new Cup[1000000];
		Cup startCup = new Cup(cups_initial[0]);
		cups2[cups_initial[0] - 1] = startCup;
		Cup current = startCup;
		Cup previous = startCup;

		for (int i = 2; i <= cups2.length; i++) {
			if (i <= cups_initial.length) {
				current = new Cup(cups_initial[i - 1]);
			} else {
				current = new Cup(i);
			}

			previous.next = current;
			previous = current;
			cups2[current.label - 1] = current;
		}

		current.next = startCup;

		current = startCup;
		Cup[] picked2 = new Cup[3];
		for (int i = 0; i < 10000000; i++) {
			picked2[0] = current;
			for (int j = 0; j < picked2.length; j++) {
				picked2[j] = picked2[Math.max(j - 1, 0)].next;
			}

			Cup target = null;
			if (current.label > 1) {
				target = cups2[(current.label - 2)];
			} else {
				target = cups2[cups2.length - 1];
			}

			while (findInArray(picked2, target) != -1) {
				if (target.label > 1) {
					target = cups2[target.label - 2];
				} else {
					target = cups2[cups2.length - 1];
				}
			}

			current.next = picked2[picked2.length - 1].next;
			Cup oldNext = target.next;
			target.next = picked2[0];
			picked2[picked2.length - 1].next = oldNext;

			current = current.next;
		}

		long result2 = (long) cups2[0].next.label * cups2[0].next.next.label;
		System.out.println("The part 2 result is " + result2 + ".");
	}

	private static int findInArray(byte[] array, byte target) {
		for (int i = 0; i < array.length; i++) {
			if (array[i] == target) {
				return i;
			}
		}
		return -1;
	}

	private static <T> int findInArray(T[] array, T target) {
		for (int i = 0; i < array.length; i++) {
			if (array[i] == target) {
				return i;
			}
		}
		return -1;
	}

	private static class Cup {
		public int label;
		public Cup next;

		public Cup(int lable) {
			this.label = lable;
		}
	}
}
