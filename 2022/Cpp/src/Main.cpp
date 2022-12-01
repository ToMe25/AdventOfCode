/*
 * Main.cpp
 *
 *  Created on: 30.11.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <chrono>
#include <filesystem>
#include <regex>
#include <string>
#include <map>

// At this scale arrays would totally work, and also be more performant.
// But this is easier :P
std::map<uint8_t, aoc::part_func> *part1_funcs;
std::map<uint8_t, aoc::part_func> *part2_funcs;
std::map<uint8_t, aoc::combined_func> *combined_funcs;

int main(int argc, char *argv[]) {
	if (argc == 0) {
		std::cerr
				<< "This program expects the first parameter to be its execution path,"
				<< std::endl;
		std::cerr << "however it wasn't given a parameter." << std::endl;
		return 1;
	}

	bool time = false;
	for (int i = 0; i < argc; i++) {
		if (std::regex_match(argv[i], std::regex("-{0,2}h(elp)?"))) {
			aoc::printUsage(std::cout, argv[0]);
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
			int day = 1;
			if (std::regex_match(argv[i],
					std::regex("-{0,2}d(ay)?\\s+\\d{1,2}"))) {
				std::string dayStr = std::string(argv[i]);
				day = std::stoi(dayStr.substr(dayStr.find_last_of(' ')));
			} else if (argc > i + 1
					&& std::regex_match(argv[++i], std::regex("\\d{1,2}"))) {
				day = std::stoi(argv[i]);
			} else {
				aoc::printUsage(std::cout, argv[0]);
				return 0;
			}

			if (day < 1 || day > 25) {
				std::cerr << "Can't run day " << day
						<< " because it isn't a valid day." << std::endl;
				;
				std::exit(5);
			}

			bool p1 = part1_funcs && part1_funcs->count(day) > 0;
			bool p2 = part2_funcs && part2_funcs->count(day) > 0;
			bool c = combined_funcs && combined_funcs->count(day) > 0;
			std::chrono::steady_clock::time_point start1;
			std::chrono::steady_clock::time_point end1;
			std::chrono::steady_clock::time_point start2;
			std::chrono::steady_clock::time_point end2;

			if (p1 || p2 || c) {
				std::cout << "Running day " << (uint16_t) day << '.'
						<< std::endl;
				dayRun = true;
			} else {
				std::cerr << "Can't run day " << day
						<< " because no functions were registered for it."
						<< std::endl;
				std::exit(5);
			}

			std::string p1res;
			std::string p2res;
			if (p1) {
				start1 = std::chrono::steady_clock::now();
				p1res = part1_funcs->at(day)(aoc::getInputFileStream(day));
				end1 = std::chrono::steady_clock::now();
				std::cout << "Day " << (uint16_t) day << " part 1 result: "
						<< p1res << std::endl;
			}

			if (p2) {
				start2 = std::chrono::steady_clock::now();
				p2res = part2_funcs->at(day)(aoc::getInputFileStream(day));
				end2 = std::chrono::steady_clock::now();
				if (p1) {
					// Print day 1 result again, in case part 2 did a bunch of logging.
					std::cout << "Day " << (uint16_t) day << " part 1 result: "
							<< p1res << std::endl;
				}
				std::cout << "Day " << (uint16_t) day << " part 2 result: "
						<< p2res << std::endl;
			}

			if (c) {
				start1 = std::chrono::steady_clock::now();
				std::pair<std::string, std::string> res = combined_funcs->at(
						day)(aoc::getInputFileStream(day));
				end2 = std::chrono::steady_clock::now();
				p1res = res.first;
				p2res = res.second;
				std::cout << "Day " << (uint16_t) day << " part 1 result: "
						<< p1res << std::endl;
				std::cout << "Day " << (uint16_t) day << " part 2 result: "
						<< p2res << std::endl;
			}

			if (time) {
				if (p1 || p2 || c) {
					std::cout << "Running day " << (uint16_t) day;
				}

				if (p1) {
					std::cout << " part 1 took ";
					uint64_t time_us = std::chrono::duration_cast<
							std::chrono::microseconds>(end1 - start1).count();
					aoc::formatTime(std::cout, time_us);
					if (p2) {
						std::cout << ", and ";
					}
				}

				if (p2) {
					std::cout << " part 2 took ";
					uint64_t time_us = std::chrono::duration_cast<
							std::chrono::microseconds>(end2 - start2).count();
					aoc::formatTime(std::cout, time_us);
				}

				if (c) {
					std::cout << " took ";
					uint64_t time_us = std::chrono::duration_cast<
							std::chrono::microseconds>(end2 - start1).count();
					aoc::formatTime(std::cout, time_us);
				}

				if (p1 || p2 || c) {
					std::cout << '.' << std::endl;
				}
			}
		}
	}

	if (!dayRun) {
		aoc::printUsage(std::cout, argv[0]);
		return 1;
	}
}

std::ifstream aoc::getInputFileStream(const uint8_t day) {
	namespace fs = std::filesystem;

	fs::path input("..");
	input += fs::path::preferred_separator;
	input += "input";
	if (!fs::exists(input)) {
		std::cerr << "Directory " << input.c_str() << " doesn't exist."
				<< std::endl;
		exit(2);
	}
	input = fs::canonical(input);

	if (!fs::is_directory(input)) {
		std::cerr << "File " << input.c_str() << " is not a directory."
				<< std::endl;
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

std::vector<std::string> aoc::readLines(std::ifstream &input) {
	std::vector<std::string> lines;

	std::string line;
	while (std::getline(input, line)) {
		if (line.length() > 0) {
			lines.push_back(line);
		}
	}

	return lines;
}

void aoc::printUsage(std::ostream &out, const char *file) {
	out << "Usage: " << file << " [<OPTIONS>] --day <DAY>" << std::endl;
	out << "At least one -day argument has to be specified." << std::endl;
	out
			<< " -d --day <DAY>	Specifies a day to be run. Can be supplied more then once."
			<< std::endl;
	out << " -h --help		Prints this help text and terminates." << std::endl;
	out << " -t --time		Measures the execution time of each day." << std::endl;
}

void aoc::formatTime(std::ostream &out, const uint64_t time_us) {
	if (time_us / 3600000000 > 0) {
		out << time_us / 60000000 << "h ";
	}

	if (time_us % 3600000000 / 60000000 > 0) {
		out << time_us / 60000000 << "m ";
	}

	if (time_us % 60000000 / 1000000 > 0) {
		out << time_us % 60000000 / 1000000 << "s ";
	}

	if (time_us % 1000000 / 1000 > 0) {
		out << time_us % 1000000 / 1000 << "ms ";
	}

	out << time_us % 1000 << "µs";
}

bool aoc::registerPart1(const uint8_t day, const part_func &func) {
	if (part1_funcs && part1_funcs->count(day) > 0) {
		std::cerr << "A function for day " << (uint16_t) day
				<< " part 1 was already registered." << std::endl;
		exit(4);
	} else if (combined_funcs && combined_funcs->count(day) > 0) {
		std::cerr << "A combined function for day " << (uint16_t) day
				<< " was already registered." << std::endl;
		exit(4);
	}

	if (day == 0 || day > 25) {
		std::cerr << "Day " << (uint16_t) day
				<< " is not a valid day. Must be 1-25." << std::endl;
		exit(4);
	}

	if (!part1_funcs) {
		part1_funcs = new std::map<uint8_t, part_func>();
	}

	part1_funcs->insert( { day, func });

	return true;
}

bool aoc::registerPart2(const uint8_t day, const part_func &func) {
	if (part2_funcs && part2_funcs->count(day) > 0) {
		std::cerr << "A function for day " << (uint16_t) day
				<< " part 2 was already registered." << std::endl;
		exit(4);
	} else if (combined_funcs && combined_funcs->count(day) > 0) {
		std::cerr << "A combined function for day " << (uint16_t) day
				<< " was already registered." << std::endl;
		exit(4);
	}

	if (day == 0 || day > 25) {
		std::cerr << "Day " << (uint16_t) day
				<< " is not a valid day. Must be 1-25." << std::endl;
		exit(4);
	}

	if (!part2_funcs) {
		part2_funcs = new std::map<uint8_t, part_func>();
	}

	part2_funcs->insert( { day, func });

	return true;
}

bool aoc::registerCombined(const uint8_t day, const combined_func &func) {
	if (part1_funcs && part1_funcs->count(day) > 0) {
		std::cerr << "A function for day " << (uint16_t) day
				<< " part 1 was already registered." << std::endl;
		exit(4);
	} else if (part2_funcs && part2_funcs->count(day) > 0) {
		std::cerr << "A function for day " << (uint16_t) day
				<< " part 2 was already registered." << std::endl;
		exit(4);
	}

	if (day == 0 || day > 25) {
		std::cerr << "Day " << (uint16_t) day
				<< " is not a valid day. Must be 1-25." << std::endl;
		exit(4);
	}

	if (!combined_funcs) {
		combined_funcs = new std::map<uint8_t, combined_func>();
	}

	combined_funcs->insert( { day, func });

	return true;
}
