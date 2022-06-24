/*
 * Day23.h
 *
 *  Created on: 21.06.2022
 *      Author: ToMe25
 */

#ifndef DAY23_H_
#define DAY23_H_

#include "Main.h"

/**
 * Calculates the minimum cost for ordering the amphipods from the given token.
 * Token structure is "SPACES:BURROWS:" where "BURROWS" is a colon separated list of the spaces in each burrow.
 *
 * @param token	The token representing the initial state to check.
 * @return	The minimum cost for sorting the amphipods in their burrows.
 */
uint32_t get_min_cost(std::string token);

/**
 * Calculates the new token and cost after moving a amphipod according to the given values.
 *
 * @param token			The current token before moving.
 * @param cost			The current cost before moving.
 * @param burrow		The index of the burrow into/from which to move.
 * @param burrow_length	The number of spaces in each burrow.
 * @param pos			The position in the "hallway" spaces into/from which to move.
 * @param depth			The depth of how deep in the burrow the space to use is.
 * @return	A pair containing the new token(first) and new sum cost(second) after moving.
 */
std::pair<std::string, uint32_t> move(const std::string token,
		const uint32_t cost, const uint8_t burrow, const uint8_t burrow_length,
		const uint8_t pos, const uint8_t depth);

#endif /* DAY23_H_ */
