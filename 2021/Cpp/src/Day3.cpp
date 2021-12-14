/*
 * Day3.cpp
 *
 *  Created on: 03.12.2021
 *      Author: ToMe25
 */

#include "Day3.h"
#include <iostream>

template<>
void DayRunner<3>::solve(std::ifstream input) {
	std::string line;
	std::vector<uint16_t> lines;
	while (input >> line) {
		lines.push_back(std::stoi(line, 0, 2));
	}

	uint16_t gamma = 0;
	uint16_t epsilon = 0;
	for (uint8_t i = 0; i < 12; i++) {
		if (count_bits(lines, i) > 0) {
			gamma |= 1 << (11 - i);
		} else {
			epsilon |= 1 << (11 - i);
		}
	}

	std::cout << "The power consumption of the submarine is " << gamma * epsilon
			<< '.' << std::endl;

	uint16_t oxygen = find_number(lines, false);
	uint16_t co2 = find_number(lines, true);

	std::cout << "The submarine life support rating is " << oxygen * co2 << '.' << std::endl;
}

int16_t count_bits(const std::vector<uint16_t> numbers, const uint8_t bit) {
	int16_t diff = 0;
	for (uint16_t number : numbers) {
		if (((number >> (11 - bit)) & 1) == 1) {
			diff++;
		} else {
			diff--;
		}
	}
	return diff;
}

uint16_t find_number(const std::vector<uint16_t> numbers, const bool lcb) {
	std::vector<uint16_t> current_numbers = numbers;
	std::vector<uint16_t> valid_numbers;
	uint8_t i = 0;
	while (current_numbers.size() > 1 && i < 12) {
		for (uint16_t number : current_numbers) {
			bool one = ((number >> (11 - i)) & 1) == 1;
			int16_t count = count_bits(current_numbers, i);
			if (count == 0) {
				if (one != lcb) {
					valid_numbers.push_back(number);
				}
			} else if (one == (lcb ? count < 0 : count > 0)) {
				valid_numbers.push_back(number);
			}
		}

		i++;
		current_numbers = valid_numbers;
		valid_numbers = std::vector<uint16_t>();
	}

	return current_numbers[0];
}
