/*
 * Day14.cpp
 *
 *  Created on: Dec 14, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::string day14part1(std::ifstream input) {
	std::vector<std::array<uint16_t, 4>> lines;
	std::string line;
	uint16_t x_offset = UINT16_MAX;
	uint16_t x_max = 0;
	uint16_t y_max = 0;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		size_t comma_idx = line.find(',');
		size_t space_idx = line.find(' ');
		std::array<uint16_t, 4> arr;
		arr[2] = std::stoi(line.substr(0, comma_idx));
		arr[3] = std::stoi(
				line.substr(comma_idx + 1, space_idx - comma_idx - 1));
		line = line.substr(line.find(' ', space_idx + 1) + 1);
		comma_idx = line.find(',');
		space_idx = line.find(' ');

		if (arr[2] < x_offset) {
			x_offset = arr[2];
		}
		if (arr[2] > x_max) {
			x_max = arr[2];
		}
		if (arr[3] > y_max) {
			y_max = arr[3];
		}

		while (comma_idx != std::string::npos) {
			arr = { arr[2], arr[3], 0, 0 };
			arr[2] = std::stoi(line.substr(0, comma_idx));
			arr[3] = std::stoi(
					line.substr(comma_idx + 1,
							space_idx == std::string::npos ?
									space_idx : space_idx - comma_idx - 1));
			if (space_idx != std::string::npos) {
				line = line.substr(line.find(' ', space_idx + 1) + 1);
			} else {
				line = "";
			}

			lines.push_back(arr);
			comma_idx = line.find(',');
			space_idx = line.find(' ');

			if (arr[2] < x_offset) {
				x_offset = arr[2];
			}
			if (arr[2] > x_max) {
				x_max = arr[2];
			}
			if (arr[3] > y_max) {
				y_max = arr[3];
			}
		}
	}

	bool map[y_max + 1][x_max - x_offset + 1] = {};
	for (std::array<uint16_t, 4> arr : lines) {
		for (uint16_t y = std::min(arr[1], arr[3]); y <= std::max(arr[1], arr[3]); y++) {
			for (uint16_t x = std::min(arr[0], arr[2]); x <= std::max(arr[0], arr[2]); x++) {
				map[y][x - x_offset] = true;
			}
		}
	}

	bool vanished = false;
	uint16_t count = 0;
	while (!vanished) {
		std::pair<uint16_t, uint16_t> pos = { 500 - x_offset, 0 };
		count++;
		while (true) {
			if (pos.second == y_max) {
				vanished = true;
				break;
			} else if (!map[pos.second + 1][pos.first]) {
				pos.second++;
			} else if (pos.first == 0) {
				vanished = true;
				break;
			} else if (!map[pos.second + 1][pos.first - 1]) {
				pos.second++;
				pos.first--;
			} else if (pos.first == x_max - x_offset) {
				vanished = true;
				break;
			} else if (!map[pos.second + 1][pos.first + 1]) {
				pos.second++;
				pos.first++;
			} else {
				map[pos.second][pos.first] = true;
				break;
			}
		}
	}

	return std::to_string(count - 1);
}

std::string day14part2(std::ifstream input) {
	std::vector<std::array<uint16_t, 4>> lines;
	std::string line;
	uint16_t x_offset = UINT16_MAX;
	uint16_t x_max = 0;
	uint16_t y_max = 0;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		size_t comma_idx = line.find(',');
		size_t space_idx = line.find(' ');
		std::array<uint16_t, 4> arr;
		arr[2] = std::stoi(line.substr(0, comma_idx));
		arr[3] = std::stoi(
				line.substr(comma_idx + 1, space_idx - comma_idx - 1));
		line = line.substr(line.find(' ', space_idx + 1) + 1);
		comma_idx = line.find(',');
		space_idx = line.find(' ');

		if (arr[2] < x_offset) {
			x_offset = arr[2];
		}
		if (arr[2] > x_max) {
			x_max = arr[2];
		}
		if (arr[3] > y_max) {
			y_max = arr[3];
		}

		while (comma_idx != std::string::npos) {
			arr = { arr[2], arr[3], 0, 0 };
			arr[2] = std::stoi(line.substr(0, comma_idx));
			arr[3] = std::stoi(
					line.substr(comma_idx + 1,
							space_idx == std::string::npos ?
									space_idx : space_idx - comma_idx - 1));
			if (space_idx != std::string::npos) {
				line = line.substr(line.find(' ', space_idx + 1) + 1);
			} else {
				line = "";
			}

			lines.push_back(arr);
			comma_idx = line.find(',');
			space_idx = line.find(' ');

			if (arr[2] < x_offset) {
				x_offset = arr[2];
			}
			if (arr[2] > x_max) {
				x_max = arr[2];
			}
			if (arr[3] > y_max) {
				y_max = arr[3];
			}
		}
	}

	// Adjust for infinite floor.
	y_max += 2;
	x_offset -= y_max;
	x_max += y_max;
	lines.push_back( { x_offset, y_max, x_max, y_max });
	bool map[y_max + 1][x_max - x_offset + 1] = {};
	for (std::array<uint16_t, 4> arr : lines) {
		for (uint16_t y = std::min(arr[1], arr[3]); y <= std::max(arr[1], arr[3]); y++) {
			for (uint16_t x = std::min(arr[0], arr[2]); x <= std::max(arr[0], arr[2]); x++) {
				map[y][x - x_offset] = true;
			}
		}
	}

	bool blocked = false;
	uint16_t count = 0;
	while (!blocked) {
		std::pair<uint16_t, uint16_t> pos = { 500 - x_offset, 0 };
		count++;
		while (true) {
			if (!map[pos.second + 1][pos.first]) {
				pos.second++;
			} else if (!map[pos.second + 1][pos.first - 1]) {
				pos.second++;
				pos.first--;
			} else if (!map[pos.second + 1][pos.first + 1]) {
				pos.second++;
				pos.first++;
			} else {
				map[pos.second][pos.first] = true;
				if (pos.second == 0) {
					blocked = true;
				}
				break;
			}
		}
	}

	return std::to_string(count);
}

bool d14p1 = aoc::registerPart1(14, &day14part1);
bool d14p2 = aoc::registerPart2(14, &day14part2);
