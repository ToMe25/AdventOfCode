package aoc;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public abstract class Day7 {

	public static void main(String[] args) throws IOException {
		Map<String, Map<String, Integer>> bags = new HashMap<>();
		for (String line : InputReader.readInputFileLines(7)) {
			Map<String, Integer> contained = new HashMap<>();
			for (String bag : line.substring(line.indexOf("tain ") + 5).split(", ")) {
				if (bag.equals("no other bags."))
					break;
				contained.put(bag.substring(2, bag.indexOf(" bag")), Character.getNumericValue(bag.charAt(0)));
			}

			bags.put(line.substring(0, line.indexOf(" bag")), contained);
		}

		int goldContaining = 0;
		for (String bag : bags.keySet()) {
			if (checkBag(bags, bag, "shiny gold"))
				goldContaining++;
		}

		System.out.println("Found " + goldContaining + " bags that contain a \"shiny gold\" bag.");

		System.out.println("A single \"shiny gold\" bag contains " + (countBags(bags, "shiny gold") - 1) + " bags.");
	}

	private static boolean checkBag(Map<String, Map<String, Integer>> bags, String bag, String lookingFor) {
		for (String check : bags.get(bag).keySet()) {
			if (check.equals(lookingFor)) {
				return true;
			} else if (checkBag(bags, check, lookingFor)) {
				return true;
			}
		}
		return false;
	}

	private static int countBags(Map<String, Map<String, Integer>> bags, String bag) {
		int count = 1;
		for (String check : bags.get(bag).keySet()) {
			count += countBags(bags, check) * bags.get(bag).get(check);
		}
		return count;
	}
}
