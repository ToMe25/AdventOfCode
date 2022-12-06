/*
 * Day6.cpp
 *
 *  Created on: Dec 6, 2022
 *      Author: ToMe25
 */

#include "Main.h"

std::string day6part1(std::ifstream input) {
	std::string line;
	while (input >> line && line.length() == 0);

	if (line.length() == 0) {
		std::cerr << "Couldn't read a non empty input line." << std::endl;
		return "error";
	}

	char buf[4] { 0 };
	int64_t result = -1;
	for (size_t i = 0; i < line.length(); i++) {
		buf[i % 4] = line[i];
		if (i < 3) {
			continue;
		}

		bool failed = false;
		for (uint8_t j = 0; j < 4; j++) {
			for (uint8_t k = 0; k < 4; k++) {
				if (k != j && buf[j] == buf[k]) {
					failed = true;
					break;
				}
			}

			if (failed) {
				break;
			}
		}

		if (!failed) {
			result = i + 1;
			break;
		}
	}

	return std::to_string(result);
}

std::string day6part2(std::ifstream input) {
	std::string line;
	while (input >> line && line.length() == 0);

	if (line.length() == 0) {
		std::cerr << "Couldn't read a non empty input line." << std::endl;
		return "error";
	}

	char buf[14] { 0 };
	int64_t result = -1;
	for (size_t i = 0; i < line.length(); i++) {
		buf[i % 14] = line[i];
		if (i < 13) {
			continue;
		}

		bool failed = false;
		for (uint8_t j = 0; j < 14; j++) {
			for (uint8_t k = 0; k < 14; k++) {
				if (k != j && buf[j] == buf[k]) {
					failed = true;
					break;
				}
			}

			if (failed) {
				break;
			}
		}

		if (!failed) {
			result = i + 1;
			break;
		}
	}

	return std::to_string(result);
}

bool p6d1 = aoc::registerPart1(6, &day6part1);
bool p6d2 = aoc::registerPart2(6, &day6part2);
