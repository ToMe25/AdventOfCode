/*
 * Day5.cpp
 *
 *  Created on: Dec 5, 2022
 *      Author: ToMe25
 */

#include "Main.h"
#include <sstream>

std::string day5part1(std::ifstream input) {
	std::vector<std::vector<uint8_t>> crates;
	std::string line;
	bool reversed = false;
	while (std::getline(input, line)) {
		if (line.length() < 3) {
			continue;
		} else if (line.find('[') != std::string::npos) {
			for (size_t i = 1; i < line.length() + 4; i += 4) {
				if (!std::isalnum(line[i])) {
					continue;
				}

				for (size_t j = crates.size(); j <= (i - 1) / 4; j++) {
					crates.push_back(std::vector<uint8_t>());
				}
				crates[(i - 1) / 4].push_back(line[i]);
			}
		} else if (line.rfind("move", 0) == 0) {
			if (!reversed) {
				for (std::vector<uint8_t> &stack : crates) {
					std::reverse(stack.begin(), stack.end());
				}
				reversed = true;
			}

			std::istringstream in(line);
			std::vector<std::string> tokens;
			std::string tok;
			while (in >> tok) {
				tokens.push_back(tok);
			}

			if (tokens.size() < 6) {
				std::cerr << "Received invalid move line \"" << line << "\"."
						<< std::endl;
				continue;
			}

			const uint32_t count = std::stoi(tokens[1]);
			const uint32_t start = std::stoi(tokens[3]) - 1;
			const uint32_t target = std::stoi(tokens[5]) - 1;
			for (size_t i = 0; i < count; i++) {
				crates[target].push_back(*crates[start].rbegin());
				crates[start].pop_back();
			}
		}
	}

	std::ostringstream result;
	for (size_t i = 0; i < crates.size(); i++) {
		if (crates[i].size() > 0) {
			result << *crates[i].rbegin();
		}
	}

	return result.str();
}

std::string day5part2(std::ifstream input) {
	std::vector<std::string> crates;
	std::string line;
	bool reversed = false;
	while (std::getline(input, line)) {
		if (line.length() < 3) {
			continue;
		} else if (line.find('[') != std::string::npos) {
			for (size_t i = 1; i < line.length() + 4; i += 4) {
				if (!std::isalnum(line[i])) {
					continue;
				}

				for (size_t j = crates.size(); j <= (i - 1) / 4; j++) {
					crates.push_back(std::string());
				}
				crates[(i - 1) / 4] += line[i];
			}
		} else if (line.rfind("move", 0) == 0) {
			if (!reversed) {
				for (std::string &stack : crates) {
					std::reverse(stack.begin(), stack.end());
				}
				reversed = true;
			}

			std::istringstream in(line);
			std::vector<std::string> tokens;
			std::string tok;
			while (in >> tok) {
				tokens.push_back(tok);
			}

			if (tokens.size() < 6) {
				std::cerr << "Received invalid move line \"" << line << "\"."
						<< std::endl;
				continue;
			}

			const uint32_t count = std::stoi(tokens[1]);
			const uint32_t start = std::stoi(tokens[3]) - 1;
			const uint32_t target = std::stoi(tokens[5]) - 1;
			crates[target] += crates[start].substr(crates[start].length() - count);
			crates[start] = crates[start].substr(0, crates[start].length() - count);
		}
	}

	std::ostringstream result;
	for (size_t i = 0; i < crates.size(); i++) {
		if (crates[i].size() > 0) {
			result << *crates[i].rbegin();
		}
	}

	return result.str();
}

bool d5p1 = aoc::registerPart1(5, &day5part1);
bool d5p2 = aoc::registerPart2(5, &day5part2);
