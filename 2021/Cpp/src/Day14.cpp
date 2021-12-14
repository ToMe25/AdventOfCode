/*
 * Day14.cpp
 *
 *  Created on: 14.12.2021
 *      Author: ToMe25
 */

#include "Day14.h"
#include <iostream>

template <>
void DayRunner<14>::solve(std::ifstream input) {
	std::string line;
	std::map<std::pair<char, char>, uint64_t> polymer;
	std::map<std::pair<char, char>, char> insertions;
	char first = 0;
	char last = 0;
	while (std::getline(input, line)) {
		if (polymer.empty()) {
			for (char c : line) {
				if (last != 0) {
					polymer[{ last, c }]++;
				} else {
					first = c;
				}

				last = c;
			}
		} else {
			insertions[{ line[0], line[1] }] = line[6];
		}
	}

	uint64_t result = expand_and_count(polymer, first, insertions, 10);

	std::cout << "The part one result is " << result << '.' << std::endl;

	result = expand_and_count(polymer, first, insertions, 40);

	std::cout << "The part two result is " << result << '.' << std::endl;
}

uint64_t expand_and_count(std::map<std::pair<char, char>, uint64_t> polymer,
		const char first,
		const std::map<std::pair<char, char>, char> expansions,
		const uint8_t times) {
	std::map<std::pair<char, char>, uint64_t> polymer_cpy;
	std::map<std::pair<char, char>, char> expansions_cpy(expansions);

	for (uint8_t i = 0; i < times; i++) {
		polymer_cpy = std::map<std::pair<char, char>, uint64_t>();
		for (std::pair<std::pair<char, char>, uint64_t> entry : polymer) {
			if (expansions.find(entry.first) != expansions.end()) {
				polymer_cpy[{ entry.first.first, expansions_cpy[entry.first] }] += entry.second;
				polymer_cpy[{ expansions_cpy[entry.first], entry.first.second }] += entry.second;
			} else {
				polymer_cpy[entry.first] += entry.second;
			}
		}

		polymer = polymer_cpy;
	}

	std::map<char, uint64_t> counts = std::map<char, uint64_t>();
	for (std::pair<std::pair<char, char>, uint64_t> entry : polymer) {
		counts[entry.first.second] += entry.second;
	}

	counts[first]++;

	uint64_t min = 0;
	uint64_t max = 0;
	for (std::pair<char, uint64_t> entry : counts) {
		if (min == 0 || min > entry.second) {
			min = entry.second;
		}

		if (max < entry.second) {
			max = entry.second;
		}
	}

	return max - min;
}
