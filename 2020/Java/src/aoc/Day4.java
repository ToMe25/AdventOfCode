package aoc;

import java.io.IOException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public abstract class Day4 {

	public static void main(String[] args) throws IOException {

		final String[] requiredFields = new String[] { "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" };
		final String[] validEyeColors = new String[] { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" };
		int validPassports1 = 0;
		int validPassports2 = 0;
		Map<String, String> fields = new HashMap<>();
		for (String line : InputReader.readInputFileLines(4)) {
			if (line.isEmpty()) {
				boolean invalid1 = false;
				boolean invalid2 = false;
				for (String field : requiredFields) {
					if (fields.containsKey(field)) {
						switch (field) {
						case "byr":
							int byr = Integer.parseInt(fields.get(field));
							if (byr < 1920 || byr > 2002)
								invalid2 = true;
							break;
						case "iyr":
							int iyr = Integer.parseInt(fields.get(field));
							if (iyr < 2010 || iyr > 2020)
								invalid2 = true;
							break;
						case "eyr":
							int eyr = Integer.parseInt(fields.get(field));
							if (eyr < 2020 || eyr > 2030)
								invalid2 = true;
							break;
						case "hgt":
							String hgt = fields.get(field);
							if (hgt.length() < 4) {
								invalid2 = true;
								break;
							}
							String unit = hgt.substring(hgt.length() - 2, hgt.length());
							int value = Integer.parseInt(hgt.substring(0, hgt.length() - 2));
							if (unit.equals("cm")) {
								if (value < 150 || value > 193)
									invalid2 = true;
							} else if (unit.equals("in")) {
								if (value < 59 || value > 76)
									invalid2 = true;
							}
							break;
						case "hcl":
							String hcl = fields.get(field);
							if (hcl.charAt(0) != '#' || hcl.length() != 7) {
								invalid2 = true;
								break;
							}
							try {
								Integer.parseInt(hcl.substring(1), 16);
							} catch (NumberFormatException e) {
								invalid2 = true;
							}
							break;
						case "ecl":
							String ecl = fields.get(field);
							if (!Arrays.stream(validEyeColors).anyMatch(ecl::equals))
								invalid2 = true;
							break;
						case "pid":
							String pid = fields.get(field);
							if (pid.length() != 9) {
								invalid2 = true;
								break;
							}
							try {
								Integer.parseInt(pid);
							} catch (NumberFormatException e) {
								invalid2 = true;
							}
							break;
						}
					} else {
						invalid1 = invalid2 = true;
					}
				}

				if (!invalid1) {
					validPassports1++;
				}
				if (!invalid2) {
					validPassports2++;
				}
				fields.clear();
			} else {
				for (String field : line.split(" ")) {
					fields.put(field.substring(0, field.indexOf(':')), field.substring(field.indexOf(':') + 1));
				}
			}
		}

		System.out.printf("Step 1 found %d valid passports.%n", validPassports1);
		System.out.printf("Step 2 found %d valid passports.%n", validPassports2);
	}

}
