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
		const size_t len = line.length();
		if (len == 0) {
			continue;
		}

		const size_t end1 = line.length() / 2;
		uint64_t first = 0;
		for (size_t i = 0; i < len; i++) {
			const char c = line[i];
			const uint64_t pos = (c > 'Z' ? c - 'a' : c - 'A' + 26);
			if (i < end1) {
				first |= (1ll << pos);
			} else if (first & (1ll << pos)) {
				sum += pos + 1;
				break;
			}
		}
	}

	return std::to_string(sum);
}

std::string day3part2(std::ifstream input) {
	uint32_t sum = 0;

	uint64_t options = 0;
	size_t ln = 0;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		uint64_t newopts = 0;
		for (char c : line) {
			newopts |= (1ll << (c > 'Z' ? c - 'a' : c - 'A' + 26));
		}

		if (ln % 3 == 0) {
			options = newopts;
		} else {
			options &= newopts;
			if (ln % 3 == 2) {
				uint8_t pos = 1;
				uint64_t mask = 1;
				while (!(options & mask) && pos < 52) {
					mask <<= 1;
					pos++;
				}

				sum += pos;
			}
		}
		ln++;
	}

	return std::to_string(sum);
}

bool d3p1 = aoc::registerPart1(3, &day3part1);
bool d3p2 = aoc::registerPart2(3, &day3part2);
