/*
 * Day21.cpp
 *
 *  Created on: 21.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <iostream>
#include <map>
#include <stack>

template<>
void DayRunner<21>::solve(std::ifstream input) {
	std::string line;

	uint8_t player_1_start_pos = 0;
	uint8_t player_2_start_pos;
	while (std::getline(input, line)) {
		if (player_1_start_pos == 0) {
			player_1_start_pos = std::stoi(line.substr(line.find_last_of(' ')));
		} else {
			player_2_start_pos = std::stoi(line.substr(line.find_last_of(' ')));
			break;
		}
	}

	uint8_t dice = 1;
	uint16_t player_1_points = 0;
	uint16_t player_2_points = 0;
	uint16_t player_1_pos = player_1_start_pos;
	uint16_t player_2_pos = player_2_start_pos;
	uint16_t rolls = 0;
	while (player_1_points < 1000 && player_1_points < 1000) {
		for (uint8_t i = 0; i < 3; i++) {
			player_1_pos += dice;
			dice = dice % 100 + 1;
		}

		player_1_pos = (player_1_pos - 1) % 10 + 1;
		player_1_points += player_1_pos;

		if (player_1_points >= 1000) {
			rolls += 3;
			break;
		}

		for (uint8_t i = 0; i < 3; i++) {
			player_2_pos += dice;
			dice = dice % 100 + 1;
		}

		player_2_pos = (player_2_pos - 1) % 10 + 1;
		player_2_points += player_2_pos;

		rolls += 6;
	}

	uint32_t result = rolls * std::min(player_1_points, player_2_points);

	std::cout
			<< "The number of die rolls multiplied by the losing players score is "
			<< result << '.' << std::endl;

	uint8_t dirac_values[7] { 0 };
	for (uint8_t i = 0; i < 3; i++) {
		for (uint8_t j = 0; j < 3; j++) {
			for (uint8_t k = 0; k < 3; k++) {
				dirac_values[i + j + k]++;
			}
		}
	}

	uint64_t player_1_wins = 0;
	uint64_t player_2_wins = 0;
	std::stack<std::pair<uint16_t, uint16_t>> universes;
	std::map<std::pair<uint16_t, uint16_t>, std::pair<uint64_t, uint64_t>> cache;
	universes.push( { player_1_start_pos, player_2_start_pos });
	std::pair<uint16_t, uint16_t> current, next;
	bool found;
	while (!universes.empty()) {
		current = universes.top();
		found = true;

		player_1_wins = player_2_wins = 0;
		for (uint8_t i = 2; i < 9; i++) {
			player_1_pos = current.first & 0xF;
			player_1_pos = (player_1_pos + i) % 10 + 1;
			player_1_points = (current.first >> 4) & 0x1F;
			player_1_points += player_1_pos;

			if (player_1_points >= 21) {
				player_1_wins += dirac_values[i - 2];
			} else {
				for (uint8_t j = 2; j < 9; j++) {
					player_2_pos = current.second & 0xF;
					player_2_pos = (player_2_pos + j) % 10 + 1;
					player_2_points = (current.second >> 4) & 0x1F;
					player_2_points += player_2_pos;
					next = { player_1_pos + (player_1_points << 4), player_2_pos + (player_2_points << 4) };

					if (player_2_points >= 21) {
						player_2_wins += dirac_values[i - 2] * dirac_values[j - 2];
					} else if (cache.count(next) > 0) {
						player_1_wins += cache[next].first * dirac_values[i - 2] * dirac_values[j - 2];
						player_2_wins += cache[next].second * dirac_values[i - 2] * dirac_values[j - 2];
					} else {
						universes.push(next);
						found = false;
					}
				}
			}
		}

		if (found) {
			cache[current] = { player_1_wins, player_2_wins };
			universes.pop();
		}
	}

	std::cout << "The player that won in more universes won in "
			<< std::max(player_1_wins, player_2_wins)
			<< " universes." << std::endl;
}
