/*
 * Day9.cpp
 *
 *  Created on: Dec 9, 2022
 *      Author: ToMe25
 */

#include "Main.h"
#include "set"

std::string day9part1(std::ifstream input) {
	std::set<std::pair<int32_t, int32_t>> visited_positions;
	std::string line;
	std::pair<int32_t, int32_t> head_pos = { 0, 0 };
	std::pair<int32_t, int32_t> tail_pos = head_pos;
	visited_positions.insert(tail_pos);
	while (std::getline(input, line)) {
		if (line.length() >= 3) {
			const uint8_t count = std::stoi(line.substr(2));
			for (uint8_t i = 0; i < count; i++) {
				switch (line[0]) {
				case 'U':
					head_pos.second++;
					break;
				case 'D':
					head_pos.second--;
					break;
				case 'L':
					head_pos.first--;
					break;
				case 'R':
					head_pos.first++;
					break;
				}

				bool neighbor = false;
				for (int8_t x = -1; x < 2; x++) {
					for (int8_t y = -1; y < 2; y++) {
						if (head_pos.first == tail_pos.first + x
								&& head_pos.second == tail_pos.second + y) {
							neighbor = true;
							break;
						}
					}

					if (neighbor) {
						break;
					}
				}

				if (!neighbor) {
					if (head_pos.first > tail_pos.first) {
						tail_pos.first++;
					} else if (head_pos.first < tail_pos.first) {
						tail_pos.first--;
					}

					if (head_pos.second > tail_pos.second) {
						tail_pos.second++;
					} else if (head_pos.second < tail_pos.second) {
						tail_pos.second--;
					}

					visited_positions.insert(tail_pos);
				}
			}
		}
	}

	return std::to_string(visited_positions.size());
}

std::string day9part2(std::ifstream input) {
	std::set<std::pair<int32_t, int32_t>> visited_positions;
	std::string line;
	std::pair<int32_t, int32_t> positions[10] = { { 0, 0 } };
	for (uint8_t i = 1; i < 10; i++) {
		positions[i] = positions[0];
	}
	visited_positions.insert(positions[9]);

	while (std::getline(input, line)) {
		if (line.length() >= 3) {
			const uint8_t count = std::stoi(line.substr(2));
			for (uint8_t i = 0; i < count; i++) {
				switch (line[0]) {
				case 'U':
					positions[0].second++;
					break;
				case 'D':
					positions[0].second--;
					break;
				case 'L':
					positions[0].first--;
					break;
				case 'R':
					positions[0].first++;
					break;
				}

				for (uint8_t j = 1; j < 10; j++) {
					bool neighbor = false;
					for (int8_t x = -1; x < 2; x++) {
						for (int8_t y = -1; y < 2; y++) {
							if (positions[j - 1].first == positions[j].first + x
									&& positions[j - 1].second == positions[j].second + y) {
								neighbor = true;
								break;
							}
						}

						if (neighbor) {
							break;
						}
					}

					if (!neighbor) {
						if (positions[j - 1].first > positions[j].first) {
							positions[j].first++;
						} else if (positions[j - 1].first < positions[j].first) {
							positions[j].first--;
						}

						if (positions[j - 1].second > positions[j].second) {
							positions[j].second++;
						} else if (positions[j - 1].second < positions[j].second) {
							positions[j].second--;
						}
					}
				}
				visited_positions.insert(positions[9]);
			}
		}
	}

	return std::to_string(visited_positions.size());
}

bool d9p1 = aoc::registerPart1(9, &day9part1);
bool d9p2 = aoc::registerPart2(9, &day9part2);
