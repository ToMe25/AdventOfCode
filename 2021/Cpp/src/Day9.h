/*
 * Day9.h
 *
 *  Created on: 10.12.2021
 *      Author: ToMe25
 */

#ifndef DAY9_H_
#define DAY9_H_

#include "Main.h"
#include <set>

/*
 * Recursively checks this field and its neighbors for whether they are part of the same basin.
 *
 * @param x_pos		The x coordinate of the position to check.
 * @param y_pos		The y coordinate of the position to check.
 * @param checked	A set containing all the already checked positions. Used to prevent checking the same position again.
 * @param map		The map to check. A 2D 100 x 100 uint8_t array containing the heights of all positions.
 * @return	A set containing all the not positions that are part of this basin and reachable
 *        	without crossing already checked positions.
 */
const std::set<std::pair<uint8_t, uint8_t>> recursive_check(const uint8_t x_pos,
		const uint8_t y_pos,
		const std::set<std::pair<uint8_t, uint8_t>> checked,
		const uint8_t map[100][100]);

#endif /* DAY9_H_ */
