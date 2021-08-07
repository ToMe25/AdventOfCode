package aoc;

import java.io.IOException;

public abstract class Day12 {

	public static void main(String[] args) throws IOException {
		int north = 0;
		int west = 0;
		int waypointNorth = 1;
		int waypointWest = -10;
		int shipNorth = 0;
		int shipWest = 0;
		int rotation = 1;
		for (String line : InputReader.readInputFileLines(12)) {
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

				shipNorth += waypointNorth * value;
				shipWest += waypointWest * value;
			}
		}

		System.out.println("The Manhattan distance from the ferrys starting position to its part 1 position is "
				+ (Math.abs(north) + Math.abs(west)) + ".");

		System.out.println("The Manhattan distance from the ferrys starting position to its part 2 position is "
				+ (Math.abs(shipNorth) + Math.abs(shipWest)) + ".");
	}
}
