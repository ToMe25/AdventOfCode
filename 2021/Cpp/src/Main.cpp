/*
 * Main.cpp
 *
 *  Created on: 02.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include "Day1.h"
#include "Day2.h"
#include <filesystem>
#include <iostream>
#include <regex>
#include <string>

AoCRunner* getRunner(uint8_t day) {
	switch (day) {
	case 1:
		return new DayRunner<1>();
	case 2:
		return new DayRunner<2>();
	default:
		std::cerr << "Received not yet implemented day " << day << "to run.";
		exit(2);
	}
}

int main(int argc, char* argv[]) {
	for (int i = 0; i < argc; i++) {
		if (std::regex_match(argv[i], std::regex("-{0,2}day\\s*\\d{0,2}"))) {
			uint8_t day = 1;
			if (std::regex_match(argv[i], std::regex("-{0,2}day\\s+\\d{1,2}"))) {
				std::string dayStr = std::string(argv[i]);
				day = std::stoi(dayStr.substr(dayStr.find_last_of(' ')));
			} else if (argc > i
					&& std::regex_match(argv[i + 1], std::regex("\\d{1,2}"))) {
				day = std::stoi(argv[i + 1]);
			}
			AoCRunner *runner = getRunner(day);
			runner->solve();
			return 0;
		}
	}

	std::cout << "Please specify the day to run with -day DAY." << std::endl;
	return 1;
}

std::ifstream getInputFileStream(uint8_t day) {
	namespace fs = std::filesystem;

	fs::path input = "..";
	input += fs::path::preferred_separator;
	input += "input";
	input = fs::canonical(input);
	if (!fs::exists(input)) {
		std::cerr << "Directory " << input.c_str() << " doesn't exist." << std::endl;
		exit(1);
	}

	if (!fs::is_directory(input)) {
		std::cerr << "File " << input.c_str() << " is not a directory." << std::endl;
		exit(1);
	}

	input += fs::path::preferred_separator;
	input += "Day";
	input += std::to_string(day);
	input += ".txt";
	if (!fs::exists(input) || !fs::is_regular_file(input)) {
		std::cerr << "File " << input.c_str() << " doesn't exist or isn't a file." << std::endl;
		exit(1);
	}

	return std::ifstream(input);
}
