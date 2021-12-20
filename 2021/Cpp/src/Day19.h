/*
 * Day19.h
 *
 *  Created on: 19.12.2021
 *      Author: ToMe25
 */

#ifndef DAY19_H_
#define DAY19_H_

#include "Main.h"
#include <array>
#include <list>
#include <map>
#include <vector>

typedef std::array<int16_t, 3> Position;
typedef std::vector<Position> Scanner;

/**
 * Gets all the currently unknown neighbors of the given scanner.
 * Also removes said neighbors from the list of unknown scanners.
 *
 * @param unknown_scanners	A list containing all the scanners whose position is not yet known.
 * @param scanner			The scanner for whose neighbors to search.
 * @return	A map mapping the found and correctly rotated neighbors to their positions relative to the given scanner.
 */
std::map<Scanner, Position> get_neighbors(std::list<Scanner> &unknown_scanners,
		const Scanner scanner);

/**
 * Rotates the given Scanner around its center by the given amount.
 *
 * @param scanner	The scanner to rotate.
 * @param rotation	How much to rotate the Scanner.
 * 					Range 0-63.
 * 					The last 2 bits(values 1, 2, and 3) are for the x rotation.
 * 					The 2 bits before that(values 4, 8, and 12) are for the y rotation.
 * 					The 2 bits before that(values 16, 32, and 48) are for the z rotation.
 * @return	The rotated scanner.
 */
Scanner rotate(const Scanner scanner, const uint8_t rotation);

/**
 * Returns a scanner having the given offset added to all the values of the given scanner.
 *
 * @param scanner	The scanner for which to generate a translated version.
 * @param pos		The offset to add the every position the scanner found.
 * @return 	The new scanner with translated values.
 */
Scanner get_relative_to(const Scanner scanner, const Position pos);

/**
 * Adds the given two positions together.
 *
 * @param pos_a	The position to add pos_b to.
 * @param pos_b	The position to add to pos_a.
 * @return	The sum of the two given positions.
 */
Position position_add(const Position pos_a, const Position pos_b);

/**
 * Subtracts the second position from the first.
 *
 * @param pos_a	The position to subtract pos_b from.
 * @param pos_b The position to subtract from pos_a.
 * @return	The difference between the two positions.
 */
Position position_sub(const Position pos_a, const Position pos_b);

/**
 * Prints a string representation of the given position to the given output stream.
 *
 * @param stream	The stream to print to.
 * @param scanner	The position to print to the given output stream.
 * @return	The given output stream.
 */
std::ostream &operator <<(std::ostream &stream, const Position &pos);

/**
 * Prints a string representation of the given scanner to the given output stream.
 *
 * @param stream	The stream to print to.
 * @param scanner	The scanner to print to the given output stream.
 * @return	The given output stream.
 */
std::ostream &operator <<(std::ostream &stream, const Scanner &scanner);

namespace std {
template<>
struct hash<Position> {
	/**
	 * Creates a hash for the given Position.
	 *
	 * @param to_hash	The position for which to calculate the hash.
	 * @return The hash value for the given position.
	 */
	size_t operator ()(const Position pos) const;
};
}

#endif /* DAY19_H_ */
