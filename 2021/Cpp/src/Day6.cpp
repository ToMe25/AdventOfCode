/*
 * Day6.cpp
 *
 *  Created on: 06.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <algorithm>
#include <iostream>
#include <sstream>

template <>
void DayRunner<6>::solve(std::ifstream input) {
	std::string line;
	input >> line;
	std::replace(line.begin(), line.end(), ',', ' ');
	std::istringstream fishes_stream(line);

	uint16_t fish;
	uint64_t fishes[9] { 0 };
	while (fishes_stream >> fish) {
		fishes[fish]++;
	}

	uint8_t day = 0;
	uint64_t temp;
	for (uint8_t i = 0; i < 80; i++) {
		day = i % 7;
		temp = fishes[7];
		fishes[7] = fishes[8];
		fishes[8] = fishes[day];
		fishes[day] += temp;
	}

	uint64_t total = 0;
	for (uint8_t i = 0; i < 9; i++) {
		total += fishes[i];
	}

	std::cout << "After 80 days there are " << total << " lanternfishes." << std::endl;

	for (uint16_t i = 80; i < 256; i++) {
		day = i % 7;
		temp = fishes[7];
		fishes[7] = fishes[8];
		fishes[8] = fishes[day];
		fishes[day] += temp;
	}

	total = 0;
	for (uint8_t i = 0; i < 9; i++) {
		total += fishes[i];
	}

	std::cout << "After 256 days there are " << total << " lanternfishes." << std::endl;
}
