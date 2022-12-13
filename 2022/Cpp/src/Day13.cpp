/*
 * Day13.cpp
 *
 *  Created on: Dec 13, 2022
 *      Author: ToMe25
 */

#include "Main.h"

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
			continue;
		}

		std::string::iterator lli = last_line.begin();
		std::string::iterator li = line.begin();
		while (lli < last_line.end() && li < line.end()) {
			char lc = *lli;
			char c = *li;
			if (c == lc) {
				lli++;
				li++;
				continue;
			}

			if (lc == ']') {
				correct_pairs.push_back(pair_idx);
			} else if (c == ']') {
				// Do nothing
			} else if (lc == '[') {
				uint8_t layers = 0;
				while (*lli == '[') {
					lli++;
					layers++;
				}
				lc = *lli;

				if (lc == ']') {
					correct_pairs.push_back(pair_idx);
				} else if (c == ']') {
					// Do nothing
				} else if (std::isdigit(*(li + 1))
						&& !std::isdigit(*(lli + 1))) {
					correct_pairs.push_back(pair_idx);
				} else if (!std::isdigit(*(li + 1))
						&& std::isdigit(*(lli + 1))) {
					// Do nothing
				} else if (lc < c) {
					correct_pairs.push_back(pair_idx);
				} else if (lc == c) {
					lli++;
					li++;
					if (std::isdigit(*li) && std::isdigit(*lli)) {
						if (*lli < *li) {
							correct_pairs.push_back(pair_idx);
						} else if (*lli > *li) {
							// Do nothing
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
					correct_pairs.push_back(pair_idx);
				} else if (c == ']') {
					// Do nothing
				} else if (std::isdigit(*(li + 1)) && !std::isdigit(*(lli + 1))) {
					correct_pairs.push_back(pair_idx);
				} else if (!std::isdigit(*(li + 1)) && std::isdigit(*(lli + 1))) {
					// Do nothing
				} else if (lc < c) {
					correct_pairs.push_back(pair_idx);
				} else if (lc == c) {
					lli++;
					li++;
					if (std::isdigit(*li) && std::isdigit(*lli)) {
						if (*lli < *li) {
							correct_pairs.push_back(pair_idx);
						} else if (*lli > *li) {
							std::cout << last_line << std::endl;
							std::cout << line << std::endl << std::endl;
							// Do nothing
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
					correct_pairs.push_back(pair_idx);
				} else if (!std::isdigit(*(li + 1)) && std::isdigit(*(lli + 1))) {
					// Do nothing
				} else if (lc < c) {
					correct_pairs.push_back(pair_idx);
				}
			} else if (lc == ',') {
				correct_pairs.push_back(pair_idx);
			} else if (c == ',') {
				// Do nothing
			} else if (!std::isdigit(lc)) {
				std::cerr << "Line " << last_line
						<< " contained unrecognized char " << lc << '.'
						<< std::endl;
			} else {
				std::cerr << "Line " << line << " contained unrecognized char "
						<< c << '.' << std::endl;
			}
			break;
		}
		last_line = "";
	}

	uint64_t idx_sum = 0;
	for (uint32_t i : correct_pairs) {
		idx_sum += i;
	}
	return std::to_string(idx_sum);
}

bool d13p1 = aoc::registerPart1(13, &day13part1);
