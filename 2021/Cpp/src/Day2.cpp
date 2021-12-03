/*
 * Day2.cpp
 *
 *  Created on: 02.12.2021
 *      Author: ToMe25
 */

#include "Day2.h"
#include <iostream>

template <>
void DayRunner<2>::solve(std::ifstream input) {
	uint depth = 0;
	uint horizontal = 0;
	uint aim = 0;
	std::string command;
	uint value;
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

	std::cout << "The horizontal multiplied by the aim(part 1 depth) is " << horizontal * aim << '.' << std::endl;
	std::cout << "The horizontal multiplied by the depth is " << horizontal * depth << '.' << std::endl;
}
