/*
 * Day25.cpp
 *
 *  Created on: Nov 29, 2022
 *      Author: ToMe25
 */

#include "Main.h"
#include <iostream>
#include <vector>

template<>
void DayRunner<25>::solve(std::ifstream input) {
	std::vector<std::string> lines;

	std::string line;
	while (std::getline(input, line)) {
		if (line.length() > 0) {
			lines.push_back(line);
		}
	}

	int8_t cucumbers[lines.size()][lines[0].length()];
	for (size_t i = 0; i < lines.size(); i++) {
		for (size_t j = 0; j < lines[0].length(); j++) {
			char c = lines[i][j];
			if (c == '>') {
				cucumbers[i][j] = 1;
			} else if (c == 'v') {
				cucumbers[i][j] = 3;
			} else {
				cucumbers[i][j] = 0;
			}
		}
	}

	bool moved = true;
	size_t steps = 0;
	while (moved) {
		moved = false;
		for (size_t i = 0; i < lines.size(); i++) {
			for (size_t j = 0; j < lines[0].length(); j++) {
				if (cucumbers[i][j] == 1) {
					if (j < lines[0].length() - 1) {
						if (cucumbers[i][j + 1] < 1) {
							cucumbers[i][j + 1] = 2;
							cucumbers[i][j] = -1;
							moved = true;
						}
					} else {
						if (cucumbers[i][0] == 0) {
							cucumbers[i][0] = 2;
							cucumbers[i][j] = -1;
							moved = true;
						}
					}
				}
			}
		}

		for (size_t i = 0; i < lines.size(); i++) {
			for (size_t j = 0; j < lines[0].length(); j++) {
				if (cucumbers[i][j] == -1) {
					cucumbers[i][j] = 0;
				} else if (cucumbers[i][j] == 2) {
					cucumbers[i][j] = 1;
				}
			}
		}

		for (size_t i = 0; i < lines.size(); i++) {
			for (size_t j = 0; j < lines[0].length(); j++) {
				if (cucumbers[i][j] == 3) {
					if (i < lines.size() - 1) {
						if (cucumbers[i + 1][j] < 1) {
							cucumbers[i + 1][j] = 4;
							cucumbers[i][j] = -1;
							moved = true;
						}
					} else {
						if (cucumbers[0][j] == 0) {
							cucumbers[0][j] = 4;
							cucumbers[i][j] = -1;
							moved = true;
						}
					}
				}
			}
		}

		for (size_t i = 0; i < lines.size(); i++) {
			for (size_t j = 0; j < lines[0].length(); j++) {
				if (cucumbers[i][j] == -1) {
					cucumbers[i][j] = 0;
				} else if (cucumbers[i][j] == 4) {
					cucumbers[i][j] = 3;
				}
			}
		}

		steps++;
	}

	std::cout << "The sea cucumbers stopped moving after " << steps << " steps."
			<< std::endl;
}
