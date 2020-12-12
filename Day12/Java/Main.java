import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

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

		int north = 0;
		int west = 0;
		int waypointNorth = 1;
		int waypointWest = -10;
		int shipNorth = 0;
		int shipWest = 0;
		int rotation = 1;
		for (String line : Files.readAllLines(input.toPath())) {
			int value = Integer.parseInt(line.substring(1));
			switch (line.charAt(0)) {
			case 'N':
				north += value;
				waypointNorth += value;
				break;
			case 'S':
				north -= value;
				waypointNorth -= value;
				break;
			case 'E':
				west -= value;
				waypointWest -= value;
				break;
			case 'W':
				west += value;
				waypointWest += value;
				break;
			case 'L':
				value /= 90;
				rotation = (rotation - value) % 4;
				while (rotation < 0) {
					rotation += 4;
				}
				int oldWest = waypointWest;
				if (value == 1) {
					waypointWest = waypointNorth;
					waypointNorth = -oldWest;
				} else if (value == 2) {
					waypointWest *= -1;
					waypointNorth *= -1;
				} else if (value == 3) {
					waypointWest = -waypointNorth;
					waypointNorth = oldWest;
				}
				break;
			case 'R':
				value /= 90;
				rotation = (rotation + value) % 4;
				while (rotation < 0) {
					rotation += 4;
				}
				oldWest = waypointWest;
				if (value == 1) {
					waypointWest = -waypointNorth;
					waypointNorth = oldWest;
				} else if (value == 2) {
					waypointWest *= -1;
					waypointNorth *= -1;
				} else if (value == 3) {
					waypointWest = waypointNorth;
					waypointNorth = -oldWest;
				}
				break;
			case 'F':
				if (rotation == 0) {
					north += value;
				} else if (rotation == 1) {
					west -= value;
				} else if (rotation == 2) {
					north -= value;
				} else if (rotation == 3) {
					west += value;
				}

				shipNorth += waypointNorth;
				shipWest += waypointWest;
			}
		}

		System.out.println("The Manhattan distance from the ferrys starting position to its part 1 position is "
				+ (Math.abs(north) + Math.abs(west)) + ".");

		System.out.println("The Manhattan distance from the ferrys starting position to its part 2 position is "
				+ (Math.abs(shipNorth) + Math.abs(shipWest)) + ".");
	}
}
