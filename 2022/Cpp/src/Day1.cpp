/*
 * Day1.cpp
 *
 *  Created on: Dec 1, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::pair<std::string, std::string> day1comb(std::ifstream input) {
	uint32_t max[3] { 0 };
	uint32_t current = 0;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			if (current >= max[0]) {
				max[2] = max[1];
				max[1] = max[0];
				max[0] = current;
			} else if (current >= max[1]) {
				max[2] = max[1];
				max[1] = current;
			} else if (current > max[2]) {
				max[2] = current;
			}

			current = 0;
		} else {
			current += std::stoi(line);
		}
	}

	if (current >= max[0]) {
		max[2] = max[1];
		max[1] = max[0];
		max[0] = current;
	} else if (current >= max[1]) {
		max[2] = max[1];
		max[1] = current;
	} else if (current > max[2]) {
		max[2] = current;
	}

	return {std::to_string(max[0]), std::to_string(max[0] + max[1] + max[2])};
}

bool d1c = aoc::registerCombined(1, &day1comb);
