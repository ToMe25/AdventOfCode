/*
 * Day22.cpp
 *
 *  Created on: 22.12.2021
 *      Author: ToMe25
 */

#include "Day22.h"
#include <iostream>
#include <list>
#include <regex>
#include <sstream>

template<>
void DayRunner<22>::solve(std::ifstream input) {
	std::string line;
	bool on = false;
	int32_t coords[6];
	std::string token;
	std::list<Region> on_regions;
	while (std::getline(input, line)) {
		std::replace(line.begin(), line.end(), ',', ' ');
		line = std::regex_replace(line, std::regex("\\.\\."), " ..");

		std::istringstream line_stream(line);
		uint8_t i = 0;
		while (line_stream >> token) {
			if (i == 0) {
				on = token == "on";
			} else {
				coords[i - 1] = std::stoi(token.substr(2));
			}
			i++;
		}

		Region new_region(coords);
		if (new_region.empty()) {
			continue;
		}

		std::list<Region> new_regions;
		for (Region reg : on_regions) {
			if (reg.intersects(new_region)) {
				std::vector<Region> outside = reg.outside_intersection(new_region);
				if (outside.size() > 1 || (outside.size() == 1 && outside[0] != reg)) {
					for (Region r : outside) {
						new_regions.push_back(r);
					}
				}
			} else {
				new_regions.push_back(reg);
			}
		}

		if (on && !new_region.empty()) {
			new_regions.push_back(new_region);
		}

		on_regions = new_regions;
	}

	uint64_t active = 0;
	Region limit(-50, 50, -50, 50, -50, 50);
	for (Region region : on_regions) {
		active += region.intersection(limit).size;
	}

	std::cout << "There are " << active
			<< " on positions in the -50:-50:-50 to 50:50:50 area."
			<< std::endl;

	active = 0;
	for (Region region : on_regions) {
		active += region.size;
	}

	std::cout << "There are a total of " << active << " on cubes." << std::endl;
}

bool Region::intersects(const Region &other) const {
	if (empty() || other.empty()) {
		return false;
	}

	if (min_x > other.max_x || other.min_x > max_x) {
		return false;
	}

	if (min_y > other.max_y || other.min_y > max_y) {
		return false;
	}

	if (min_z > other.max_z || other.min_z > max_z) {
		return false;
	}

	return true;
}

Region Region::intersection(const Region &other) const {
	if (!intersects(other)) {
		return Region();
	}

	int32_t new_min_x = std::max(min_x, other.min_x);
	int32_t new_max_x = std::min(max_x, other.max_x);

	int32_t new_min_y = std::max(min_y, other.min_y);
	int32_t new_max_y = std::min(max_y, other.max_y);

	int32_t new_min_z = std::max(min_z, other.min_z);
	int32_t new_max_z = std::min(max_z, other.max_z);

	return Region(new_min_x, new_max_x, new_min_y, new_max_y, new_min_z,
			new_max_z);
}

std::vector<Region> Region::outside_intersection(const Region &other) const {
	std::vector<Region> regions;
	regions.reserve(6);

	if (!intersects(other)) {
		regions.push_back(*this);
		return regions;
	}

	Region inter = intersection(other);

	if (min_x < inter.min_x) {
		regions.push_back(
				Region(min_x, inter.min_x - 1, min_y, max_y, min_z, max_z));
	}

	if (max_x > inter.max_x) {
		regions.push_back(
				Region(inter.max_x + 1, max_x, min_y, max_y, min_z, max_z));
	}

	if (min_y < inter.min_y) {
		regions.push_back(
				Region(inter.min_x, inter.max_x, min_y, inter.min_y - 1, min_z,
						max_z));
	}

	if (max_y > inter.max_y) {
		regions.push_back(
				Region(inter.min_x, inter.max_x, inter.max_y + 1, max_y, min_z,
						max_z));
	}

	if (min_z < inter.min_z) {
		regions.push_back(
				Region(inter.min_x, inter.max_x, inter.min_y, inter.max_y,
						min_z, inter.min_z - 1));
	}

	if (max_z > inter.max_z) {
		regions.push_back(
				Region(inter.min_x, inter.max_x, inter.min_y, inter.max_y,
						inter.max_z + 1, max_z));
	}

	return regions;
}

bool Region::operator ==(const Region &other) const {
	return min_x == other.min_x && max_x == other.max_x && min_y == other.min_y
			&& max_y == other.max_y && min_z == other.min_z
			&& max_z == other.max_z;
}

uint64_t Region::calculate_size() const {
	uint64_t size = abs(max_x - min_x) + 1;
	size *= abs(max_y - min_y) + 1;
	size *= abs(max_z - min_z) + 1;
	return size;
}

std::ostream& operator <<(std::ostream &stream, const Region &region) {
	stream << "Region(x=" << region.min_x << ".." << region.max_x;
	stream << ", y=" << region.min_y << ".." << region.max_y;
	stream << ", z=" << region.min_z << ".." << region.max_z;
	stream << ", size=" << region.size << ')';
	return stream;
}
