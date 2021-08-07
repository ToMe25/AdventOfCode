package aoc;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public abstract class Day19 {

	public static void main(String[] args) throws IOException {
		List<String> lines = InputReader.readInputFileLines(19);
		String[] rules = new String[lines.size() / 2];
		Map<Integer, String> ruleReplacements = new HashMap<>();
		ruleReplacements.put(8, "( 42 )+");
		ruleReplacements.put(11, "( 42 ){n}( 31 ){n}");
		String pattern = null;
		String pattern2 = null;
		int matches = 0;
		int matches2 = 0;
		for (String line : lines) {
			if (line.contains(":")) {
				String value = line.substring(line.indexOf(':') + 1) + ' ';
				if (value.contains("|")) {
					value = '(' + value + ')';
				}

				if (value.contains("\"")) {
					value = value.replace("\"", "");
				}

				rules[Integer.parseInt(line.substring(0, line.indexOf(':')))] = value;
			} else if (pattern == null) {
				pattern = pattern2 = rules[0];
				while (true) {
					int toReplace = 0;
					for (String nr : pattern.split(" ")) {
						if (nr.matches("\\d+")) {
							toReplace = Integer.parseInt(nr);
							break;
						}
					}

					if (toReplace == 0)
						break;

					pattern = pattern.replaceAll(" " + toReplace + " ", " " + rules[toReplace] + " ");
					if (ruleReplacements.containsKey(toReplace)) {
						pattern2 = pattern2.replaceAll(" " + toReplace + " ",
								" " + ruleReplacements.get(toReplace) + " ");
					} else {
						pattern2 = pattern2.replaceAll(" " + toReplace + " ", " " + rules[toReplace] + " ");
					}

					for (int i = 0; i < rules.length; i++) {
						if (rules[i] != null && rules[i] != "") {
							rules[i] = rules[i].replaceAll(" " + toReplace + " ", " " + rules[toReplace] + " ");
						}
					}

					for (int i : ruleReplacements.keySet()) {
						ruleReplacements.put(i, ruleReplacements.get(i).replaceAll(" " + toReplace + " ",
								" " + rules[toReplace] + " "));
					}
				}

				pattern = pattern.replace(" ", "");
				pattern2 = pattern2.replace(" ", "");
			} else {
				if (line.isEmpty())
					continue;

				if (line.matches(pattern))
					matches++;

				for (int i = 1; i < 10; i++) {
					if (line.matches(pattern2.replace('n', String.valueOf(i).charAt(0))))
						matches2++;
				}
			}
		}

		System.out.println("Found " + matches + " valid part 1 messages.");
		System.out.println("Found " + matches2 + " valid part 2 messages.");
	}
}
