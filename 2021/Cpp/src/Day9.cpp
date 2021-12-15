/*
 * Day9.cpp
 *
 *  Created on: 10.12.2021
 *      Author: ToMe25
 */

#include "Day9.h"
#include <algorithm>
#include <iostream>
#include <list>
#include <vector>

template<>
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
			if (map[y][x] != 9 && checked.count( { x, y }) == 0) {
				const std::set<std::pair<uint8_t, uint8_t>> basin =
						recursive_check(x, y, checked, map);
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

	std::cout << "The product of the top three basin sizes is " << result << '.'
			<< std::endl;
}

const std::set<std::pair<uint8_t, uint8_t>> recursive_check(const uint8_t x_pos,
		const uint8_t y_pos,
		const std::set<std::pair<uint8_t, uint8_t>> checked,
		const uint8_t map[100][100]) {
	std::set<std::pair<uint8_t, uint8_t>> found;
	if (checked.count( { x_pos, y_pos }) > 0) {
		return found;
	}

	if (map[y_pos][x_pos] == 9) {
		return found;
	}

	std::set<std::pair<uint8_t, uint8_t>> checked_cpy(checked);
	std::list<std::pair<uint8_t, uint8_t>> to_check;
	found.insert( { x_pos, y_pos });
	checked_cpy.insert( { x_pos, y_pos });
	to_check.push_back( { x_pos, y_pos });

	while (to_check.size() > 0) {
		uint8_t x = to_check.front().first;
		uint8_t y = to_check.front().second;

		found.insert( { x, y });
		checked_cpy.insert( { x, y });

		if (x > 0 && map[y][x - 1] != 9
				&& checked_cpy.count( { x - 1, y }) == 0
				&& std::find(to_check.begin(), to_check.end(),
						std::pair<uint8_t, uint8_t>( { x - 1, y }))
						== to_check.end()) {
			to_check.push_back( { x - 1, y });
		}

		if (x < 99 && map[y][x + 1] != 9
				&& checked_cpy.count( { x + 1, y }) == 0
				&& std::find(to_check.begin(), to_check.end(),
						std::pair<uint8_t, uint8_t>( { x + 1, y }))
						== to_check.end()) {
			to_check.push_back( { x + 1, y });
		}

		if (y > 0 && map[y - 1][x] != 9
				&& checked_cpy.count( { x, y - 1 }) == 0
				&& std::find(to_check.begin(), to_check.end(),
						std::pair<uint8_t, uint8_t>( { x, y - 1 }))
						== to_check.end()) {
			to_check.push_back( { x, y - 1 });
		}

		if (y < 99 && map[y + 1][x] != 9
				&& checked_cpy.count( { x, y + 1 }) == 0
				&& std::find(to_check.begin(), to_check.end(),
						std::pair<uint8_t, uint8_t>( { x, y + 1 }))
						== to_check.end()) {
			to_check.push_back( { x, y + 1 });
		}

		to_check.pop_front();
	}

	return found;
}
