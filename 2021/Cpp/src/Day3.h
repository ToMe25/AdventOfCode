/*
 * Day3.h
 *
 *  Created on: 03.12.2021
 *      Author: ToMe25
 */

#ifndef DAY3_H_
#define DAY3_H_

#include "Main.h"
#include <vector>

template <>
void DayRunner<3>::solve(std::ifstream input);

/**
 * Counts the number of 1 and 0 bits in the given position, and returns the difference.
 * Positive if there are more ones.
 *
 * numbers: the vector of numbers to search in.
 * bit: the position of the bit to count.
 * returns: the difference in ones and zeros.
 */
int16_t count_bits(const std::vector<uint16_t> numbers, const uint8_t bit);

/*
 * Finds the number that matches the given criteria out of the given list.
 *
 * numbers: the vector of numbers to search in.
 * lcb: whether to look for the least common bit rather then the most common bit.
 * returns: the found number.
 */
uint16_t find_number(const std::vector<uint16_t> numbers, const bool lcb);

#endif /* DAY3_H_ */
