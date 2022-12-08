/*
 * Day8.cpp
 *
 *  Created on: Dec 8, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::pair<std::string, std::string> day8combined(std::ifstream input) {
	std::vector<std::vector<uint8_t>> map;
	std::string line;
	size_t ln = 0;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		map.push_back(std::vector<uint8_t>());
		map[ln].reserve(line.length());
		for (char c : line) {
			map[ln].push_back(c - '0');
		}
		ln++;
	}

	size_t visible = 0;
	size_t best_scenic = 0;
	for (size_t y = 0; y < map.size(); y++) {
		for (size_t x = 0; x < map[y].size(); x++) {
			const uint8_t c = map[y][x];
			bool hidden = false;
			size_t scenic = 1;
			bool vis = false;
			for (int64_t x1 = x - 1; x1 >= 0; x1--) {
				if (map[y][x1] >= c) {
					hidden = true;
					scenic *= x - x1;
					break;
				}
			}

			if (!hidden) {
				vis = true;
				scenic *= x;
			}

			hidden = false;
			for (int64_t x1 = x + 1; x1 < map[y].size(); x1++) {
				if (map[y][x1] >= c) {
					hidden = true;
					scenic *= x1 - x;
					break;
				}
			}

			if (!hidden) {
				vis = true;
				scenic *= map[y].size() - x - 1;
			}

			hidden = false;
			for (int64_t y1 = y - 1; y1 >= 0; y1--) {
				if (map[y1][x] >= c) {
					hidden = true;
					scenic *= y - y1;
					break;
				}
			}

			if (!hidden) {
				vis = true;
				scenic *= y;
			}

			hidden = false;
			for (int64_t y1 = y + 1; y1 < map.size(); y1++) {
				if (map[y1][x] >= c) {
					hidden = true;
					scenic *= y1 - y;
					break;
				}
			}

			if (!hidden) {
				vis = true;
				scenic *= map.size() - y - 1;
			}

			if (vis) {
				visible++;
			}

			if (best_scenic < scenic) {
				best_scenic = scenic;
			}
		}
	}

	return {std::to_string(visible), std::to_string(best_scenic)};
}

bool d8c = aoc::registerCombined(8, &day8combined);
