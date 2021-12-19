/*
 * Day18.cpp
 *
 *  Created on: 18.12.2021
 *      Author: ToMe25
 */

#include "Day18.h"
#include <cmath>
#include <iostream>
#include <sstream>
#include <stack>
#include <vector>

template<>
void DayRunner<18>::solve(std::ifstream input) {
	std::string line;
	std::string current;
	std::vector<std::string> lines;
	while (input >> line) {
		if (current.empty()) {
			current = line;
		} else {
			current = add_snailfish(current, line);
		}
		lines.push_back(line);
	}

	uint16_t magnitude = get_magnitude(current);

	std::cout << "The resulting magnitute is " << magnitude << '.' << std::endl;

	uint16_t max_magnitude = 0;
	for (std::string line : lines) {
		for (std::string line2 : lines) {
			if (line != line2) {
				uint16_t magnitude = get_magnitude(add_snailfish(line, line2));
				if (magnitude > max_magnitude) {
					max_magnitude = magnitude;
				}
			}
		}
	}

	std::cout << "The max possible magnitude from adding two numbers is "
			<< max_magnitude << '.' << std::endl;
}

std::string add_snailfish(const std::string first, const std::string second) {
	std::string current = '[' + first + ',' + second + ']';
	std::stack<uint16_t> pairs;
	uint8_t last_nr;
	uint16_t last_nr_i;
	bool modified = true;

	while (modified) {
		modified = false;
		last_nr = 0;
		last_nr_i = 0;
		pairs = std::stack<uint16_t>();

		for (uint16_t i = 0; i < current.length(); i++) {
			char c = current[i];
			if (c == '[') {
				pairs.push(i);
			} else if (c == ']') {
				pairs.pop();
			} else if (isdigit(c)) {
				if (pairs.size() > 4) {
					if (current[i + 1] == ',' && isdigit(current[i + 2])) {
						if (last_nr_i > 0) {
							uint8_t number;
							if (isdigit(current[i - 1])) {
								number = std::stoi(current.substr(i - 1, 2));
							} else {
								number = c - '0';
							}

							if (last_nr > 9) {
								current = current.replace(last_nr_i, 2,
										std::to_string(last_nr + number));
							} else {
								current = current.replace(last_nr_i, 1,
										std::to_string(last_nr + number));
								if (last_nr + number > 9) {
									i++;
								}
							}
						}

						i += 2;

						for (uint16_t j = i + 1; j < current.length(); j++) {
							if (isdigit(current[j])) {
								uint16_t number;
								if (isdigit(current[i + 1])) {
									if (i + 1 == j) {
										continue;
									}
									number = std::stoi(current.substr(i, 2));
								} else {
									number = current[i] - '0';
								}

								if (isdigit(current[j + 1])) {
									number += std::stoi(current.substr(j, 2));
									current = current.replace(j, 2,
											std::to_string(number));
									break;
								} else {
									number += current[j] - '0';
									current = current.replace(j, 1,
											std::to_string(number));
									break;
								}
							}
						}

						if (isdigit(current[i - 3])
								&& isdigit(current[i + 1])) {
							current = current.replace(i - 4, 7, "0");
							i -= 4;
						} else if (isdigit(current[i - 3])) {
							current = current.replace(i - 4, 6, "0");
							i -= 4;
						} else if (isdigit(current[i + 1])) {
							current = current.replace(i - 3, 6, "0");
							i -= 3;
						} else {
							current = current.replace(i - 3, 5, "0");
							i -= 3;
						}

						pairs.pop();
						last_nr_i = i;
						last_nr = 0;
						modified = true;
					}
				} else {
					if (last_nr_i + 1 == i) {
						last_nr = std::stoi(current.substr(i - 1, 2));
					} else {
						last_nr = c - '0';
						last_nr_i = i;
					}
				}
			}
		}

		if (!modified) {
			for (uint16_t i = 0; i < current.length(); i++) {
				char c = current[i];
				if (c == '[') {
					pairs.push(i);
				} else if (c == ']') {
					pairs.pop();
				} else if (isdigit(c) && isdigit(current[i + 1])) {
					uint8_t number = std::stoi(current.substr(i, 2));
					std::ostringstream format;
					format << '[' << (uint16_t) number / 2;
					format << ',' << (uint16_t) ceil(number / 2.0) << ']';
					current = current.replace(i, 2, format.str());
					modified = true;
					break;
				}
			}
		}
	}

	return current;
}

uint16_t get_magnitude(const std::string number) {
	std::stack<uint16_t> numbers;
	for (uint16_t i = 0; i < number.length(); i++) {
		char c = number[i];
		if (isdigit(c)) {
			numbers.push(c - '0');
		} else if (c == ']') {
			uint32_t nr = numbers.top() * 2;
			numbers.pop();
			nr += numbers.top() * 3;
			numbers.pop();
			numbers.push(nr);
		}
	}

	return numbers.top();
}
