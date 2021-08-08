package aoc;

import java.io.IOException;
import java.io.StringReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

public abstract class Day24 {

	public static void main(String[] args) throws IOException {
		Hexagon center = new Hexagon(0, 0);
		int black = 0;
		for (String line : InputReader.readInputFileLines(24)) {
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

		/**
		 * Initializes a new Pos object.
		 * 
		 * @param x the x coordinate of the position.
		 * @param y the y coordinate of the position.
		 */
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

		/**
		 * Initializes a new Hexagon at the given position.<br>
		 * Also looks up the neighboring Hexagons.
		 * 
		 * @param x the x coordinate of the new Hexagon.
		 * @param y the y coordinate of the new Hexagon.
		 */
		public Hexagon(float x, float y) {
			COORD_LOOKUP.put(new Pos(this.x = x, this.y = y), this);
			setEast(COORD_LOOKUP.get(new Pos(x + 1, y)));
			setSouthEast(COORD_LOOKUP.get(new Pos(x + 0.5f, y + 1)));
			setSouthWest(COORD_LOOKUP.get(new Pos(x - 0.5f, y + 1)));
			setWest(COORD_LOOKUP.get(new Pos(x - 1, y)));
			setNorthWest(COORD_LOOKUP.get(new Pos(x - 0.5f, y - 1)));
			setNorthEast(COORD_LOOKUP.get(new Pos(x + 0.5f, y - 1)));
		}

		/**
		 * Toggles whether the Hexagon is black.<br>
		 * Also creates the neighboring Hexagons if they don't already exist.
		 * 
		 * @return true if the Hexagon is black.
		 */
		public boolean toggle() {
			getOrCreateEast();
			getOrCreateSouthEast();
			getOrCreateSouthWest();
			getOrCreateWest();
			getOrCreateNorthWest();
			getOrCreateNorthEast();
			return black = !black;
		}

		/**
		 * Checks whether this Hexagon has a neighbor to the east.
		 * 
		 * @return true if it has.
		 */
		public boolean hasEast() {
			return east != null;
		}

		/**
		 * Gets the Hexagon east of this one.
		 * 
		 * @return the Hexagon east of this one.
		 */
		public Hexagon getEast() {
			return east;
		}

		/**
		 * Gets the Hexagon east of this one, or creates it if it is missing.
		 * 
		 * @return the Hexagon east of this one.
		 */
		public Hexagon getOrCreateEast() {
			if (!hasEast()) {
				setEast(new Hexagon(x + 1, y));
			}

			return getEast();
		}

		/**
		 * Sets the Hexagon east of this one.<br>
		 * Also tells that Hexagon that this is the one west of it.
		 * 
		 * @param east the Hexagon east of this.
		 */
		public void setEast(Hexagon east) {
			if (east == null) {
				return;
			}

			this.east = east;

			if (!east.hasWest()) {
				east.setWest(this);
			}
		}

		/**
		 * Checks whether this Hexagon has a neighbor to the south east.
		 * 
		 * @return true if it has.
		 */
		public boolean hasSouthEast() {
			return southEast != null;
		}

		/**
		 * Gets the Hexagon south east of this one.
		 * 
		 * @return the Hexagon south east of this one.
		 */
		public Hexagon getSouthEast() {
			return southEast;
		}

		/**
		 * Gets the Hexagon south east of this one, or creates it if it is missing.
		 * 
		 * @return the Hexagon south east of this one.
		 */
		public Hexagon getOrCreateSouthEast() {
			if (!hasSouthEast()) {
				setSouthEast(new Hexagon(x + 0.5f, y + 1));
			}

			return getSouthEast();
		}

		/**
		 * Sets the Hexagon south east of this one.<br>
		 * Also tells that Hexagon that this is the one north west of it.
		 * 
		 * @param southEast the Hexagon south east of this.
		 */
		public void setSouthEast(Hexagon southEast) {
			if (southEast == null) {
				return;
			}

			this.southEast = southEast;

			if (!southEast.hasNorthWest()) {
				southEast.setNorthWest(this);
			}
		}

		/**
		 * Checks whether this Hexagon has a neighbor to the south west.
		 * 
		 * @return true if it has.
		 */
		public boolean hasSouthWest() {
			return southWest != null;
		}

		/**
		 * Gets the Hexagon south west of this one.
		 * 
		 * @return the Hexagon south west of this one.
		 */
		public Hexagon getSouthWest() {
			return southWest;
		}

		/**
		 * Gets the Hexagon south west of this one, or creates it if it is missing.
		 * 
		 * @return the Hexagon south west of this one.
		 */
		public Hexagon getOrCreateSouthWest() {
			if (!hasSouthWest()) {
				setSouthWest(new Hexagon(x - 0.5f, y + 1));
			}

			return getSouthWest();
		}

		/**
		 * Sets the Hexagon south west of this one.<br>
		 * Also tells that Hexagon that this is the one north east of it.
		 * 
		 * @param southWest the Hexagon south west of this.
		 */
		public void setSouthWest(Hexagon southWest) {
			if (southWest == null) {
				return;
			}

			this.southWest = southWest;

			if (!southWest.hasNorthEast()) {
				southWest.setNorthEast(this);
			}
		}

		/**
		 * Checks whether this Hexagon has a neighbor to the west.
		 * 
		 * @return true if it has.
		 */
		public boolean hasWest() {
			return west != null;
		}

		/**
		 * Gets the Hexagon west of this one.
		 * 
		 * @return the Hexagon west of this one.
		 */
		public Hexagon getWest() {
			return west;
		}

		/**
		 * Gets the Hexagon west of this one, or creates it if it is missing.
		 * 
		 * @return the Hexagon west of this one.
		 */
		public Hexagon getOrCreateWest() {
			if (!hasWest()) {
				setWest(new Hexagon(x - 1, y));
			}

			return getWest();
		}

		/**
		 * Sets the Hexagon west of this one.<br>
		 * Also tells that Hexagon that this is the one east of it.
		 * 
		 * @param west the Hexagon west of this.
		 */
		public void setWest(Hexagon west) {
			if (west == null) {
				return;
			}

			this.west = west;

			if (!west.hasEast()) {
				west.setEast(this);
			}
		}

		/**
		 * Checks whether this Hexagon has a neighbor to the north west.
		 * 
		 * @return true if it has.
		 */
		public boolean hasNorthWest() {
			return northWest != null;
		}

		/**
		 * Gets the Hexagon north west of this one.
		 * 
		 * @return the Hexagon north west of this one.
		 */
		public Hexagon getNorthWest() {
			return northWest;
		}

		/**
		 * Gets the Hexagon north west of this one, or creates it if it is missing.
		 * 
		 * @return the Hexagon north west of this one.
		 */
		public Hexagon getOrCreateNorthWest() {
			if (!hasNorthWest()) {
				setNorthWest(new Hexagon(x - 0.5f, y - 1));
			}

			return getNorthWest();
		}

		/**
		 * Sets the Hexagon north west of this one.<br>
		 * Also tells that Hexagon that this is the one south east of it.
		 * 
		 * @param northWest the Hexagon north west of this.
		 */
		public void setNorthWest(Hexagon northWest) {
			if (northWest == null) {
				return;
			}

			this.northWest = northWest;

			if (!northWest.hasSouthEast()) {
				northWest.setSouthEast(this);
			}
		}

		/**
		 * Checks whether this Hexagon has a neighbor to the north east.
		 * 
		 * @return true if it has.
		 */
		public boolean hasNorthEast() {
			return northEast != null;
		}

		/**
		 * Gets the Hexagon north east of this one.
		 * 
		 * @return the Hexagon north east of this one.
		 */
		public Hexagon getNorthEast() {
			return northEast;
		}

		/**
		 * Gets the Hexagon north east of this one, or creates it if it is missing.
		 * 
		 * @return the Hexagon north east of this one.
		 */
		public Hexagon getOrCreateNorthEast() {
			if (!hasNorthEast()) {
				setNorthEast(new Hexagon(x + 0.5f, y - 1));
			}

			return getNorthEast();
		}

		/**
		 * Sets the Hexagon north east of this one.<br>
		 * Also tells that Hexagon that this is the one south west of it.
		 * 
		 * @param northEast the Hexagon north east of this.
		 */
		public void setNorthEast(Hexagon northEast) {
			if (northEast == null) {
				return;
			}

			this.northEast = northEast;

			if (!northEast.hasSouthWest()) {
				northEast.setSouthWest(this);
			}
		}

		/**
		 * Checks whether this Hexagon should toggle being black.
		 * 
		 * <p>
		 * If a Hexagon is black it becomes white if either none or more then 2 of its
		 * neighbors are also black.<br>
		 * If a Hexagon is white it becomes black if 2 of its neighbors are black.
		 * </p>
		 * 
		 * @return whether this Hexagon should change its color.
		 */
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

		/**
		 * Makes all Hexagons check if they should change their color, and if yes do so.
		 * 
		 * @return the number of black Hexagons after the change.
		 */
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
