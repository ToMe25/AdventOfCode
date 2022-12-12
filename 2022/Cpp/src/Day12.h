/*
 * Day12.h
 *
 *  Created on: Dec 12, 2022
 *      Author: ToMe25
 */

#ifndef DAY12_H_
#define DAY12_H_

#include "Main.h"
#include <map>

namespace aoc {
/**
 * Checks whether the given new position should be checked.
 *
 * @param heights			The current height map.
 * @param costs				The cheapest cost to reach each position.
 * @param current_height	The height of the current position.
 * @param current_cost		The cost required to reach the current position.
 * @param end_cost			The minimum cost known to reach the target. Set to -1 if the end was not yet reached.
 * @param new_pos			The position to potentially check next.
 */
bool check_position(const std::vector<std::vector<uint8_t>> &heights,
		const std::map<std::pair<uint8_t, uint8_t>, uint16_t> &costs,
		const uint8_t current_height, const uint16_t current_cost,
		const int16_t end_cost, const std::pair<uint8_t, uint8_t> &new_pos);
}

namespace std {
template<class T1, class T2>
struct hash<pair<T1, T2>> {
	/**
	 * Generates a hash code for the given pair.
	 *
	 * @tparam T1	The type of the first element of the pair.
	 * @tparam T2	The type of the second element in the pair.
	 * @param p	The pair to generate a hash for.
	 * @return	the newly generated hash.
	 */
	size_t operator()(const pair<T1, T2> &p) const;
};
}

#endif /* DAY12_H_ */
