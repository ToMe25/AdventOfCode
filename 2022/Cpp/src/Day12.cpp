/*
 * Day12.cpp
 *
 *  Created on: Dec 12, 2022
 *      Author: ToMe25
 */

#include "Main.h"
#include <map>

std::string day12part1(std::ifstream input) {
	std::vector<std::vector<uint8_t>> heights;
	std::string line;
	std::pair<uint8_t, uint8_t> start_pos;
	std::pair<uint8_t, uint8_t> end_pos;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		std::vector<uint8_t> row;
		for (char c : line) {
			if (c == 'S') {
				start_pos.second = heights.size();
				start_pos.first = row.size();
				row.push_back(0);
			} else if (c == 'E') {
				end_pos.second = heights.size();
				end_pos.first = row.size();
				row.push_back(25);
			} else {
				row.push_back(c - 'a');
			}
		}
		heights.push_back(row);
	}

	std::vector<std::pair<uint8_t, uint8_t>> stack;
	std::map<std::pair<uint8_t, uint8_t>, uint16_t> costs;
	stack.push_back(start_pos);
	costs[start_pos] = 0;
	while (stack.size() > 0) {
		const std::pair<uint8_t, uint8_t> current_pos = stack.back();
		stack.pop_back();
		const uint16_t current_cost = costs[current_pos];
		const uint8_t current_height = heights[current_pos.second][current_pos.first];
		if (costs.count(end_pos) > 0 && current_cost >= costs[end_pos]) {
			continue;
		}

		if (current_pos.first <= end_pos.first && current_pos.first > 0) {
			if (heights[current_pos.second][current_pos.first - 1] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first - 1, current_pos.second);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}

		if (current_pos.first >= end_pos.first && current_pos.first < heights[current_pos.second].size() - 1) {
			if (heights[current_pos.second][current_pos.first + 1] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first + 1, current_pos.second);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}

		if (current_pos.second <= end_pos.second && current_pos.second > 0) {
			if (heights[current_pos.second - 1][current_pos.first] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first, current_pos.second - 1);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}

		if (current_pos.second >= end_pos.second && current_pos.second < heights.size() - 1) {
			if (heights[current_pos.second + 1][current_pos.first] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first, current_pos.second + 1);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}

		if (current_pos.first > end_pos.first) {
			if (heights[current_pos.second][current_pos.first - 1] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first - 1, current_pos.second);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}

		if (current_pos.first < end_pos.first) {
			if (heights[current_pos.second][current_pos.first + 1] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first + 1, current_pos.second);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}

		if (current_pos.second > end_pos.second) {
			if (heights[current_pos.second - 1][current_pos.first] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first, current_pos.second - 1);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}

		if (current_pos.second < end_pos.second) {
			if (heights[current_pos.second + 1][current_pos.first] <= current_height + 1) {
				const std::pair<uint8_t, uint8_t> new_pos(current_pos.first, current_pos.second + 1);
				if (costs.count(new_pos) == 0 || current_cost + 1 < costs[new_pos]) {
					if (costs.count(end_pos) == 0 || costs[end_pos] > current_cost + 1) {
						costs[new_pos] = current_cost + 1;
						if (new_pos == end_pos) {
							continue;
						} else if (std::count(stack.rbegin(), stack.rend(), new_pos) == 0) {
							stack.push_back(new_pos);
						}
					}
				}
			}
		}
	}

	return std::to_string(costs[end_pos]);
}

bool d12p1 = aoc::registerPart1(12, &day12part1);
