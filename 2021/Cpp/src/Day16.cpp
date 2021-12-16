/*
 * Day16.cpp
 *
 *  Created on: 16.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <iostream>
#include <map>
#include <stack>
#include <vector>

template<>
void DayRunner<16>::solve(std::ifstream input) {
	std::string line;
	input >> line;
	std::vector<bool> data(line.length() * 4);

	uint8_t byte;
	for (uint16_t i = 0; i < line.length(); i += 2) {
		byte = std::stoi(line.substr(i, 2), 0, 16);
		for (uint8_t j = 0; j < 8; j++) {
			data[i * 4 + j] = byte >> (7 - j) & 1;
		}
	}

	uint32_t version_sum = 0;
	std::stack<uint16_t> packets;
	std::map<uint16_t, std::pair<bool, uint16_t>> packet_subpackets;
	std::map<uint16_t, uint8_t> packet_types;
	std::map<uint16_t, int64_t> packet_values;

	// packet fields
	uint8_t version;
	uint8_t type;
	bool subpackets_number = false;
	uint16_t subpackets = 0;
	uint16_t i = 0;

	int64_t result = 0;
	do {
		if (!packets.empty()) {
			uint16_t packet = packets.top();
			if ((packet_subpackets[packet].first
					&& packet_subpackets[packet].second == 0)
					|| (!packet_subpackets[packet].first
							&& packet + packet_subpackets[packet].second <= i)) {
				int64_t value = packet_values[packet];
				packets.pop();

				if (packets.empty()) {
					result = packet_values[packet];
				} else {
					packet = packets.top();

					switch (packet_types[packet]) {
					case 0:
						if (packet_values[packet] == -1) {
							packet_values[packet]++;
						}
						packet_values[packet] += value;
						break;
					case 1:
						if (packet_values[packet] == -1) {
							packet_values[packet] = value;
						} else {
							packet_values[packet] *= value;
						}
						break;
					case 2:
						if (packet_values[packet] == -1 || value < packet_values[packet]) {
							packet_values[packet] = value;
						}
						break;
					case 3:
						if (value > packet_values[packet]) {
							packet_values[packet] = value;
						}
						break;
					case 5:
						if (packet_values[packet] == -1) {
							packet_values[packet] = value;
						} else {
							packet_values[packet] = packet_values[packet] > value;
						}
						break;
					case 6:
						if (packet_values[packet] == -1) {
							packet_values[packet] = value;
						} else {
							packet_values[packet] = packet_values[packet] < value;
						}
						break;
					case 7:
						if (packet_values[packet] == -1) {
							packet_values[packet] = value;
						} else {
							packet_values[packet] = value == packet_values[packet];
						}
						break;
					default:
						break;
					}

					if (packet_subpackets[packet].first) {
						packet_subpackets[packet].second--;
					}
				}
				continue;
			}
		}

		version = data[i++] << 2;
		version += data[i++] << 1;
		version += data[i++];
		version_sum += version;
		type = data[i++] << 2;
		type += data[i++] << 1;
		type += data[i++];

		if (type == 4) {
			bool reading = true;
			int64_t value = 0;
			while (reading) {
				reading = data[i++];
				value <<= 4;
				for (uint8_t j = 0; j < 4; j++) {
					value += data[i++] << (3 - j);
				}
			}

			packets.push(i);
			packet_subpackets[i] = { true, 0 };
			packet_types[i] = type;
			packet_values[i] = value;
		} else {
			subpackets_number = data[i++];
			subpackets = 0;
			if (subpackets_number) {
				for (uint8_t j = 0; j < 11; j++) {
					subpackets += data[i++] << (10 - j);
				}
			} else {
				for (uint8_t j = 0; j < 15; j++) {
					subpackets += data[i++] << (14 - j);
				}
			}

			packets.push(i);
			packet_subpackets[i] = { subpackets_number, subpackets };
			packet_types[i] = type;
			packet_values[i] = -1;
		}
	} while (i <= data.size() && !packets.empty());

	std::cout << "The sum of all version headers is " << version_sum << '.' << std::endl;

	std::cout << "The result of the outer most packet is " << result << '.' << std::endl;
}
