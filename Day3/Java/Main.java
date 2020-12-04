import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.List;

public class Main {

	public static void main(String[] args) throws IOException {
		File input = new File("input").getAbsoluteFile();
		if (!input.exists()) {
			System.out.println(input.getPath() + " does not exist!");
			System.out.println("trying " + input.getPath() + ".txt instead.");
			input = new File(input.getPath() + ".txt");
			if (!input.exists()) {
				System.err.println("neither " + input.getPath().substring(0, input.getPath().length() - 4) + ",");
				System.err.println("nor " + input.getPath() + " exists!");
				return;
			}
		}

		boolean[][] map = readInput(input);
		System.out.println("Part one hit " + checkSlope(map, 3, 1) + " trees.");

		final byte[][] slopes = { { 1, 1 }, { 3, 1 }, { 5, 1 }, { 7, 1 }, { 1, 2 } };
		long result = 1;
		for (byte[] slope : slopes) {
			int hits = checkSlope(map, slope[0], slope[1]);
			System.out.printf("Slope Right %d, Down %d hit %d trees.%n", slope[0], slope[1], hits);
			result *= hits;
		}

		System.out.println("Part two result is " + result + ".");
	}

	private static int checkSlope(boolean[][] trees, int right, int down) {
		int treesHit = 0;
		int x = 0;
		for (int y = down; y < trees.length; y += down) {
			x = (x + right) % trees[y].length;
			if (trees[y][x]) {
				treesHit++;
			}
		}

		return treesHit;
	}

	private static boolean[][] readInput(File input) throws IOException {
		List<String> lines = Files.readAllLines(input.toPath());
		boolean[][] trees = new boolean[lines.size()][lines.get(1).length()];
		for (int y = 0; y < trees.length; y++) {
			String line = lines.get(y);
			for (int x = 0; x < line.length(); x++) {
				trees[y][x] = line.charAt(x) == '#';
			}
		}

		return trees;
	}
}
