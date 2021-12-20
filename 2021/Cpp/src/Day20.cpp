/*
 * Day20.cpp
 *
 *  Created on: 20.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <iostream>
#include <vector>

template<>
void DayRunner<20>::solve(std::ifstream input) {
	std::string line;
	input >> line;

	bool enhancement[line.length()];
	for (uint16_t i = 0; i < line.length(); i++) {
		if (line[i] == '#') {
			enhancement[i] = true;
		} else {
			enhancement[i] = false;
		}
	}

	std::vector<std::string> lines;
	while (input >> line) {
		lines.push_back(line);
	}

	uint8_t current_height = lines.size();
	uint8_t current_width = lines[0].length();
	bool *image = new bool[current_height * current_width];
	for (uint8_t y = 0; y < current_height; y++) {
		for (uint8_t x = 0; x < current_width; x++) {
			image[y * current_width + x] = lines[y][x] == '#';
		}
	}

	bool *new_image;
	bool empty_lit = false;
	uint16_t lit = 0;
	uint8_t new_height, new_width;
	for (uint8_t i = 0; i < 50; i++) {
		new_height = current_height + 2;
		new_width = current_width + 2;
		new_image = new bool[new_height * new_width];
		for (uint8_t y = 0; y < new_height; y++) {
			for (uint8_t x = 0; x < new_width; x++) {
				uint16_t value = 0;
				for (int8_t j = -2; j < 1; j++) {
					for (int8_t k = -2; k < 1; k++) {
						if (y + j >= 0 && y + j < current_height && x + k >= 0
								&& x + k < current_width) {
							value += image[(y + j) * current_width + (x + k)]
									<< (8 - (j + 2) * 3 - k - 2);
						} else {
							value += empty_lit << (8 - (j + 2) * 3 - k - 2);
						}
					}
				}
				new_image[y * new_width + x] = enhancement[value];
			}
		}

		delete[] image;
		current_height = new_height;
		current_width = new_width;
		image = new_image;
		empty_lit = empty_lit ? enhancement[511] : enhancement[0];

		if (i == 2) {
			for (uint16_t i = 0; i < current_height * current_width; i++) {
				if (image[i]) {
					lit++;
				}
			}

			std::cout << "The doubly enhanced image has " << lit
					<< " lit pixels." << std::endl;
		}
	}

	lit = 0;
	for (uint16_t i = 0; i < current_height * current_width; i++) {
		if (image[i]) {
			lit++;
		}
	}

	std::cout << "After enhancing the image 50 times " << lit
			<< " pixels are lit." << std::endl;
}
