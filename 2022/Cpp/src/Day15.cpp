/*
 * Day15.cpp
 *
 *  Created on: Dec 15, 2022
 *      Author: ToMe25
 */

#include "Day15.h"

aoc::sensor::sensor(const std::pair<int64_t, int64_t> sensor_pos,
		const std::pair<int64_t, int64_t> beacon_pos) :
		pos_x(sensor_pos.first), pos_y(sensor_pos.second), beacon_x(
				beacon_pos.first), beacon_y(beacon_pos.second), distance(
				abs(pos_x - beacon_x) + abs(pos_y - beacon_y)) {
}

std::pair<int64_t, int64_t> aoc::sensor::get_sensor_pos() const {
	return std::pair<int64_t, int64_t>(pos_x, pos_y);
}

std::pair<int64_t, int64_t> aoc::sensor::get_beacon_pos() const {
	return std::pair<int64_t, int64_t>(beacon_x, beacon_y);
}

uint64_t aoc::sensor::get_distance() const {
	return distance;
}

bool aoc::sensor::covers_position(const int64_t x, const int64_t y) const {
	const uint64_t x_dist = abs(pos_x - x);
	if (x_dist > distance) {
		return false;
	}

	const uint64_t y_dist = abs(pos_y - y);
	if (y_dist > distance) {
		return false;
	}

	return x_dist + y_dist <= distance;
}

void aoc::sensor::add_covered_on_y(std::unordered_set<int64_t> &positions,
		const int64_t pos_y, const bool empty) const {
	const uint64_t dist_y = abs(this->pos_y - pos_y);
	if (dist_y > distance) {
		return;
	}

	for (int64_t x = (int64_t) (pos_x - distance + dist_y);
			x <= (int64_t) (pos_x + distance - dist_y); x++) {
		if (!empty || beacon_y != pos_y || beacon_x != x) {
			positions.insert(x);
		}
	}
}

uint64_t aoc::sensor::distance_to(const int64_t x, const int64_t y) const {
	return abs(pos_x - x) + abs(pos_y - y);
}

std::ostream& aoc::operator<<(std::ostream &stream, const sensor &sensor) {
	stream << "sensor[pos=" << sensor.pos_x << ':' << sensor.pos_y;
	stream << ", beacon_pos=" << sensor.beacon_x << ':' << sensor.beacon_y;
	stream << ", distance=" << sensor.distance << ']';
	return stream;
}

std::string day15part1(std::ifstream input) {
	std::vector<aoc::sensor> sensors;
	const int64_t checked_line = 2000000;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		size_t start_pos = 12;
		size_t end_pos = line.find(',', start_pos);
		std::pair<int64_t, int64_t> sensor_pos;
		sensor_pos.first = std::stoi(
				line.substr(start_pos, end_pos - start_pos));
		start_pos = end_pos + 4;
		end_pos = line.find(':', start_pos);
		sensor_pos.second = std::stoi(
				line.substr(start_pos, end_pos - start_pos));

		start_pos = end_pos + 25;
		end_pos = line.find(',', start_pos);
		std::pair<int64_t, int64_t> beacon_pos;
		beacon_pos.first = std::stoi(
				line.substr(start_pos, end_pos - start_pos));
		start_pos = end_pos + 4;
		end_pos = line.length();
		beacon_pos.second = std::stoi(
				line.substr(start_pos, end_pos - start_pos));
		sensors.push_back(aoc::sensor(sensor_pos, beacon_pos));
	}

	std::unordered_set<int64_t> checked_positions;
	for (const aoc::sensor &s : sensors) {
		s.add_covered_on_y(checked_positions, checked_line, true);
	}

	return std::to_string(checked_positions.size());
}

std::string day15part2(std::ifstream input) {
	std::vector<aoc::sensor> sensors;
	const int64_t max_pos = 4000000;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		size_t start_pos = 12;
		size_t end_pos = line.find(',', start_pos);
		std::pair<int64_t, int64_t> sensor_pos;
		sensor_pos.first = std::stoi(
				line.substr(start_pos, end_pos - start_pos));
		start_pos = end_pos + 4;
		end_pos = line.find(':', start_pos);
		sensor_pos.second = std::stoi(
				line.substr(start_pos, end_pos - start_pos));

		start_pos = end_pos + 25;
		end_pos = line.find(',', start_pos);
		std::pair<int64_t, int64_t> beacon_pos;
		beacon_pos.first = std::stoi(
				line.substr(start_pos, end_pos - start_pos));
		start_pos = end_pos + 4;
		end_pos = line.length();
		beacon_pos.second = std::stoi(
				line.substr(start_pos, end_pos - start_pos));
		sensors.push_back(aoc::sensor(sensor_pos, beacon_pos));
	}

	int64_t x = 0;
	int64_t y = 0;
	bool covered = true;
	for (; y <= max_pos; y++) {
		while (x <= max_pos) {
			covered = false;
			for (const aoc::sensor &s : sensors) {
				if (s.covers_position(x, y)) {
					x += s.get_distance() - s.distance_to(x, y) + 1;
					if (x == s.get_sensor_pos().first) {
						x += s.get_distance() + 1;
					}
					covered = true;
					break;
				}
			}
			if (!covered) {
				break;
			}
		}
		if (!covered) {
			break;
		} else {
			x = 0;
		}
	}

	return std::to_string(x * 4000000 + y);
}

bool d15p1 = aoc::registerPart1(15, &day15part1);
bool d15p2 = aoc::registerPart2(15, &day15part2);
