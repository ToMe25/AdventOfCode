package aoc;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public abstract class Day8 {

	public static void main(String[] args) throws IOException {
		List<String> instructions = InputReader.readInputFileLines(8);
		int[] result = checkCode(new ArrayList<>(instructions), new HashSet<>(), 0, 0, false);
		System.out.println(
				"The acc value was " + result[0] + " just before executing the first instruction a second time.");

		result = checkCode(new ArrayList<>(instructions), new HashSet<>(), 0, 0, true);
		if (result[1] == 1)
			System.out.println("The acc value was " + result[0] + " after executing everything.");
		else
			System.out.println("Failed to fully execute the code.");
	}

	private static int[] checkCode(List<String> instructions, Set<Integer> executed, int current, int acc,
			boolean change) {
		while (current < instructions.size()) {
			String inst = instructions.get(current).substring(0, 3);
			int add = 1;
			if (inst.equals("acc"))
				acc += Integer.parseInt(instructions.get(current).substring(4, instructions.get(current).length()));
			else if (inst.equals("jmp")) {
				add = Integer.parseInt(instructions.get(current).substring(4, instructions.get(current).length()));
				if (change) {
					instructions.set(current, "nop" + instructions.get(current).substring(3));
					int[] result = checkCode(instructions, new HashSet<>(executed), current, acc, false);
					if (result[1] == 1)
						return result;
				}
			} else if (change) {
				instructions.set(current, "jmp" + instructions.get(current).substring(3));
				int[] result = checkCode(instructions, new HashSet<>(executed), current, acc, false);
				if (result[1] == 1)
					return result;
			}

			if (executed.contains(current + add)) {
				return new int[] { acc, 0 };
			} else {
				executed.add(current);
				current += add;
			}
		}
		return new int[] { acc, 1 };
	}
}
