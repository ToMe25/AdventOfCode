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

		final int cycles = 6;
		List<String> lines = Files.readAllLines(input.toPath());
		boolean[][][] dimension1 = new boolean[cycles * 2 + 1][cycles * 2 + lines.size()][cycles * 2
				+ lines.get(0).length()];
		boolean[][][] dimension2 = new boolean[cycles * 2 + 1][cycles * 2 + lines.size()][cycles * 2
				+ lines.get(0).length()];
		boolean[][][][] dimension3 = new boolean[cycles * 2 + 1][cycles * 2 + 1][cycles * 2 + lines.size()][cycles * 2
				+ lines.get(0).length()];
		boolean[][][][] dimension4 = new boolean[cycles * 2 + 1][cycles * 2 + 1][cycles * 2 + lines.size()][cycles * 2
				+ lines.get(0).length()];
		for (int y = 0; y < lines.size(); y++) {
			String line = lines.get(y);
			for (int x = 0; x < line.length(); x++) {
				dimension1[cycles][cycles + y][cycles + x] = line.charAt(x) == '#';
				dimension2[cycles][cycles + y][cycles + x] = line.charAt(x) == '#';
				dimension3[cycles][cycles][cycles + y][cycles + x] = line.charAt(x) == '#';
				dimension4[cycles][cycles][cycles + y][cycles + x] = line.charAt(x) == '#';
			}
		}

		for (int i = 0; i < cycles; i++) {
			if (i % 2 == 0) {
				simulate3D(dimension1, dimension2);
			} else {
				simulate3D(dimension2, dimension1);
			}
		}

		int active = 0;
		for (int z = 0; z < dimension1.length; z++) {
			for (int y = 0; y < dimension1[z].length; y++) {
				for (int x = 0; x < dimension1[z][y].length; x++) {
					if (dimension2[z][y][x]) {
						active++;
					}
				}
			}
		}

		System.out.println("Found " + active + " active cubes in 3D space.");

		for (int i = 0; i < cycles; i++) {
			if (i % 2 == 0) {
				simulate4D(dimension3, dimension4);
			} else {
				simulate4D(dimension4, dimension3);
			}
		}

		active = 0;
		for (int w = 0; w < dimension3.length; w++) {
			for (int z = 0; z < dimension3[w].length; z++) {
				for (int y = 0; y < dimension3[w][z].length; y++) {
					for (int x = 0; x < dimension3[w][z][y].length; x++) {
						if (dimension4[w][z][y][x]) {
							active++;
						}
					}
				}
			}
		}

		System.out.println("Found " + active + " active cubes in 4D space.");
	}

	private static void simulate3D(boolean[][][] current, boolean[][][] last) {
		for (int z = 0; z < current.length; z++) {
			for (int y = 0; y < current[z].length; y++) {
				for (int x = 0; x < current[z][y].length; x++) {
					current[z][y][x] = check3D(last, x, y, z);
				}
			}
		}
	}

	private static boolean check3D(boolean[][][] last, int x, int y, int z) {
		int activeNeighbors = 0;
		for (int i = -1; i < 2; i++) {
			for (int j = -1; j < 2; j++) {
				for (int k = -1; k < 2; k++) {
					if (z + i < 0 || z + i >= last.length || y + j < 0 || y + j >= last[z + i].length || x + k < 0
							|| x + k >= last[z + i][y + j].length) {
						continue;
					} else if (i == 0 && j == 0 && k == 0) {
						continue;
					} else if (last[z + i][y + j][x + k]) {
						activeNeighbors++;
						if (activeNeighbors > 3) {
							return false;
						}
					}
				}
			}
		}

		if (last[z][y][x]) {
			return activeNeighbors == 2 || activeNeighbors == 3;
		} else {
			return activeNeighbors == 3;
		}
	}

	private static void simulate4D(boolean[][][][] current, boolean[][][][] last) {
		for (int w = 0; w < current.length; w++) {
			for (int z = 0; z < current[w].length; z++) {
				for (int y = 0; y < current[w][z].length; y++) {
					for (int x = 0; x < current[w][z][y].length; x++) {
						current[w][z][y][x] = check4D(last, w, x, y, z);
					}
				}
			}
		}
	}

	private static boolean check4D(boolean[][][][] last, int w, int x, int y, int z) {
		int activeNeighbors = 0;
		for (int i = -1; i < 2; i++) {
			for (int j = -1; j < 2; j++) {
				for (int k = -1; k < 2; k++) {
					for (int l = -1; l < 2; l++) {
						if (w + i < 0 || w + i >= last.length || z + j < 0 || z + j >= last[w + i].length) {
							continue;
						} else if (y + k < 0 || y + k >= last[w + i][z + j].length || x + l < 0
								|| x + l >= last[w + i][z + j][y + k].length) {
							continue;
						} else if (i == 0 && j == 0 && k == 0 && l == 0) {
							continue;
						} else if (last[w + i][z + j][y + k][x + l]) {
							activeNeighbors++;
							if (activeNeighbors > 3) {
								return false;
							}
						}
					}
				}
			}
		}

		if (last[w][z][y][x]) {
			return activeNeighbors == 2 || activeNeighbors == 3;
		} else {
			return activeNeighbors == 3;
		}
	}
}
