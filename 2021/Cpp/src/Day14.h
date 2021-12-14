/*
 * Day14.h
 *
 *  Created on: 14.12.2021
 *      Author: ToMe25
 */

#ifndef DAY14_H_
#define DAY14_H_

#include "Main.h"
#include <map>

/**
 * Increases the values in the given map based on the given rules the given number of times.
 *
 * @param polymer		The map of character pairs to numbers to increase.
 * @param first			The first char of the polymer. Required for counting.
 * @param expansions	A map containing the rules for the expansion.
 * 						The value for the key.first + value pair and the key.second + value pair
 * 						in polymer are increased by the number of the key pair.
 * 						Afterwards the value of the key pair is reduced by its initial value.
 * @param times			The number of times to expand the input.
 * @return	The difference in the count of the most and least common element of the result.
 */
uint64_t expand_and_count(std::map<std::pair<char, char>, uint64_t> polymer,
		const char first,
		const std::map<std::pair<char, char>, char> expansions,
		const uint8_t times);

#endif /* DAY14_H_ */
