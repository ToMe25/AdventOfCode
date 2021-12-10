/*
 * Day9.cpp
 *
 *  Created on: 10.12.2021
 *      Author: ToMe25
 */

#include "Day9.h"
#include <algorithm>
#include <iostream>
#include <vector>

template <>
void DayRunner<9>::solve(std::ifstream input) {
	uint8_t map[100][100];

	std::string line;
	uint8_t x = 0;
	uint8_t y = 0;
	while (input >> line) {
		for (char c : line) {
			map[y][x++] = c - '0';
		}
		x = 0;
		y++;
	}

	uint32_t risk_levels_sum = 0;
	uint8_t here;
	for (uint8_t y = 0; y < 100; y++) {
		for (uint8_t x = 0; x < 100; x++) {
			here = map[y][x];
			bool lowest = true;
			if (x > 0 && map[y][x - 1] <= here) {
				lowest = false;
			}
			if (x < 99 && map[y][x + 1] <= here) {
				lowest = false;
			}

			if (y > 0 && map[y - 1][x] <= here) {
				lowest = false;
			}
			if (y < 99 && map[y + 1][x] <= here) {
				lowest = false;
			}

			if (lowest) {
				risk_levels_sum += 1 + here;
			}
		}
	}

	std::cout << "The sum of all risk levels is " << risk_levels_sum << '.' << std::endl;

	std::set<std::pair<uint8_t, uint8_t>> checked;
	std::vector<uint8_t> basin_sizes;
	for (uint8_t y = 0; y < 100; y++) {
		for (uint8_t x = 0; x < 100; x++) {
			if (map[y][x] != 9 && checked.find({x, y}) == checked.end()) {
				const std::set<std::pair<uint8_t, uint8_t>> basin = recursive_check(x, y, checked, map);
				if (basin.size() > 0) {
					checked.insert(basin.begin(), basin.end());
					basin_sizes.push_back(basin.size());
				}
			}
		}
	}

	std::sort(basin_sizes.begin(), basin_sizes.end());
	uint64_t result = basin_sizes[basin_sizes.size() - 1]
			* basin_sizes[basin_sizes.size() - 2]
			* basin_sizes[basin_sizes.size() - 3];

	std::cout << "The product of the top three basin sizes is " << result << '.' << std::endl;
}

const std::set<std::pair<uint8_t, uint8_t>> recursive_check(const uint8_t x_pos,
		const uint8_t y_pos,
		const std::set<std::pair<uint8_t, uint8_t>> checked,
		const uint8_t map[100][100]) {
	std::set<std::pair<uint8_t, uint8_t>> found;
	if (checked.find({x_pos, y_pos}) != checked.end()) {
		return found;
	}

	if (map[y_pos][x_pos] == 9) {
		return found;
	}

	std::set<std::pair<uint8_t, uint8_t>> checked_cpy(checked);
	found.insert({x_pos, y_pos});
	checked_cpy.insert({x_pos, y_pos});

	if (x_pos > 0 && map[y_pos][x_pos - 1] != 9 && checked.find({x_pos - 1, y_pos}) == checked.end()) {
		std::set<std::pair<uint8_t, uint8_t>> neighbor = recursive_check(x_pos - 1, y_pos, checked_cpy, map);
		found.insert(neighbor.begin(), neighbor.end());
		checked_cpy.insert(neighbor.begin(), neighbor.end());
	}

	if (x_pos < 99 && map[y_pos][x_pos + 1] != 9 && checked.find({x_pos + 1, y_pos}) == checked.end()) {
		std::set<std::pair<uint8_t, uint8_t>> neighbor = recursive_check(x_pos + 1, y_pos, checked_cpy, map);
		found.insert(neighbor.begin(), neighbor.end());
		checked_cpy.insert(neighbor.begin(), neighbor.end());
	}

	if (y_pos > 0 && map[y_pos - 1][x_pos] != 9 && checked.find({x_pos, y_pos - 1}) == checked.end()) {
		std::set<std::pair<uint8_t, uint8_t>> neighbor = recursive_check(x_pos, y_pos - 1, checked_cpy, map);
		found.insert(neighbor.begin(), neighbor.end());
		checked_cpy.insert(neighbor.begin(), neighbor.end());
	}

	if (y_pos < 99 && map[y_pos + 1][x_pos] != 9 && checked.find({x_pos, y_pos + 1}) == checked.end()) {
		std::set<std::pair<uint8_t, uint8_t>> neighbor = recursive_check(x_pos, y_pos + 1, checked_cpy, map);
		found.insert(neighbor.begin(), neighbor.end());
		checked_cpy.insert(neighbor.begin(), neighbor.end());
	}

	return found;
}
