import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.Arrays;
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

		List<String> lines = Files.readAllLines(input.toPath());
		char[][] board = new char[lines.size()][];
		for (int i = 0; i < lines.size(); i++) {
			board[i] = lines.get(i).toCharArray();
		}

		char[][] oldBoard;
		int lastOccupied = -1;
		int occupied = 0;
		while (lastOccupied != occupied) {
			lastOccupied = occupied;
			oldBoard = Arrays.stream(board).map(char[]::clone).toArray(char[][]::new);
			for (int y = 0; y < board.length; y++) {
				for (int x = 0; x < board[y].length; x++) {
					if (board[y][x] != '.') {
						int occupiedNeighbors = 0;
						for (int i = -1; i < 2; i++) {
							for (int j = -1; j < 2; j++) {
								if (y + i >= 0 && y + i < board.length && x + j >= 0 && x + j < board[y + i].length) {
									if (oldBoard[y + i][x + j] == '#' && !(i == 0 && j == 0)) {
										occupiedNeighbors++;
									}
								}
							}
						}

						if (occupiedNeighbors == 0) {
							board[y][x] = '#';
						} else if (occupiedNeighbors >= 4) {
							board[y][x] = 'L';
						}
					}
				}
			}

			occupied = (int) Arrays.stream(board).flatMapToInt(arr -> new String(arr).chars()).filter(c -> c == '#')
					.count();
		}

		System.out.println("Found " + occupied + " occupied seats for part 1.");

		for (int i = 0; i < lines.size(); i++) {
			board[i] = lines.get(i).toCharArray();
		}

		lastOccupied = -1;
		occupied = 0;
		while (lastOccupied != occupied) {
			lastOccupied = occupied;
			oldBoard = Arrays.stream(board).map(char[]::clone).toArray(char[][]::new);
			for (int y = 0; y < board.length; y++) {
				for (int x = 0; x < board[y].length; x++) {
					if (board[y][x] != '.') {
						int occupiedNeighbors = 0;
						for (int i = -1; i < 2; i++) {
							for (int j = -1; j < 2; j++) {
								int k = 1;
								while (y + i * k >= 0 && y + i * k < board.length && x + j * k >= 0
										&& x + j * k < board[y + i * k].length && !(i == 0 && j == 0)
										&& board[y + i * k][x + j * k] == '.') {
									k++;
								}

								if (y + i * k >= 0 && y + i * k < board.length && x + j * k >= 0
										&& x + j * k < board[y + i * k].length && !(i == 0 && j == 0)) {
									if (oldBoard[y + i * k][x + j * k] == '#') {
										occupiedNeighbors++;
									}
								}
							}
						}

						if (occupiedNeighbors == 0) {
							board[y][x] = '#';
						} else if (occupiedNeighbors >= 5) {
							board[y][x] = 'L';
						}
					}
				}
			}

			occupied = (int) Arrays.stream(board).flatMapToInt(arr -> new String(arr).chars()).filter(c -> c == '#')
					.count();
		}

		System.out.println("Found " + occupied + " occupied seats for part 2.");
	}
}
