/*
 * Day18.h
 *
 *  Created on: 19.12.2021
 *      Author: ToMe25
 */

#ifndef DAY18_H_
#define DAY18_H_

#include "Main.h"

/**
 * Adds the two given snailfish numbers together.
 *
 * @param first		The number to add the second number to.
 * @param second	The number to add the the first number.
 * @return	The reduced result from adding the given two numbers.
 */
std::string add_snailfish(const std::string first, const std::string second);

/**
 * Gets the magnitude of a snailfish number.
 *
 * @param number	The snailfish number for which to get the magnitude.
 * @return	The magnitude for the given number.
 */
uint16_t get_magnitude(const std::string number);

#endif /* DAY18_H_ */
