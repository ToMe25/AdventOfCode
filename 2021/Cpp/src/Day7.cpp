/*
 * Day7.cpp
 *
 *  Created on: 07.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <algorithm>
#include <cmath>
#include <iostream>
#include <sstream>
#include <vector>

template<>
void DayRunner<7>::solve(std::ifstream input) {
	std::string input_str;
	input >> input_str;
	std::replace(input_str.begin(), input_str.end(), ',', ' ');
	std::istringstream numbers_stream(input_str);

	uint16_t number;
	std::vector<uint16_t> numbers;
	uint32_t average = 0;
	while (numbers_stream >> number) {
		numbers.push_back(number);
		average += number;
	}

	std::sort(numbers.begin(), numbers.end());

	uint16_t median = numbers[numbers.size() / 2];
	uint32_t distance = 0;
	for (int16_t nr : numbers) {
		distance += abs(nr - median);
	}

	std::cout << "The total fuel used for part 1 is " << distance << " units." << std::endl;

	average /= numbers.size();

	distance = 0;
	uint16_t dist;
	for (int16_t nr : numbers) {
		dist = abs(nr - average);
		distance += (dist + 1) * dist / 2.0;
	}

	std::cout << "The total fuel used for part 2 is " << distance << " units." << std::endl;
}
