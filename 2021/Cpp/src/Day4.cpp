/*
 * Day4.cpp
 *
 *  Created on: 04.12.2021
 *      Author: ToMe25
 */

#include "Day4.h"
#include <algorithm>
#include <iostream>
#include <sstream>

template <>
void DayRunner<4>::solve(std::ifstream input) {
	std::string token;
	input >> token;
	std::replace(token.begin(), token.end(), ',', ' ');
	std::istringstream numbers(token);

	uint8_t boards[100][5][5];
	for (uint8_t board = 0; board < 100; board++) {
		for (uint8_t x = 0; x < 5; x++) {
			for (uint8_t y = 0; y < 5; y++) {
				input >> token;
				boards[board][x][y] = std::stoi(token);
			}
		}
	}

	bool values[100][5][5] { false };
	uint16_t number;
	int8_t first_win_number = -1;
	std::vector<uint16_t> winners;
	while (numbers >> number) {
		set_number(boards, values, number, winners);
		if (winners.size() == 1 && first_win_number == -1) {
			first_win_number = number;
		} else if (winners.size() == 100) {
			break;
		}
	}

	uint32_t unmarked = 0;
	for (uint8_t x = 0; x < 5; x++) {
		for (uint8_t y = 0; y < 5; y++) {
			if (!values[winners[0]][x][y]) {
				unmarked += boards[winners[0]][x][y];
			}
		}
	}

	std::cout << "The first board winning score is " << unmarked * first_win_number << '.' << std::endl;

	unmarked = 0;
	for (uint8_t x = 0; x < 5; x++) {
		for (uint8_t y = 0; y < 5; y++) {
			if (!values[winners[99]][x][y]) {
				unmarked += boards[winners[99]][x][y];
			}
		}
	}

	std::cout << "The last board winning score is " << unmarked * number << '.' << std::endl;
}

bool check_win(const bool board[5][5]) {
	for (uint8_t x = 0; x < 5; x++) {
		bool full = true;
		for (uint8_t y = 0; y < 5; y++) {
			full = full && board[x][y];
		}

		if (full) {
			return true;
		}
	}

	for (uint8_t y = 0; y < 5; y++) {
		bool full = true;
		for (uint8_t x = 0; x < 5; x++) {
			full = full && board[x][y];
		}

		if (full) {
			return true;
		}
	}

	return false;
}

std::vector<uint16_t> set_number(const uint8_t boards[100][5][5],
		bool values[100][5][5], const uint8_t number,
		std::vector<uint16_t> &previous_winners) {
	for (uint8_t board = 0; board < 100; board++) {
		if (std::find(previous_winners.begin(), previous_winners.end(), board)
				== previous_winners.end()) {
			for (uint8_t x = 0; x < 5; x++) {
				for (uint8_t y = 0; y < 5; y++) {
					if (boards[board][x][y] == number) {
						values[board][x][y] = true;
					}
				}
			}

			if (check_win(values[board])) {
				previous_winners.push_back(board);
			}
		}
	}

	return previous_winners;
}
