/*
 * Day15.cpp
 *
 *  Created on: Dec 15, 2022
 *      Author: ToMe25
 */

#include "Day15.h"
#include <set>

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

bool aoc::sensor::convers_position(const int64_t x, const int64_t y) const {
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

std::vector<int64_t> aoc::sensor::covered_on_y(const int64_t y, const bool empty) const {
	const int64_t dist_y = abs(pos_y - y);
	if (dist_y > distance) {
		return std::vector<int64_t>();
	}

	std::vector<int64_t> positions;
	positions.reserve((distance - dist_y) * 2 + 1);
	for (int64_t x = pos_x - (int64_t) distance + dist_y; x <= pos_x + (int64_t) distance - dist_y; x++) {
		if (!empty || beacon_y != y || beacon_x != x) {
			positions.push_back(x);
		}
	}

	return positions;
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
		sensor_pos.first = std::stoi(line.substr(start_pos, end_pos - start_pos));
		start_pos = end_pos + 4;
		end_pos = line.find(':', start_pos);
		sensor_pos.second = std::stoi(line.substr(start_pos, end_pos - start_pos));

		start_pos = end_pos + 25;
		end_pos = line.find(',', start_pos);
		std::pair<int64_t, int64_t> beacon_pos;
		beacon_pos.first = std::stoi(line.substr(start_pos, end_pos - start_pos));
		start_pos = end_pos + 4;
		end_pos = line.length();
		beacon_pos.second = std::stoi(line.substr(start_pos, end_pos - start_pos));
		sensors.push_back(aoc::sensor(sensor_pos, beacon_pos));
	}

	std::set<int64_t> checked_positions;
	for (const aoc::sensor &s : sensors) {
		const std::vector<int64_t> posis = s.covered_on_y(checked_line, true);
		checked_positions.insert(posis.begin(), posis.end());
	}

	return std::to_string(checked_positions.size());
}

bool d15p1 = aoc::registerPart1(15, &day15part1);
