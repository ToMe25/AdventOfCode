/*
 * Day10.cpp
 *
 *  Created on: Dec 10, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::string day10part1(std::ifstream input) {
	int64_t registerX = 1;
	uint64_t score = 0;
	uint16_t clock = 0;
	std::string line;
	while(std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}
		clock++;
		if (clock % 40 == 20) {
			score += registerX * clock;
		}

		if (line == "noop") {
			// Do nothing
		} else if (line.substr(0, 4) == "addx") {
			clock++;
			if (clock % 40 == 20) {
				score += registerX * clock;
			}
			registerX += std::stoi(line.substr(5));
		}
	}
	return std::to_string(score);
}

bool d10p1 = aoc::registerPart1(10, &day10part1);
