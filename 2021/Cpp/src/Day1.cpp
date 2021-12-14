/*
 * Day1.cpp
 *
 *  Created on: 01.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <iostream>

template<>
void DayRunner<1>::solve(std::ifstream input) {
	uint increases = 0, sum_increases = 0;
	uint depth = 0;
	uint previous_depths[2] { 0 };
	uint sum = 0, previous_sum = 0;
	while (input >> depth) {
		if (previous_depths[0] != 0) {
			if (depth > previous_depths[0]) {
				increases++;
			}

			if (previous_depths[1] != 0) {
				sum = depth + previous_depths[0] + previous_depths[1];
				if (previous_sum != 0) {
					if (previous_sum < sum) {
						sum_increases++;
					}
				}
				previous_sum = sum;
			}
		}

		previous_depths[1] = previous_depths[0];
		previous_depths[0] = depth;
	}

	std::cout << "The depth increased " << increases << " times." << std::endl;
	std::cout << "The depth sum increased " << sum_increases << " times." << std::endl;
}
