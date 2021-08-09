package aoc;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public abstract class Day8 {

	public static void main(String[] args) throws IOException {
		List<String> instructions = InputReader.readInputFileLines(8);
		int[] result = runCode(new ArrayList<>(instructions), new HashSet<>(), 0, 0, false);
		System.out.println(
				"The acc value was " + result[0] + " just before executing the first instruction a second time.");

		result = runCode(new ArrayList<>(instructions), new HashSet<>(), 0, 0, true);
		if (result[1] == 1)
			System.out.println("The acc value was " + result[0] + " after executing everything.");
		else
			System.out.println("Failed to fully execute the code.");
	}

	/**
	 * Executes the given list of instructions.
	 * 
	 * <p>
	 * Each instruction has an integer value to be used for its function, this can
	 * be positive or negative.<br>
	 * Supported instructions:
	 * <ul>
	 * <li>acc: adds its integer value to the accumulator value</li>
	 * <li>jmp: jumps the amount of instructions defined by its integer value
	 * forward</li>
	 * <li>nop: does nothing
	 * </ul>
	 * Terminates when the first instruction would be executed a second time, or the
	 * instruction to execute is after the last instruction.
	 * </p>
	 * 
	 * @param instructions the list of instructions to execute.
	 * @param executed     a set containing the positions of instructions to be
	 *                     considered executed already.
	 * @param current      the position of the next instruction to execute.
	 * @param acc          the starting accumulator value.
	 * @param change       whether a single jmp or nop instruction should be changed
	 *                     to make the program terminate by trying to execute an
	 *                     instruction after the last one, rather then executing an
	 *                     earlier one twice.
	 * @return an integer array containing the current accumulator value, and an
	 *         exit code. This exit code is 1 if the program terminated by trying to
	 *         execute an instruction after the last one. If it tried to execute an
	 *         instruction twice the exit code is 0.
	 */
	private static int[] runCode(List<String> instructions, Set<Integer> executed, int current, int acc,
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
					int[] result = runCode(instructions, new HashSet<>(executed), current, acc, false);
					if (result[1] == 1)
						return result;
				}
			} else if (change) {
				instructions.set(current, "jmp" + instructions.get(current).substring(3));
				int[] result = runCode(instructions, new HashSet<>(executed), current, acc, false);
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
