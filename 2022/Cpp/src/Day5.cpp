/*
 * Day5.cpp
 *
 *  Created on: Dec 5, 2022
 *      Author: ToMe25
 */

#include "Main.h"
#include <sstream>

std::pair<std::string, std::string> day5Combined(std::ifstream input) {
	std::vector<std::vector<uint8_t>> crates;
	std::vector<std::vector<uint8_t>> crates2;
	std::string line;
	bool reversed = false;
	while (std::getline(input, line)) {
		if (line.length() < 3) {
			continue;
		} else if (line.find('[') != std::string::npos) {
			for (size_t i = 1; i < line.length(); i += 4) {
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
				crates2 = crates;
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

			crates2[target].insert(crates2[target].end(),
					crates2[start].begin() + (crates2[start].size() - count),
					crates2[start].end());
			crates2[start].resize(crates2[start].size() - count);
		}
	}

	std::ostringstream result1;
	for (size_t i = 0; i < crates.size(); i++) {
		if (crates[i].size() > 0) {
			result1 << *crates[i].rbegin();
		}
	}

	std::ostringstream result2;
	for (size_t i = 0; i < crates2.size(); i++) {
		if (crates2[i].size() > 0) {
			result2 << *crates2[i].rbegin();
		}
	}

	return {result1.str(), result2.str()};
}

bool d5c = aoc::registerCombined(5, &day5Combined);
