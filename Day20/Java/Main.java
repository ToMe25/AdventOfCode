import java.awt.Point;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

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

		final List<String> lines = Files.readAllLines(input.toPath());
		Map<Integer, Tile> tiles = new HashMap<>();
		for (int i = 0; i < lines.size(); i += 12) {
			boolean[][] tile = new boolean[10][10];
			for (int j = 0; j < 10; j++) {
				String line = lines.get(i + j + 1);
				for (int k = 0; k < line.length(); k++) {
					tile[j][k] = line.charAt(k) == '#';
				}
			}
			tiles.put(Integer.parseInt(lines.get(i).substring(5, 9)), new Tile(tile));
		}

		int[][] grid = new int[(int) Math.ceil(Math.sqrt(tiles.size()))][(int) Math.ceil(Math.sqrt(tiles.size()))];
		for (int id : tiles.keySet()) {
			boolean put = false;
			while (!put) {
				boolean topMatches = false;
				boolean bottomMatches = false;
				boolean leftMatches = false;
				boolean rightMatches = false;
				for (int i : tiles.keySet()) {
					if (id == i)
						continue;

					int match = tiles.get(id).fitsMirrored(tiles.get(i));
					int side = match & 0x7;
					if (side == 1)
						topMatches = true;
					else if (side == 2)
						leftMatches = true;
					else if (side == 3)
						bottomMatches = true;
					else if (side == 4)
						rightMatches = true;

					if (topMatches && bottomMatches && leftMatches && rightMatches)
						break;
				}

				if (!topMatches && !leftMatches) {
					if (grid[0][0] == 0) {
						grid[0][0] = id;
						put = true;
					}
				} else if ((leftMatches || bottomMatches) && (bottomMatches || rightMatches)
						&& (rightMatches || topMatches)) {
					put = true;
				}

				if (!put)
					tiles.put(id, tiles.get(id).rotate());
			}

			if (grid[0][0] != 0)
				break;
		}

		for (int i = 0; i < grid.length * grid.length - 1; i++) {
			if (grid[(i + 1) / grid.length][(i + 1) % grid.length] != 0
					&& (i / grid.length == grid.length - 1 || grid[i / grid.length + 1][i % grid.length] != 0))
				continue;

			Tile last = tiles.get(grid[i / grid.length][i % grid.length]);
			for (int id : tiles.keySet()) {
				int match = last.fitsMirrored(tiles.get(id));
				int side = match & 0x7;
				int rotation = match >> 3 & 0x7;
				int mirror = match >> 6 & 0x7;
				if (side == 3)
					grid[i / grid.length + 1][i % grid.length] = id;
				else if (side == 4)
					grid[(i + 1) / grid.length][(i + 1) % grid.length] = id;
				else
					continue;

				Tile rotated = tiles.get(id);

				if (mirror == 1)
					rotated = rotated.mirror();

				for (int r = 0; r < rotation; r++) {
					rotated = rotated.rotate();
				}

				tiles.put(id, rotated);
			}
		}

		long part1sum = grid[0][0];
		part1sum *= grid[0][grid[0].length - 1];
		part1sum *= grid[grid.length - 1][0];
		part1sum *= grid[grid.length - 1][grid[grid.length - 1].length - 1];
		System.out.println("The sum of the ids from the four corners is " + part1sum + ".");

		boolean[][] image = new boolean[grid.length * 8][grid[0].length * 8];
		for (int i = 0; i < grid.length; i++) {
			for (int j = 0; j < grid[i].length; j++) {
				Tile tile = tiles.get(grid[i][j]);
				for (int k = 0; k < 8; k++) {
					for (int l = 0; l < 8; l++) {
						image[i * 8 + k][j * 8 + l] = tile.pixels[k + 1][l + 1];
					}
				}
			}
		}

		final String[] MONSTER = new String[] { "                  # ", "#    ##    ##    ###",
				" #  #  #  #  #  #   " };
		final boolean[][] monster = new boolean[MONSTER.length][MONSTER[0].length()];
		for (int i = 0; i < MONSTER.length; i++) {
			for (int j = 0; j < MONSTER[0].length(); j++) {
				monster[i][j] = MONSTER[i].charAt(j) == '#';
			}
		}

		int waves = countWaves(image, monster);

		if (waves == 0) {
			boolean[][] copy = new boolean[image.length][image[0].length];
			for (int i = 0; i < image.length; i++) {
				copy[image.length - 1 - i] = Arrays.copyOf(image[i], image[i].length);
			}
			image = copy;
			waves = countWaves(image, monster);
		}

		System.out.println("The roughness of the water is " + waves + ".");
	}

	private static int countWaves(boolean[][] image, boolean[][] monster) {
		int waves = 0;
		for (int r = 0; r < 4; r++) {
			boolean monsterFound = false;
			List<Point> monsters = new ArrayList<>();
			for (int y = 0; y < image.length; y++) {
				for (int x = 0; x < image[y].length; x++) {
					if (y < image.length - monster.length && x < image[y].length - monster[0].length) {
						boolean error = false;
						for (int i = 0; i < monster.length; i++) {
							for (int j = 0; j < monster[i].length; j++) {
								if (monster[i][j] && !image[y + i][x + j]) {
									error = true;
									break;
								}
							}

							if (error) {
								break;
							}
						}

						if (!error) {
							monsterFound = true;
							monsters.add(new Point(x, y));
						}
					}

					if (image[y][x]) {
						boolean monsterHit = false;
						for (Point monsterPos : monsters) {
							if (y >= monsterPos.y && x >= monsterPos.x && y - monsterPos.y < monster.length
									&& x - monsterPos.x < monster[y - monsterPos.y].length
									&& monster[y - monsterPos.y][x - monsterPos.x]) {
								monsterHit = true;
								break;
							}
						}

						if (!monsterHit) {
							waves++;
						}
					}
				}
			}

			if (!monsterFound) {
				waves = 0;
				boolean[][] copy = new boolean[image.length][image[0].length];
				for (int i = 0; i < image.length; i++) {
					for (int j = 0; j < image[i].length; j++) {
						copy[j][image[i].length - 1 - i] = image[i][j];
					}
				}
				image = copy;
			} else {
				break;
			}
		}

		return waves;
	}

	private static class Tile {

		public final boolean[][] pixels;
		public final boolean[] top;
		public final boolean[] bottom;
		public final boolean[] left;
		public final boolean[] right;

		public Tile(boolean[][] pixels) {
			this.pixels = pixels;
			top = pixels[0];
			bottom = pixels[pixels.length - 1];
			left = new boolean[pixels.length];
			right = new boolean[pixels.length];
			for (int i = 0; i < pixels.length; i++) {
				left[i] = pixels[i][0];
				right[i] = pixels[i][pixels[i].length - 1];
			}
		}

		public short fitsMirrored(Tile t) {
			short fit = fitsRotated(t);
			if (fit != 0)
				return fit;

			fit = fitsRotated(t.mirror());
			if (fit != 0)
				return (short) (fit | 1 << 6);

			return 0;
		}

		public short fitsRotated(Tile t) {
			for (short i = 0; i < 4; i++) {
				short fit = fits(t);
				if (fit != 0)
					return (short) (fit | i << 3);

				t = t.rotate();
			}
			return 0;
		}

		public short fits(Tile t) {
			if (Arrays.equals(top, t.bottom))
				return 1;
			else if (Arrays.equals(left, t.right))
				return 2;
			else if (Arrays.equals(bottom, t.top))
				return 3;
			else if (Arrays.equals(right, t.left))
				return 4;
			else
				return 0;
		}

		public Tile rotate() {
			boolean[][] copy = new boolean[pixels.length][pixels[0].length];
			for (int i = 0; i < pixels.length; i++) {
				for (int j = 0; j < pixels[i].length; j++) {
					copy[j][pixels[i].length - 1 - i] = pixels[i][j];
				}
			}
			return new Tile(copy);
		}

		public Tile mirror() {
			boolean[][] copy = new boolean[pixels.length][pixels[0].length];
			for (int i = 0; i < pixels.length; i++) {
				copy[pixels.length - 1 - i] = Arrays.copyOf(pixels[i], pixels[i].length);
			}
			return new Tile(copy);
		}

		@Override
		public String toString() {
			return Arrays.deepToString(pixels).replace("false", ".").replace("true", "#").replace(", ", "")
					.replace("[", "").replace("]", "\n");
		}

	}
}
