/*
 * Day1.cpp
 *
 *  Created on: 01.12.2021
 *      Author: ToMe25
 */

#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

namespace fs = std::filesystem;

int main() {
	fs::path input = "..";
	input += fs::path::preferred_separator;
	input += "input";
	input = fs::canonical(input);
	if (!fs::exists(input)) {
		std::cerr << "Directory " << input.c_str() << " doesn't exist." << std::endl;
		return 1;
	}

	if (!fs::is_directory(input)) {
		std::cerr << "File " << input.c_str() << " is not a directory." << std::endl;
		return 1;
	}

	input += fs::path::preferred_separator;
	input += "Day1.txt";
	if (!fs::exists(input) || !fs::is_regular_file(input)) {
		std::cerr << "File " << input.c_str() << " doesn't exist or isn't a file." << std::endl;
		return 1;
	}

	std::ifstream in_stream(input);

	uint increases = 0, sum_increases = 0;
	uint depth = 0;
	uint previous_depths[2] { 0 };
	uint sum = 0, previous_sum = 0;
	while (in_stream >> depth) {
		if (previous_depths[0] != 0) {
			if (depth > previous_depths[0]) {
				increases++;
			}

			if (previous_depths[1] != 0) {
				sum = depth + previous_depths[0] + previous_depths[1];
				if (previous_sum != 0) {
					if (previous_sum < sum) {
						sum_increases ++;
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
