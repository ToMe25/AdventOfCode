/*
 * Day1.cpp
 *
 *  Created on: Dec 1, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::string part1(std::ifstream input) {
	uint32_t max = 0;
	uint32_t current = 0;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			if (current > max) {
				max = current;
			}
			current = 0;
		} else {
			current += std::stoi(line);
		}
	}

	if (current > max) {
		max = current;
	}

	return std::to_string(max);
}

bool d1p1 = aoc::registerPart1(1, &part1);
