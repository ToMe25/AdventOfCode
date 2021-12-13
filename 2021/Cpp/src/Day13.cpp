/*
 * Day13.cpp
 *
 *  Created on: 13.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <iostream>
#include <vector>

template <>
void DayRunner<13>::solve(std::ifstream input) {
	std::vector<std::vector<bool>> map(2000);
	std::vector<std::vector<bool>> new_map;
	std::string line;
	uint16_t points = 0;
	while (input >> line) {
		size_t comma_pos = line.find(',');
		if (comma_pos != std::string::npos) {
			uint16_t x = std::stoi(line.substr(0, comma_pos));
			uint16_t y = std::stoi(line.substr(comma_pos + 1));
			map[y].resize(2000);
			map[y][x] = true;
		} else {
			size_t equals_pos = line.find('=');
			if (equals_pos != std::string::npos) {
				char axis = line[0];
				uint16_t pos = std::stoi(line.substr(equals_pos + 1));
				new_map = std::vector<std::vector<bool>>(
						axis == 'x' ? map.size() : pos,
						std::vector<bool>(axis == 'x' ? pos : map[0].size(),
								false));
				for (size_t y = 0; y < map.size(); y++) {
					for (size_t x = 0; x < map[y].size(); x++) {
						if (map[y][x]) {
							if (axis == 'x') {
								if (x < pos) {
									new_map[y][x] = map[y][x];
								} else {
									new_map[y][pos * 2 - x] = map[y][x];
								}
							} else {
								if (y < pos) {
									new_map[y][x] = map[y][x];
								} else {
									new_map[pos * 2 - y][x] = map[y][x];
								}
							}
						}
					}
				}
				map = new_map;

				if (points == 0) {
					for (size_t y = 0; y < map.size(); y++) {
						for (size_t x = 0; x < map[y].size(); x++) {
							if (map[y][x]) {
								points++;
							}
						}
					}
				}
			}
		}
	}

	std::cout << "After one fold there are " << points << " points." << std::endl;

	std::cout << "The result after all folds is:" << std::endl;
	for (size_t y = 0; y < map.size(); y++) {
		for (size_t x = 0; x < map[y].size(); x++) {
			std::cout << (map[y][x] ? '#' : '.');
		}
		std::cout << std::endl;
	}
}
