/*
 * Day13.cpp
 *
 *  Created on: Dec 13, 2022
 *      Author: ToMe25
 */

#include "Day13.h"
#include <algorithm>

bool aoc::compare_lists(const std::string &first, const std::string &second) {
	std::string::const_iterator lli = first.begin();
	std::string::const_iterator li = second.begin();
	while (lli < first.end() && li < second.end()) {
		char lc = *lli;
		char c = *li;
		if (c == lc) {
			lli++;
			li++;
			continue;
		}

		if (lc == ']') {
			return true;
		} else if (c == ']') {
			return false;
		} else if (lc == '[') {
			uint8_t layers = 0;
			while (*lli == '[') {
				lli++;
				layers++;
			}
			lc = *lli;

			if (lc == ']') {
				return true;
			} else if (c == ']') {
				return false;
			} else if (std::isdigit(*(li + 1)) && !std::isdigit(*(lli + 1))) {
				return true;
			} else if (!std::isdigit(*(li + 1)) && std::isdigit(*(lli + 1))) {
				return false;
			} else if (lc < c) {
				return true;
			} else if (lc > c) {
				return false;
			} else if (lc == c) {
				lli++;
				li++;
				if (std::isdigit(*li) && std::isdigit(*lli)) {
					if (*lli < *li) {
						return true;
					} else if (*lli > *li) {
						return false;
					} else if (*lli == *li && *(lli + 1) == ']') {
						lli += layers + 1;
						li++;
						continue;
					}
				} else if (*lli == ']') {
					lli += layers + 1;
					li++;
					continue;
				}
			}
		} else if (c == '[') {
			uint8_t layers = 0;
			while (*li == '[') {
				li++;
				layers++;
			}
			c = *li;

			if (lc == ']') {
				return true;
			} else if (c == ']') {
				return false;
			} else if (std::isdigit(*(li + 1)) && !std::isdigit(*(lli + 1))) {
				return true;
			} else if (!std::isdigit(*(li + 1)) && std::isdigit(*(lli + 1))) {
				return false;
			} else if (lc < c) {
				return true;
			} else if (lc > c) {
				return false;
			} else if (lc == c) {
				lli++;
				li++;
				if (std::isdigit(*li) && std::isdigit(*lli)) {
					if (*lli < *li) {
						return true;
					} else if (*lli > *li) {
						return false;
					} else if (*lli == *li && *(li + 1) == ']') {
						lli++;
						li += layers + 1;
						continue;
					}
				} else if (*li == ']') {
					lli++;
					li += layers + 1;
					continue;
				}
			}
		} else if (std::isdigit(lc) && std::isdigit(c)) {
			// There aren't any three digit numbers.
			// Also if both numbers have two digits checking digit by digit is fine.
			if (std::isdigit(*(li + 1)) && !std::isdigit(*(lli + 1))) {
				return true;
			} else if (!std::isdigit(*(li + 1)) && std::isdigit(*(lli + 1))) {
				return false;
			} else if (lc < c) {
				return true;
			} else if (lc > c) {
				return false;
			}
		} else if (lc == ',') {
			return true;
		} else if (c == ',') {
			return false;
		} else if (!std::isdigit(lc)) {
			std::cerr << "Line " << first << " contained unrecognized char "
					<< lc << '.' << std::endl;
		} else {
			std::cerr << "Line " << second << " contained unrecognized char "
					<< c << '.' << std::endl;
		}
	}
	return false;
}

std::string day13part1(std::ifstream input) {
	std::string last_line;
	std::string line;
	uint32_t pair_idx = 0;
	std::vector<uint32_t> correct_pairs;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		if (last_line.length() == 0) {
			last_line = line;
			pair_idx++;
		} else {
			if (aoc::compare_lists(last_line, line)) {
				correct_pairs.push_back(pair_idx);
			}
			last_line = "";
		}
	}

	uint64_t idx_sum = 0;
	for (uint32_t i : correct_pairs) {
		idx_sum += i;
	}
	return std::to_string(idx_sum);
}

std::string day13part2(std::ifstream input) {
	std::vector<std::string> lines;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() > 0) {
			lines.push_back(line);
		}
	}

	lines.push_back("[[2]]");
	lines.push_back("[[6]]");
	std::sort(lines.begin(), lines.end(), &aoc::compare_lists);

	if (std::count(lines.begin(), lines.end(), "[[2]]") == 0) {
		std::cerr << "Couldn't find the first divider packet." << std::endl;
		return "error";
	}
	const size_t first = std::find(lines.begin(), lines.end(), "[[2]]")
			- lines.begin() + 1;

	if (std::count(lines.begin(), lines.end(), "[[6]]") == 0) {
		std::cerr << "Couldn't find the first divider packet." << std::endl;
		return "error";
	}
	const size_t second = std::find(lines.begin(), lines.end(), "[[6]]")
			- lines.begin() + 1;

	return std::to_string(first * second);
}

bool d13p1 = aoc::registerPart1(13, &day13part1);
bool d13p2 = aoc::registerPart2(13, &day13part2);
