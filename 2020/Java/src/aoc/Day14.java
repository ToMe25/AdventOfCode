package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day14 {

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
			input = new File(input, "Day14.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		long zeroMask = 0;
		long oneMask = 0;
		Map<Integer, Long> memory = new HashMap<>();
		List<Long> floatingZeroMasks = new ArrayList<>();
		List<Long> floatingOneMasks = new ArrayList<>();
		Map<Long, Long> memory2 = new HashMap<>();
		for (String line : Files.readAllLines(input.toPath())) {
			if (line.startsWith("mask")) {
				zeroMask = Long.parseUnsignedLong(line.substring(7).replace('X', '1'), 2);
				oneMask = Long.parseUnsignedLong(line.substring(7).replace('X', '0'), 2);

				floatingZeroMasks.clear();
				floatingOneMasks.clear();
				List<String> masksf = new ArrayList<>();
				masksf.add("");
				for (char c : line.substring(7).toCharArray()) {
					if (c == 'X') {
						int size = masksf.size();
						for (int i = 0; i < size; i++) {
							masksf.add(masksf.get(i) + "0");
							masksf.set(i, masksf.get(i) + "1");
						}
					} else if (c == '1') {
						for (int i = 0; i < masksf.size(); i++) {
							masksf.set(i, masksf.get(i) + "1");
						}
					} else if (c == '0') {
						for (int i = 0; i < masksf.size(); i++) {
							masksf.set(i, masksf.get(i) + "X");
						}
					}
				}

				for (String mask : masksf) {
					floatingZeroMasks.add(Long.parseUnsignedLong(mask.replace('X', '1'), 2));
					floatingOneMasks.add(Long.parseUnsignedLong(mask.replace('X', '0'), 2));
				}
			} else if (line.startsWith("mem")) {
				long key = Integer.parseInt(line.substring(4, line.indexOf(']')));
				long value = Long.parseLong(line.substring(line.indexOf('=') + 2));
				value &= zeroMask;
				value |= oneMask;
				memory.put((int) key, value);

				value = Long.parseLong(line.substring(line.indexOf('=') + 2));
				for (int i = 0; i < floatingZeroMasks.size(); i++) {
					key = Long.parseLong(line.substring(4, line.indexOf(']')));
					key &= floatingZeroMasks.get(i);
					key |= floatingOneMasks.get(i);
					memory2.put(key, value);
				}
			}
		}

		long values = 0;
		for (long value : memory.values()) {
			values += value;
		}

		System.out.println("All memory values for part 1 add up to " + values + ".");

		values = 0;
		for (long value : memory2.values()) {
			values += value;
		}

		System.out.println("All memory values for part 2 add up to " + values + ".");
	}
}
