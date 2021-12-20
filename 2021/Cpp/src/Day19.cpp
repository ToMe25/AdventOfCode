/*
 * Day19.cpp
 *
 *  Created on: 19.12.2021
 *      Author: ToMe25
 */

#include "Day19.h"
#include <algorithm>
#include <iostream>
#include <map>
#include <sstream>
#include <unordered_set>

template<>
void DayRunner<19>::solve(std::ifstream input) {
	std::string line;
	std::string token;
	Position pos;
	Scanner scanner;
	scanner.reserve(27);
	std::list<Scanner> scanners;
	while (std::getline(input, line)) {
		if (line.empty()) {
			scanners.push_back(scanner);
			scanner = Scanner();
			scanner.reserve(27);
		} else if (line[1] != '-') {
			std::replace(line.begin(), line.end(), ',', ' ');
			std::istringstream line_stream(line);
			pos = Position();
			uint16_t i = 0;
			while (line_stream >> token) {
				pos[i] = std::stoi(token);
				i++;
			}
			scanner.push_back(pos);
		}
	}

	if (!scanner.empty()) {
		scanners.push_back(scanner);
	}

	std::map<Scanner, Position> scanner_positions;
	scanner_positions[scanners.front()] = { 0, 0, 0 };
	scanners.pop_front();

	while (scanners.size() > 0) {
		for (std::pair<Scanner, Position> entry : scanner_positions) {
			for (std::pair<Scanner, Position> neighbor : get_neighbors(scanners,
					entry.first)) {
				scanner_positions[neighbor.first] = entry.second
						+ neighbor.second;
			}
		}
	}

	std::unordered_set<Position> global_map;
	for (std::pair<Scanner, Position> entry : scanner_positions) {
		Scanner translated_scanner = get_relative_to(entry.first, entry.second);
		global_map.insert(translated_scanner.begin(), translated_scanner.end());
	}

	std::cout << "There are a total of " << global_map.size()
			<< " unique beacons." << std::endl;

	uint16_t max_distance = 0;
	for (std::pair<Scanner, Position> first : scanner_positions) {
		for (std::pair<Scanner, Position> second : scanner_positions) {
			if (first != second) {
				uint16_t distance = abs(first.second[0] - second.second[0]);
				distance += abs(first.second[1] - second.second[1]);
				distance += abs(first.second[2] - second.second[2]);
				if (distance > max_distance) {
					max_distance = distance;
				}
			}
		}
	}

	std::cout << "The max manhattan distance between two scanners is "
			<< max_distance << '.' << std::endl;
}

std::map<Scanner, Position> get_neighbors(std::list<Scanner> &unknown_scanners,
		const Scanner scanner) {
	std::map<Scanner, Position> neighbors;
	std::unordered_set<Position> scanner_set;
	scanner_set.insert(scanner.begin(), scanner.end());

	for (std::list<Scanner>::iterator it = unknown_scanners.begin();
			it != unknown_scanners.end(); it++) {
		Scanner scanner_b = *it;
		if (scanner != scanner_b) {
			for (uint8_t rotation = 0; rotation < 64; rotation++) {
				Scanner scanner_b_rotated = rotate(scanner_b, rotation);
				for (Position pos : scanner) {
					for (Position pos_b : scanner_b_rotated) {
						Position offset = pos - pos_b;
						Scanner scanner_b_translated = get_relative_to(
								scanner_b_rotated, offset);

						Scanner intersection;
						intersection.reserve(27);
						for (Position pos : scanner_b_translated) {
							if (scanner_set.count(pos) > 0) {
								intersection.push_back(pos);
							}
						}

						if (intersection.size() >= 12) {
							unknown_scanners.erase(it++);
							it--;
							neighbors[scanner_b_rotated] = offset;
							goto LOOP_END;
						}
					}
				}
			}
		}
		LOOP_END: ;
	}

	return neighbors;
}

Scanner rotate(const Scanner scanner, const uint8_t rotation) {
	Scanner rotated(scanner);

	uint8_t x_rotation = rotation & 3;
	uint8_t y_rotation = rotation >> 2 & 3;
	uint8_t z_rotation = rotation >> 4 & 3;
	if (x_rotation == 1) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { pos[0], (int16_t) -pos[2], pos[1] });
				});
	} else if (x_rotation == 2) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { pos[0], (int16_t) -pos[1],
							(int16_t) -pos[2] });
				});
	} else if (x_rotation == 3) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { pos[0], pos[2], (int16_t) -pos[1] });
				});
	}

	if (y_rotation == 1) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { (int16_t) -pos[2], pos[1], pos[0] });
				});
	} else if (y_rotation == 2) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { (int16_t) -pos[0], pos[1],
							(int16_t) -pos[2] });
				});
	} else if (y_rotation == 3) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { pos[2], pos[1], (int16_t) -pos[0] });
				});
	}

	if (z_rotation == 1) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { (int16_t) -pos[1], pos[0], pos[2] });
				});
	} else if (z_rotation == 2) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { (int16_t) -pos[1], (int16_t) -pos[0],
							pos[2] });
				});
	} else if (z_rotation == 3) {
		std::transform(rotated.begin(), rotated.end(), rotated.begin(),
				[](Position pos) {
					return Position( { pos[1], (int16_t) -pos[0], pos[2] });
				});
	}

	return rotated;
}

Scanner get_relative_to(const Scanner scanner, const Position pos) {
	Scanner relative;

	for (Position pos1 : scanner) {
		relative.push_back(pos + pos1);
	}

	return relative;
}

constexpr Position operator+(const Position &pos_a, const Position &pos_b) {
	return {(int16_t) (pos_a[0] + pos_b[0]), (int16_t) (pos_a[1] + pos_b[1]),
		(int16_t) (pos_a[2] + pos_b[2])};
}

constexpr Position operator -(const Position &pos_a, const Position &pos_b) {
	return {(int16_t) (pos_a[0] - pos_b[0]), (int16_t) (pos_a[1] - pos_b[1]),
		(int16_t) (pos_a[2] - pos_b[2])};
}

std::ostream& operator <<(std::ostream &stream, const Position &pos) {
	stream << "Pos(" << pos[0] << ':' << pos[1] << ':' << pos[2] << ')';
	return stream;
}

std::ostream& operator <<(std::ostream &stream, const Scanner &scanner) {
	stream << "Scanner(";
	uint8_t first = true;
	for (Position pos : scanner) {
		if (first) {
			first = false;
		} else {
			stream << ',';
		}
		stream << pos;
	}
	stream << ')';
	return stream;
}

size_t std::hash<Position>::operator ()(const Position &pos) const {
	size_t hash_value = hash<uint8_t>()(pos[0]);
	hash_value ^= hash<uint8_t>()(pos[1]) << 1;
	hash_value >>= 1;
	hash_value ^= hash<uint16_t>()(pos[2]) << 1;
	return hash_value;
}
