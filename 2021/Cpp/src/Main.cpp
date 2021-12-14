/*
 * Main.cpp
 *
 *  Created on: 02.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <filesystem>
#include <iostream>
#include <regex>
#include <string>

template <uint8_t... Days>
AoCRunner* getRunner(const uint8_t day, const std::integer_sequence<uint8_t, Days...>) {
	AoCRunner *runners[] = { new DayRunner<Days + 1>()... };
	if (day > 0 && day <= sizeof(runners)/sizeof(runners[0])) {
		return runners[day - 1];
	} else {
		std::cerr << "Trying to run not yet implemented day " << (int) day << '.' << std::endl;
		exit(4);
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
			AoCRunner *runner = getRunner(day, std::make_integer_sequence<uint8_t, 14>());
			runner->solve();
			return 0;
		}
	}

	std::cout << "Please specify the day to run with -day DAY." << std::endl;
	return 1;
}

std::ifstream getInputFileStream(const uint8_t day) {
	namespace fs = std::filesystem;

	fs::path input = "..";
	input += fs::path::preferred_separator;
	input += "input";
	input = fs::canonical(input);
	if (!fs::exists(input)) {
		std::cerr << "Directory " << input.c_str() << " doesn't exist." << std::endl;
		exit(2);
	}

	if (!fs::is_directory(input)) {
		std::cerr << "File " << input.c_str() << " is not a directory." << std::endl;
		exit(2);
	}

	input += fs::path::preferred_separator;
	input += "Day";
	input += std::to_string(day);
	input += ".txt";
	if (!fs::exists(input) || !fs::is_regular_file(input)) {
		std::cerr << "File " << input.c_str() << " doesn't exist or isn't a file." << std::endl;
		exit(3);
	}

	return std::ifstream(input);
}
