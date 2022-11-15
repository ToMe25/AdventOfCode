/*
 * Day23.cpp
 *
 *  Created on: 20.06.2022
 *      Author: ToMe25
 */

#include "Day23.h"
#include <algorithm>
#include <cmath>
#include <iostream>
#include <stack>
#include <unordered_map>

template<>
void DayRunner<23>::solve(std::ifstream input) {
	const uint8_t burrow_count = 4;
	const std::string part_2_burrows[burrow_count] = { "DD", "CB", "BA", "AC" };

	std::string line;
	std::string start_tk;
	std::string burrows[burrow_count];
	while (std::getline(input, line)) {
		line.erase(remove(line.begin(), line.end(), '#'), line.end());
		line.erase(remove(line.begin(), line.end(), ' '), line.end());
		if (line.length() > 0) {
			if (start_tk.empty()) {
				start_tk += line + ':';
			} else {
				for (uint8_t i = 0; i < burrow_count; i++) {
					burrows[i] += line[i];
				}
			}
		}
	}

	for (uint8_t i = 0; i < burrow_count; i++) {
		start_tk += burrows[i] + ':';
	}

	uint32_t cost = get_min_cost(start_tk);

	std::cout << "The cheapest way to order all part 1 amphipods has a cost of "
			<< cost << '.' << std::endl;

	start_tk = start_tk.substr(0, start_tk.find(':') + 1);

	for (uint8_t i = 0; i < burrow_count; i++) {
		start_tk += burrows[i][0] + part_2_burrows[i] + burrows[i][1] + ':';
	}

	cost = get_min_cost(start_tk);

	std::cout << "The cheapest way to order all part 2 amphipods has a cost of "
			<< cost << '.' << std::endl;
}

uint32_t get_min_cost(std::string token) {
	const uint8_t valid_temp_spots[] = { 0, 1, 3, 5, 7, 9, 10 };
	const uint8_t burrow_count = std::count(token.begin(), token.end(), ':') - 1;
	const size_t burrows_start = token.find(':');

	std::unordered_map<std::string, uint32_t> known;
	known[token] = 0;
	std::stack<std::string> checking;
	checking.push(token);

	std::string burrows[burrow_count];
	while (checking.size() > 0) {
		token = checking.top();
		checking.pop();
		uint32_t cost = known[token];

		size_t start = burrows_start + 1;
		for (uint8_t i = 0; i < burrow_count; i++) {
			size_t end = token.find(':', start);
			burrows[i] = token.substr(start, end - start);
			start = end + 1;
		}

		bool modified = false;
		for (size_t i = 0; i < burrows_start; i++) {
			char c = token[i];
			if (c != '.') {
				uint8_t burrow = c - 'A';
				uint8_t target = (burrow + 1) * 2;
				bool free = true;
				for (uint8_t j = std::min((uint8_t) (i + 1), target);
						j < std::max((uint8_t) i, (uint8_t) (target + 1));
						j++) {

					if (token[j] != '.') {
						free = false;
						break;
					}
				}

				if (!free) {
					continue;
				}

				uint8_t bottom_free = 0;
				for (size_t j = 0; j < burrows[burrow].length(); j++) {
					if (burrows[burrow][j] == '.') {
						bottom_free = j;
					} else if (burrows[burrow][j] != c) {
						free = false;
						break;
					}
				}

				if (free) {
					std::pair<std::string, uint32_t> new_tk = move(token, cost,
							burrow, burrows[0].length(), i, bottom_free);
					if (known.find(new_tk.first) == known.end()
							|| known[new_tk.first] > new_tk.second) {
						known[new_tk.first] = new_tk.second;
						checking.push(new_tk.first);
						modified = true;
					}
				}
			}
		}

		if (modified) {
			continue;
		}

		for (uint8_t i = 0; i < burrow_count; i++) {
			for (uint8_t j = 0; j < burrows[i].length(); j++) {
				if (burrows[i][j] == '.') {
					continue;
				}

				bool mismatch = false;
				for (uint8_t k = j; k < burrows[i].length(); k++) {
					if (burrows[i][k] != i + 'A') {
						mismatch = true;
						break;
					}
				}

				if (!mismatch) {
					continue;
				}

				for (int8_t k = i + 1; k >= 0; k--) {
					uint8_t pos = valid_temp_spots[k];
					if (token[pos] != '.') {
						break;
					}

					std::pair<std::string, uint32_t> new_tk = move(token, cost,
							i, burrows[0].length(), pos, j);
					if (known.find(new_tk.first) == known.end()
							|| known[new_tk.first] > new_tk.second) {
						known[new_tk.first] = new_tk.second;
						checking.push(new_tk.first);
					}
				}

				for (uint8_t k = i + 2;
						k < sizeof(valid_temp_spots) / sizeof(uint8_t); k++) {
					uint8_t pos = valid_temp_spots[k];
					if (token[pos] != '.') {
						break;
					}

					std::pair<std::string, uint32_t> new_tk = move(token, cost,
							i, burrows[0].length(), pos, j);
					if (known.find(new_tk.first) == known.end()
							|| known[new_tk.first] > new_tk.second) {
						known[new_tk.first] = new_tk.second;
						checking.push(new_tk.first);
					}
				}
				break;
			}
		}
	}

	std::string solved_state;
	for (uint8_t i = 0; i < burrows_start; i++) {
		solved_state += '.';
	}
	solved_state += ':';

	for (uint8_t i = 0; i < burrow_count; i++) {
		for (uint8_t j = 0; j < burrows[i].length(); j++) {
			solved_state += 'A' + i;
		}
		solved_state += ':';
	}
	return known[solved_state];
}

std::pair<std::string, uint32_t> move(const std::string token,
		const uint32_t cost, const uint8_t burrow,
		const uint8_t burrow_length, const uint8_t pos, const uint8_t depth) {
	const size_t burrows_start = token.find(':');
	const size_t burrow_pos = burrows_start + burrow * (burrow_length + 1)
			+ depth + 1;

	std::string new_token = token;
	const uint8_t temp_char = new_token[burrow_pos];
	new_token[burrow_pos] = new_token[pos];
	new_token[pos] = temp_char;

	const uint8_t type = temp_char == '.' ? new_token[burrow_pos] : temp_char;
	uint16_t multi = std::max((uint16_t) 1,
			(uint16_t) pow((uint8_t) 10, (uint8_t) (type - 'A')));
	uint32_t new_cost = cost
			+ (abs((burrow + 1) * 2 - pos) + depth + 1) * multi;

	return {new_token, new_cost};
}
