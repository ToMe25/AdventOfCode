/*
 * Day10.cpp
 *
 *  Created on: 10.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <algorithm>
#include <iostream>
#include <stack>
#include <vector>

template <>
void DayRunner<10>::solve(std::ifstream input) {
	std::string line;
	std::stack<char> opened;
	uint32_t error_score = 0;
	std::vector<uint64_t> closing_strings;
	uint64_t closing;
	while (input >> line) {
		for (char c : line) {
			if (c == '(' || c == '[' || c == '{' || c == '<') {
				opened.push(c);
			} else if (c == ')' && opened.top() != '(') {
				error_score += 3;
				opened = std::stack<char>();
				break;
			} else if (c != ')' && c != opened.top() + 2) {
				if (c == ']') {
					error_score += 57;
				} else if (c == '}') {
					error_score += 1197;
				} else if (c == '>') {
					error_score += 25137;
				}
				opened = std::stack<char>();
				break;
			} else {
				opened.pop();
			}
		}

		closing = 0;
		while (opened.size() > 0) {
			closing *= 5;
			switch(opened.top()) {
			case '(':
				closing += 1;
				break;
			case '[':
				closing += 2;
				break;
			case '{':
				closing += 3;
				break;
			case '<':
				closing += 4;
				break;
			default:
				std::cerr << "Found invalding opening char " << opened.top() << '.' << std::endl;
				break;
			}
			opened.pop();
		}

		if (closing > 0) {
			closing_strings.push_back(closing);
		}
	}

	std::cout << "The error score is " << error_score << '.' << std::endl;

	std::sort(closing_strings.begin(), closing_strings.end());

	std::cout << "The middle closing string is " << closing_strings[closing_strings.size() / 2] << '.' << std::endl;
}
