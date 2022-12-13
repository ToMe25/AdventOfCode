/*
 * Day13.h
 *
 *  Created on: Dec 13, 2022
 *      Author: ToMe25
 */

#ifndef DAY13_H_
#define DAY13_H_

#include "Main.h"

namespace aoc {
/**
 * Compares the two lists of lists or numbers.
 *
 * @param first		The first of the two lists to compare.
 * @param second	The second of the two lists to compare.
 * @return	True if the first list should come before the second one.
 */
bool compare_lists(const std::string &first, const std::string &second);
}

#endif /* DAY13_H_ */
