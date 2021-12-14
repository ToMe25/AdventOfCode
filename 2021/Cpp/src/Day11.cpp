/*
 * Day11.cpp
 *
 *  Created on: 11.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <algorithm>
#include <cmath>
#include <iostream>
#include <list>
#include <vector>

template<>
void DayRunner<11>::solve(std::ifstream input) {
	const uint8_t MAP_SIZE = 10;

	uint8_t map[MAP_SIZE][MAP_SIZE];
	std::string line;
	uint8_t y = 0;
	while (input >> line) {
		for (uint8_t x = 0; x < MAP_SIZE; x++) {
			map[y][x] = line[x] - '0';
		}
		y++;
	}

	uint16_t flashes = 0;
	uint16_t run = 0;
	bool finished = false;
	std::list<std::pair<uint8_t, uint8_t>> stack;
	while (!finished) {
		for (uint8_t y = 0; y < MAP_SIZE; y++) {
			for (uint8_t x = 0; x < MAP_SIZE; x++) {
				map[y][x]++;
			}
		}

		for (uint8_t y = 0; y < MAP_SIZE; y++) {
			for (uint8_t x = 0; x < MAP_SIZE; x++) {
				if (map[y][x] > 9) {
					stack.push_back( { x, y });
					while (stack.size() > 0) {
						std::pair<uint8_t, uint8_t> current_pos = stack.front();
						stack.pop_front();
						if (run < 100) {
							flashes++;
						}

						map[current_pos.second][current_pos.first] = 0;
						for (uint8_t i = std::max(0,
								(int8_t) current_pos.second - 1);
								i < std::min((int) MAP_SIZE,
												current_pos.second + 2); i++) {
							for (uint8_t j = std::max(0,
									(int8_t) current_pos.first - 1);
									j < std::min((int) MAP_SIZE,
													current_pos.first + 2);
									j++) {
								if (map[i][j] != 0) {
									map[i][j]++;
									std::pair<uint8_t, uint8_t> pair(j, i);
									if (map[i][j] > 9
											&& std::find(stack.begin(),
													stack.end(), pair)
													== stack.end()) {
										stack.push_back(pair);
									}
								}
							}
						}
					}
				}
			}
		}

		finished = true;
		for (uint8_t y = 0; y < MAP_SIZE; y++) {
			for (uint8_t x = 0; x < MAP_SIZE; x++) {
				if (map[y][x] != 0) {
					finished = false;
					break;
				}
			}

			if (!finished) {
				break;
			}
		}
		run++;
	}

	std::cout << "After 100 steps there were " << flashes << " flashes." << std::endl;
	std::cout << "It took " << run
			<< " steps for all octopuses to flash simultaneously." << std::endl;
}
