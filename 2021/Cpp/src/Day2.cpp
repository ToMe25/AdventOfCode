/*
 * Day2.cpp
 *
 *  Created on: 02.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <iostream>

template<>
void DayRunner<2>::solve(std::ifstream input) {
	uint32_t depth = 0;
	uint32_t horizontal = 0;
	uint32_t aim = 0;
	std::string command;
	uint32_t value;
	while (input >> command >> value) {
		if (command == "forward") {
			horizontal += value;
			depth += value * aim;
		} else if (command == "down") {
			aim += value;
		} else if (command == "up")
			aim -= value;
		else {
			std::cerr << "Received invalid command \"" << command << "\"." << std::endl;
			exit(1);
		}
	}

	std::cout << "The horizontal multiplied by the aim(part 1 depth) is "
			<< horizontal * aim << '.' << std::endl;
	std::cout << "The horizontal multiplied by the depth is "
			<< horizontal * depth << '.' << std::endl;
}
