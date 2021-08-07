package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Set;
import java.util.function.Predicate;

public class Day16 {

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
			input = new File(input, "Day16.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		int[] ownTicket = null;
		Set<int[]> tickets = new HashSet<>();
		Map<String, Predicate<Integer>> inputValidators = new HashMap<>();
		boolean your = false;
		for (String line : Files.readAllLines(input.toPath())) {
			if (line.isEmpty())
				continue;

			if (line.startsWith("your")) {
				your = true;
				continue;
			} else if (line.startsWith("nearby")) {
				your = false;
				continue;
			}

			if (!your && ownTicket == null) {
				int[] range = Arrays.stream(line.substring(line.indexOf(':') + 2).split("-| or "))
						.mapToInt(Integer::parseInt).toArray();
				inputValidators.put(line.substring(0, line.indexOf(':')),
						in -> (in >= range[0] && in <= range[1]) || (in >= range[2] && in <= range[3]));
			} else if (your) {
				ownTicket = Arrays.stream(line.split(",")).mapToInt(Integer::parseInt).toArray();
			} else {
				tickets.add(Arrays.stream(line.split(",")).mapToInt(Integer::parseInt).toArray());
			}
		}

		int invalid = 0;
		HashSet<int[]> invalidTickets = new HashSet<int[]>();
		for (int[] ticket : tickets) {
			for (int field : ticket) {
				if (inputValidators.values().stream().noneMatch(validator -> validator.test(field))) {
					invalid += field;
					invalidTickets.add(ticket);
				}
			}
		}

		System.out.println("Found an error rate of " + invalid + ".");

		for (int[] ticket : invalidTickets) {
			tickets.remove(ticket);
		}

		Map<String, Set<Integer>> possiblePositions = new HashMap<>();
		for (Entry<String, Predicate<Integer>> field : inputValidators.entrySet()) {
			for (int i = 0; i < ownTicket.length; i++) {
				final int pos = i;
				if (!field.getValue().test(ownTicket[i]))
					continue;
				else if (!tickets.stream().allMatch(t -> field.getValue().test(t[pos])))
					continue;
				else {
					Set<Integer> positions = new HashSet<>();
					if (possiblePositions.containsKey(field.getKey()))
						positions = possiblePositions.get(field.getKey());

					positions.add(i);
					possiblePositions.put(field.getKey(), positions);
				}
			}
		}

		String[] fields = new String[ownTicket.length];
		while (Arrays.stream(fields).anyMatch(f -> f == null)) {
			for (int i = 0; i < ownTicket.length; i++) {
				final int pos = i;
				if (possiblePositions.values().stream().filter(v -> v.contains(pos)).count() == 1) {
					fields[i] = possiblePositions.keySet().stream().filter(f -> possiblePositions.get(f).contains(pos))
							.findAny().get();
					possiblePositions.forEach((k, v) -> v.remove(pos));
				}
			}

			possiblePositions.keySet().stream().filter(f -> possiblePositions.get(f).size() == 1).forEach(field -> {
				final int pos = possiblePositions.get(field).iterator().next();
				fields[pos] = field;
				possiblePositions.forEach((k, v) -> v.remove(pos));
			});
		}

		long result = 1;
		for (int i = 0; i < ownTicket.length; i++) {
			if (fields[i].startsWith("departure")) {
				result *= ownTicket[i];
			}
		}

		System.out.println("The part 2 result is " + result + ".");
	}
}
