package aoc;

import java.io.File;
import java.io.IOException;
import java.io.StringReader;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

public class Day24 {

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
			input = new File(input, "Day24.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		Hexagon center = new Hexagon(0, 0);
		int black = 0;
		for (String line : Files.readAllLines(input.toPath())) {
			StringReader reader = new StringReader(line);
			Hexagon current = center;
			ready: while (reader.ready()) {
				switch (reader.read()) {
				case 'e':
					current = current.getOrCreateEast();
					break;
				case 's':
					if (reader.read() == 'e') {
						current = current.getOrCreateSouthEast();
					} else {
						current = current.getOrCreateSouthWest();
					}
					break;
				case 'w':
					current = current.getOrCreateWest();
					break;
				case 'n':
					if (reader.read() == 'w') {
						current = current.getOrCreateNorthWest();
					} else {
						current = current.getOrCreateNorthEast();
					}
					break;
				case -1:
					break ready;
				}
			}
			black += current.toggle() ? 1 : -1;
		}

		System.out.println("After part 1 there are " + black + " black tiles.");

		int black2 = 0;
		for (int i = 0; i < 100; i++) {
			center.check();
			black2 = Hexagon.updateAll();
		}

		System.out.println("After part 2 there are " + black2 + " black tiles.");
	}

	private static class Pos {
		public float x, y;

		public Pos(float x, float y) {
			this.x = x;
			this.y = y;
		}

		@Override
		public int hashCode() {
			return Objects.hash(x, y);
		}

		@Override
		public boolean equals(Object obj) {
			if (this == obj)
				return true;
			if (obj == null)
				return false;
			if (getClass() != obj.getClass())
				return false;
			Pos other = (Pos) obj;
			return Float.floatToIntBits(x) == Float.floatToIntBits(other.x)
					&& Float.floatToIntBits(y) == Float.floatToIntBits(other.y);
		}
	}

	private static class Hexagon {
		private static final Map<Pos, Hexagon> COORD_LOOKUP = new HashMap<>();

		private Hexagon east;
		private Hexagon southEast;
		private Hexagon southWest;
		private Hexagon west;
		private Hexagon northWest;
		private Hexagon northEast;
		private float x, y;
		private boolean black;

		public Hexagon(float x, float y) {
			COORD_LOOKUP.put(new Pos(this.x = x, this.y = y), this);
			setEast(COORD_LOOKUP.get(new Pos(x + 1, y)));
			setSouthEast(COORD_LOOKUP.get(new Pos(x + 0.5f, y + 1)));
			setSouthWest(COORD_LOOKUP.get(new Pos(x - 0.5f, y + 1)));
			setWest(COORD_LOOKUP.get(new Pos(x - 1, y)));
			setNorthWest(COORD_LOOKUP.get(new Pos(x - 0.5f, y - 1)));
			setNorthEast(COORD_LOOKUP.get(new Pos(x + 0.5f, y - 1)));
		}

		public boolean toggle() {
			getOrCreateEast();
			getOrCreateSouthEast();
			getOrCreateSouthWest();
			getOrCreateWest();
			getOrCreateNorthWest();
			getOrCreateNorthEast();
			return black = !black;
		}

		public boolean hasEast() {
			return east != null;
		}

		public Hexagon getEast() {
			return east;
		}

		public Hexagon getOrCreateEast() {
			if (!hasEast()) {
				setEast(new Hexagon(x + 1, y));
			}

			return getEast();
		}

		public void setEast(Hexagon east) {
			if (east == null) {
				return;
			}

			this.east = east;

			if (!east.hasWest()) {
				east.setWest(this);
			}
		}

		public boolean hasSouthEast() {
			return southEast != null;
		}

		public Hexagon getSouthEast() {
			return southEast;
		}

		public Hexagon getOrCreateSouthEast() {
			if (!hasSouthEast()) {
				setSouthEast(new Hexagon(x + 0.5f, y + 1));
			}

			return getSouthEast();
		}

		public void setSouthEast(Hexagon southEast) {
			if (southEast == null) {
				return;
			}

			this.southEast = southEast;

			if (!southEast.hasNorthWest()) {
				southEast.setNorthWest(this);
			}
		}

		public boolean hasSouthWest() {
			return southWest != null;
		}

		public Hexagon getSouthWest() {
			return southWest;
		}

		public Hexagon getOrCreateSouthWest() {
			if (!hasSouthWest()) {
				setSouthWest(new Hexagon(x - 0.5f, y + 1));
			}

			return getSouthWest();
		}

		public void setSouthWest(Hexagon southWest) {
			if (southWest == null) {
				return;
			}

			this.southWest = southWest;

			if (!southWest.hasNorthEast()) {
				southWest.setNorthEast(this);
			}
		}

		public boolean hasWest() {
			return west != null;
		}

		public Hexagon getWest() {
			return west;
		}

		public Hexagon getOrCreateWest() {
			if (!hasWest()) {
				setWest(new Hexagon(x - 1, y));
			}

			return getWest();
		}

		public void setWest(Hexagon west) {
			if (west == null) {
				return;
			}

			this.west = west;

			if (!west.hasEast()) {
				west.setEast(this);
			}
		}

		public boolean hasNorthWest() {
			return northWest != null;
		}

		public Hexagon getNorthWest() {
			return northWest;
		}

		public Hexagon getOrCreateNorthWest() {
			if (!hasNorthWest()) {
				setNorthWest(new Hexagon(x - 0.5f, y - 1));
			}

			return getNorthWest();
		}

		public void setNorthWest(Hexagon northWest) {
			if (northWest == null) {
				return;
			}

			this.northWest = northWest;

			if (!northWest.hasSouthEast()) {
				northWest.setSouthEast(this);
			}
		}

		public boolean hasNorthEast() {
			return northEast != null;
		}

		public Hexagon getNorthEast() {
			return northEast;
		}

		public Hexagon getOrCreateNorthEast() {
			if (!hasNorthEast()) {
				setNorthEast(new Hexagon(x + 0.5f, y - 1));
			}

			return getNorthEast();
		}

		public void setNorthEast(Hexagon northEast) {
			if (northEast == null) {
				return;
			}

			this.northEast = northEast;

			if (!northEast.hasSouthWest()) {
				northEast.setSouthWest(this);
			}
		}

		private boolean check() {
			int blackNeighbors = 0;
			if (hasEast())
				blackNeighbors += east.black ? 1 : 0;
			if (hasSouthEast())
				blackNeighbors += southEast.black ? 1 : 0;
			if (hasSouthWest())
				blackNeighbors += southWest.black ? 1 : 0;
			if (hasWest())
				blackNeighbors += west.black ? 1 : 0;
			if (hasNorthWest())
				blackNeighbors += northWest.black ? 1 : 0;
			if (hasNorthEast())
				blackNeighbors += northEast.black ? 1 : 0;

			if (black) {
				return blackNeighbors == 0 || blackNeighbors > 2;
			} else {
				return blackNeighbors == 2;
			}
		}

		public static int updateAll() {
			List<Hexagon> marked = new ArrayList<>();
			for (Hexagon hex : COORD_LOOKUP.values()) {
				if (hex.check()) {
					marked.add(hex);
				}
			}

			for (Hexagon hex : marked) {
				hex.toggle();
			}

			int black = 0;
			for (Hexagon hex : COORD_LOOKUP.values()) {
				if (hex.black) {
					black++;
				}
			}
			return black;
		}
	}
}
