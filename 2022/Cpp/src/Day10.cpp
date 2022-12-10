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

std::string day10part2(std::ifstream input) {
	int64_t registerX = 1;
	std::string image = "\n";
	uint16_t clock = 0;
	std::string line;
	while(std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}
		image += (clock % 40 >= registerX - 1 && clock % 40 <= registerX + 1) ? '#' : '.';
		clock++;
		if (clock % 40 == 0) {
			image += '\n';
		}

		if (line == "noop") {
			// Do nothing
		} else if (line.substr(0, 4) == "addx") {
			image += (clock % 40 >= registerX - 1 && clock % 40 <= registerX + 1) ? '#' : '.';
			clock++;
			if (clock % 40 == 0) {
				image += '\n';
			}
			registerX += std::stoi(line.substr(5));
		}
	}
	return image;
}

bool d10p1 = aoc::registerPart1(10, &day10part1);
bool d10p2 = aoc::registerPart2(10, &day10part2);
