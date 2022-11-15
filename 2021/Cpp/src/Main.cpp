/*
 * Main.cpp
 *
 *  Created on: 02.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <chrono>
#include <filesystem>
#include <iostream>
#include <regex>
#include <string>

template<uint8_t ... Days>
AoCRunner* getRunner(const uint8_t day,
		const std::integer_sequence<uint8_t, Days...>) {
	AoCRunner *runners[] = { new DayRunner<Days + 1>()... };
	if (day > 0 && day <= sizeof(runners) / sizeof(runners[0])) {
		return runners[day - 1];
	} else {
		std::cerr << "Trying to run not yet implemented day " << (uint16_t) day
				<< '.' << std::endl;
		return NULL;
	}
}

int main(int argc, char *argv[]) {
	if (argc == 0) {
		std::cerr << "This program expects the first parameter to be its execution path," << std::endl;
		std::cerr << "however it wasn't given a parameter." << std::endl;
		return 1;
	}

	bool time = false;
	for (int i = 0; i < argc; i++) {
		if (std::regex_match(argv[i], std::regex("-{0,2}h(elp)?"))) {
			printUsage(argv[0]);
			return 0;
		} else if (std::regex_match(argv[i], std::regex("-{0,2}t(ime)?"))) {
			time = true;
		}
	}

	if (time) {
		std::cout << "Timing day executions." << std::endl;
	}

	bool dayRun = false;
	for (int i = 0; i < argc; i++) {
		if (std::regex_match(argv[i], std::regex("-{0,2}d(ay)?\\s*\\d{0,2}"))) {
			uint8_t day = 1;
			if (std::regex_match(argv[i],
					std::regex("-{0,2}d(ay)?\\s+\\d{1,2}"))) {
				std::string dayStr = std::string(argv[i]);
				day = std::stoi(dayStr.substr(dayStr.find_last_of(' ')));
			} else if (argc > i + 1
					&& std::regex_match(argv[++i], std::regex("\\d{1,2}"))) {
				day = std::stoi(argv[i]);
			} else {
				printUsage(argv[0]);
				return 0;
			}

			AoCRunner *runner = getRunner(day,
					std::make_integer_sequence<uint8_t, 23>());
			if (runner != NULL) {
				std::cout << "Running day " << (uint16_t) day << '.' << std::endl;
				std::chrono::steady_clock::time_point start = std::chrono::steady_clock::now();
				runner->solve();
				std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
				dayRun = true;

				if (time) {
					std::cout << "Running day " << (uint16_t) day << " took ";
					uint64_t time_us = std::chrono::duration_cast<
							std::chrono::microseconds>(end - start).count();
					if (time_us / 60000000 > 0) {
						std::cout << time_us / 60000000 << "m ";
					}

					if (time_us % 60000000 / 1000000 > 0) {
						std::cout << time_us % 60000000 / 1000000 << "s ";
					}

					if (time_us % 1000000 / 1000 > 0) {
						std::cout << time_us % 1000000 / 1000 << "ms ";
					}

					std::cout << time_us % 1000 << "µs." << std::endl;
				}
			}
		}
	}

	if (!dayRun) {
		printUsage(argv[0]);
		return 1;
	}
}

std::ifstream getInputFileStream(const uint8_t day) {
	namespace fs = std::filesystem;

	fs::path input = "..";
	input += fs::path::preferred_separator;
	input += "input";
	if (!fs::exists(input)) {
		std::cerr << "Directory " << input.c_str() << " doesn't exist." << std::endl;
		exit(2);
	}
	input = fs::canonical(input);

	if (!fs::is_directory(input)) {
		std::cerr << "File " << input.c_str() << " is not a directory." << std::endl;
		exit(2);
	}

	input += fs::path::preferred_separator;
	input += "Day";
	input += std::to_string(day);
	input += ".txt";
	if (!fs::exists(input) || !fs::is_regular_file(input)) {
		std::cerr << "File " << input.c_str()
				<< " doesn't exist or isn't a file." << std::endl;
		exit(3);
	}

	return std::ifstream(input);
}

void printUsage(const char *filename) {
	std::cout << "Usage: " << filename << " [<OPTIONS>] --day <DAY>" << std::endl;
	std::cout << "At least one -day argument has to be specified." << std::endl;
	std::cout << " -d --day <DAY>	Specifies a day to be run. Can be supplied more then once." << std::endl;
	std::cout << " -h --help		Prints this help text and terminates." << std::endl;
	std::cout << " -t --time		Measures the execution time of each day." << std::endl;
}
