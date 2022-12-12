/*
 * Day12.cpp
 *
 *  Created on: Dec 12, 2022
 *      Author: ToMe25
 */

#include "Day12.h"

template<class T1, class T2>
size_t std::hash<std::pair<T1, T2>>::operator ()(
		const std::pair<T1, T2> &p) const {
	return std::hash<T1> { }(p.first) ^ (std::hash<T2> { }(p.second) << 1);
}

bool aoc::check_position(const std::vector<std::vector<uint8_t>> &heights,
		const std::map<std::pair<uint8_t, uint8_t>, uint16_t> &costs,
		const uint8_t current_height, const uint16_t current_cost,
		const int16_t end_cost, const std::pair<uint8_t, uint8_t> &new_pos) {
	if (heights[new_pos.second][new_pos.first] < current_height - 1) {
		return false;
	}

	if (costs.count(new_pos) > 0 && costs.at(new_pos) <= current_cost + 1) {
		return false;
	}

	if (end_cost > -1 && end_cost <= current_cost + 1) {
		return false;
	}

	return true;
}

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
	stack.push_back(end_pos);
	costs[end_pos] = 0;
	while (stack.size() > 0) {
		const std::pair<uint8_t, uint8_t> current_pos = stack.back();
		stack.pop_back();
		const uint16_t current_cost = costs[current_pos];
		const uint8_t current_height =
				heights[current_pos.second][current_pos.first];
		if (costs.count(start_pos) > 0 && current_cost >= costs[start_pos]) {
			continue;
		}

		if (current_pos.first <= start_pos.first && current_pos.first > 0) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first - 1,
					current_pos.second);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.first >= start_pos.first
				&& current_pos.first < heights[current_pos.second].size() - 1) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first + 1,
					current_pos.second);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.second <= start_pos.second && current_pos.second > 0) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first,
					current_pos.second - 1);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.second >= start_pos.second
				&& current_pos.second < heights.size() - 1) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first,
					current_pos.second + 1);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.first > start_pos.first) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first - 1,
					current_pos.second);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.first < start_pos.first) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first + 1,
					current_pos.second);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.second > start_pos.second) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first,
					current_pos.second - 1);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.second < start_pos.second) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first,
					current_pos.second + 1);
			if (aoc::check_position(heights, costs, current_height,
					current_cost,
					costs.count(start_pos) == 0 ? -1 : costs.at(start_pos),
					new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (new_pos == start_pos) {
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}
	}

	return std::to_string(costs[start_pos]);
}

std::string day12part2(std::ifstream input) {
	std::vector<std::vector<uint8_t>> heights;
	std::string line;
	std::pair<uint8_t, uint8_t> end_pos;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		std::vector<uint8_t> row;
		for (char c : line) {
			if (c == 'S') {
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
	stack.push_back(end_pos);
	costs[end_pos] = 0;
	int16_t min_cost = -1;
	while (stack.size() > 0) {
		const std::pair<uint8_t, uint8_t> current_pos = stack.back();
		stack.pop_back();
		const uint16_t current_cost = costs[current_pos];
		const uint8_t current_height =
				heights[current_pos.second][current_pos.first];
		if (min_cost > 0 && current_cost >= min_cost) {
			continue;
		}

		if (current_pos.second > 0) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first,
					current_pos.second - 1);
			if (aoc::check_position(heights, costs, current_height,
					current_cost, min_cost, new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1;
					}
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.second < heights.size() - 1) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first,
					current_pos.second + 1);
			if (aoc::check_position(heights, costs, current_height,
					current_cost, min_cost, new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1;
					}
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.first > 0) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first - 1,
					current_pos.second);
			if (aoc::check_position(heights, costs, current_height,
					current_cost, min_cost, new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1;
					}
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}

		if (current_pos.first < heights[current_pos.second].size() - 1) {
			const std::pair<uint8_t, uint8_t> new_pos(current_pos.first + 1,
					current_pos.second);
			if (aoc::check_position(heights, costs, current_height,
					current_cost, min_cost, new_pos)) {
				costs[new_pos] = current_cost + 1;
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1;
					}
					continue;
				} else {
					stack.push_back(new_pos);
				}
			}
		}
	}

	return std::to_string(min_cost);
}

bool d12p1 = aoc::registerPart1(12, &day12part1);
bool d12p2 = aoc::registerPart2(12, &day12part2);
