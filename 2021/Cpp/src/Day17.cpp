/*
 * Day17.cpp
 *
 *  Created on: 17.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <cmath>
#include <iostream>

template<>
void DayRunner<17>::solve(std::ifstream input) {
	std::string line;
	std::pair<int16_t, int16_t> target_x;
	std::pair<int16_t, int16_t> target_y;
	size_t dot_index;
	int16_t start, end;
	while (input >> line) {
		dot_index = line.find('.');
		if (dot_index != std::string::npos) {
			start = std::stoi(line.substr(2, dot_index - 2));
			dot_index += 2;
			end = line.length() - dot_index;
			if (line.find(',') != std::string::npos) {
				end--;
			}
			end = std::stoi(line.substr(dot_index, end));
		}

		if (line[0] == 'x') {
			target_x.first = start;
			target_x.second = end;
		} else if (line[0] == 'y') {
			target_y.first = start;
			target_y.second = end;
		}
	}

	int16_t vel_x, vel_y;
	int16_t x, y;
	int16_t max_y = 0;
	int16_t best_max_y = 0;

	uint16_t velocities = 0;
	for (int16_t start_velocity_y = target_y.first;
			start_velocity_y <= -target_y.first * 2; start_velocity_y++) {
		for (int16_t start_velocity_x = 0; start_velocity_x <= target_x.second;
				start_velocity_x++) {
			x = y = 0;
			max_y = 0;
			vel_x = start_velocity_x;
			vel_y = start_velocity_y;
			while (vel_y >= 0 || y > target_y.first) {
				x += vel_x;
				y += vel_y;

				if (vel_x > 0) {
					vel_x--;
				} else if (vel_x < 0) {
					vel_x++;
				}
				vel_y--;

				if (y > max_y) {
					max_y = y;
				}

				if (x >= target_x.first && x <= target_x.second
						&& y >= target_y.first && y <= target_y.second) {
					if (max_y > best_max_y) {
						best_max_y = max_y;
					}
					velocities++;
					break;
				}
			}
		}
	}

	std::cout << "The highest possible y position is " << best_max_y << '.' << std::endl;

	std::cout << "There are a total of " << velocities
			<< " possible velocities that hit the target." << std::endl;
}
