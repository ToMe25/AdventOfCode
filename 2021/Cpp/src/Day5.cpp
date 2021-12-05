/*
 * Day5.cpp
 *
 *  Created on: 05.12.2021
 *      Author: ToMe25
 */

#include "Day5.h"
#include <algorithm>
#include <iostream>
#include <map>
#include <sstream>

template <>
void DayRunner<5>::solve(std::ifstream input) {
	std::string line;
	uint16_t x1 = 0, y1 = 0, x2 = 0, y2 = 0;
	std::istringstream converter;
	std::map<std::pair<uint16_t, uint16_t>, uint8_t> lines;

	while (input >> line) {
		if (line.find(',') != std::string::npos) {
			std::replace(line.begin(), line.end(), ',', ' ');
			converter.clear();
			converter.str(line);

			if (x1 == 0 && y1 == 0) {
				converter >> x1 >> y1;
			} else {
				converter >> x2 >> y2;
				if (x1 == x2 || y1 == y2) {
					for (uint16_t x = std::min(x1, x2); x <= std::max(x1, x2); x++) {
						for (uint16_t y = std::min(y1, y2); y <= std::max(y1, y2); y++) {
							lines[std::pair<uint16_t, uint16_t>(x, y)]++;
						}
					}
				}
				x1 = y1 = x2 = y2 = 0;
			}
		}
	}

	uint16_t dangers = 0;
	for (std::pair<std::pair<uint16_t, uint16_t>, uint8_t> point : lines) {
		if (point.second > 1) {
			dangers++;
		}
	}

	std::cout << "Found " << dangers << " part 1 dangerous spots." << std::endl;

	input = getInputFileStream(5);
	while (input >> line) {
		if (line.find(',') != std::string::npos) {
			std::replace(line.begin(), line.end(), ',', ' ');
			converter.clear();
			converter.str(line);

			if (x1 == 0 && y1 == 0) {
				converter >> x1 >> y1;
			} else {
				converter >> x2 >> y2;
				if (x1 != x2 && y1 != y2) {
					if (x1 < x2) {
						for (uint16_t x = x1, y = y1; x <= x2; x++, y1 > y2 ? y-- : y++) {
							lines[std::pair<uint16_t, uint16_t>(x, y)]++;
						}
					} else {
						for (uint16_t x = x1, y = y1; x >= x2; x--, y1 > y2 ? y-- : y++) {
							lines[std::pair<uint16_t, uint16_t>(x, y)]++;
						}
					}
				}
				x1 = y1 = x2 = y2 = 0;
			}
		}
	}

	dangers = 0;
	for (std::pair<std::pair<uint16_t, uint16_t>, uint8_t> point : lines) {
		if (point.second > 1) {
			dangers++;
		}
	}

	std::cout << "Found " << dangers << " part 2 dangerous spots." << std::endl;
}
