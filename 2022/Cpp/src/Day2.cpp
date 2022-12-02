/*
 * Day2.cpp
 *
 *  Created on: Dec 2, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::string day2part1(std::ifstream input) {
	uint32_t score = 0;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		score += line[2] - 'X' + 1;
		if (line[0] - 'A' == line[2] - 'Y') {
			score += 6;
		} else if (line[0] == 'C' && line[2] == 'X') {
			score += 6;
		} else if (line[0] - 'A' == line[2] - 'X') {
			score += 3;
		}
	}

	return std::to_string(score);
}

std::string day2part2(std::ifstream input) {
	uint32_t score = 0;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		score += (line[2] - 'X') * 3;
		score += (line[0] - 'A' + 3 + line[2] - 'Y') % 3 + 1;
	}

	return std::to_string(score);
}

bool d2p1 = aoc::registerPart1(2, &day2part1);
bool d2p2 = aoc::registerPart2(2, &day2part2);
