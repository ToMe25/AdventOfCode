/*
 * Day3.cpp
 *
 *  Created on: Dec 3, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::string day3part1(std::ifstream input) {
	uint32_t sum = 0;

	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		size_t end1 = line.length() / 2;
		for (size_t i = end1; i < line.length(); i++) {
			if (std::count(line.begin(), line.begin() + end1, line.at(i)) > 0) {
				if (line.at(i) >= 'a' && line.at(i) <= 'z') {
					sum += line.at(i) - 'a' + 1;
				} else {
					sum += line.at(i) - 'A' + 27;
				}
				break;
			}
		}
	}

	return std::to_string(sum);
}

bool d3p1 = aoc::registerPart1(3, &day3part1);
