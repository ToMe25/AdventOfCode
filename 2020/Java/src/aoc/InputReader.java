package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;

public abstract class InputReader {

	/**
	 * Gets the input file to read for the given day.
	 * 
	 * @param day the day for which to get the input file.
	 * @return the file to read for this days puzzle.
	 */
	public static File getInputFile(int day) {
		File input = new File("input").getAbsoluteFile();
		if (!input.exists() || !input.isFile()) {
			System.out.println(input.getPath() + " does not exist!");
			input = new File(input.getPath() + ".txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || !input.isFile()) {
			System.out.println(input.getPath() + " does not exist!");
			input = new File(input.getParentFile().getParent(), "input");
			input = new File(input, "Day" + day + ".txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || !input.isFile()) {
			System.err.println("None of the expected inputs exist!");
			System.exit(1);
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		return input;
	}

	/**
	 * Reads all the lines from the input file as a list of strings.
	 * 
	 * @param day the day for which to read the input file.
	 * @return an {@link ArrayList} containing a string for each line of the input
	 *         file.
	 * @throws IOException if an I/O error occurs reading from the file or a
	 *                     malformed or unmappable byte sequence is read.
	 */
	public static List<String> readInputFileLines(int day) throws IOException {
		List<String> lines = new ArrayList<>();
		Files.readAllLines(getInputFile(day).toPath()).forEach(line -> lines.add(line.trim()));
		return lines;
	}

	/**
	 * Reads all the lines from the input file as strings and parses them to longs.
	 * 
	 * @param day the day for which to read the input file.
	 * @return an {@link ArrayList} containing a long for each line of the input
	 *         file.
	 * @throws IOException if an I/O error occurs reading from the file or a
	 *                     malformed or unmappable byte sequence is read.
	 */
	public static List<Long> readInputFileNumbers(int day) throws IOException {
		List<String> lines = readInputFileLines(day);
		List<Long> numbers = new ArrayList<>();
		lines.forEach(line -> numbers.add(Long.parseLong(line)));
		return numbers;
	}

}
