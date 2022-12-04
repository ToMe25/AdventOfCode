/*
 * Day4.cpp
 *
 *  Created on: Dec 4, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::pair<std::string, std::string> day4Combined(std::ifstream input) {
	uint16_t useless = 0;
	uint16_t overlapping = 0;

	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		const size_t first = line.find('-');
		const size_t comma = line.find(',', first);
		const size_t second = line.find('-', comma);

		const int start1 = std::stoi(line.substr(0, first));
		const int end1 = std::stoi(line.substr(first + 1, comma - first - 1));
		const int start2 = std::stoi(
				line.substr(comma + 1, second - comma - 1));
		const int end2 = std::stoi(line.substr(second + 1));
		if (start1 >= start2 && end1 <= end2) {
			useless++;
		} else if (start1 <= start2 && end1 >= end2) {
			useless++;
		}

		if ((start1 >= start2 && start1 <= end2)
				|| (start2 >= start1 && start2 <= end1)) {
			overlapping++;
		}
	}

	return {std::to_string(useless), std::to_string(overlapping)};
}

bool d4c = aoc::registerCombined(4, &day4Combined);
