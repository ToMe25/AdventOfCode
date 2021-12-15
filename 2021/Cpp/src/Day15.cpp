/*
 * Day15.cpp
 *
 *  Created on: 15.12.2021
 *      Author: ToMe25
 */

#include "Day15.h"
#include <algorithm>
#include <iostream>
#include <set>
#include <unordered_set>

template<>
void DayRunner<15>::solve(std::ifstream input) {
	const uint8_t MAP_SIZE = 100;

	std::string line;
	uint8_t x = 0;
	uint8_t y = 0;
	std::array<std::array<uint8_t, MAP_SIZE>, MAP_SIZE> map;
	while (input >> line) {
		for (char c : line) {
			map[y][x] = c - '0';
			x++;
		}
		x = 0;
		y++;
	}

	Node goal = find_path(map);

	std::cout << "The cost for the easiest path for part one has a cost of "
			<< goal.cost_sum << '.' << std::endl;

	std::array<std::array<uint8_t, MAP_SIZE * 5>, MAP_SIZE * 5> map_2;
	for (uint8_t i = 0; i < 5; i++) {
		for (uint8_t j = 0; j < 5; j++) {
			for (uint8_t y = 0; y < MAP_SIZE; y++) {
				for (uint8_t x = 0; x < MAP_SIZE; x++) {
					map_2[i * MAP_SIZE + y][j * MAP_SIZE + x] = (map[y][x] + i + j) % 9;
					if (map_2[i * MAP_SIZE + y][j * MAP_SIZE + x] == 0) {
						map_2[i * MAP_SIZE + y][j * MAP_SIZE + x] = 9;
					}
				}
			}
		}
	}

	goal = find_path(map_2);

	std::cout << "The cost for the easiest path for part two has a cost of "
			<< goal.cost_sum << '.' << std::endl;
}

template<size_t Size>
Node find_path(const std::array<std::array<uint8_t, Size>, Size> map) {
	std::multiset<Node> open;
	std::unordered_set<Node> closed;

	open.insert(Node(0, 0, map[0][0]));

	Node current;
	std::multiset<Node>::iterator it;
	while (open.size() > 0) {
		it = open.begin();
		current = *it;
		open.erase(it);

		if (current.y == Size - 1 && current.x == Size - 1) {
			break;
		}

		closed.insert(current);

		for (Node neighbor : get_neighbors(current, map, open)) {
			if (closed.count(neighbor) == 0) {
				uint16_t tentative = current.cost_sum + neighbor.cost;

				if (std::count(open.begin(), open.end(), neighbor) > 0
						&& neighbor.cost_sum <= tentative) {
					continue;
				}

				if ((it = open.find(neighbor)) != open.end()) {
					open.erase(it);
				}

				neighbor.predecessor = &*closed.find(current);
				neighbor.cost_sum = tentative;
				neighbor.f = tentative + Size * 2 - 2
						- neighbor.x - neighbor.y;
				open.insert(neighbor);
			}
		}
	}

	return current;
}

template<size_t Size>
std::vector<Node> get_neighbors(const Node current,
		const std::array<std::array<uint8_t, Size>, Size> map,
		const std::multiset<Node> open) {
	std::vector<Node> neighbors;

	std::set<Node>::iterator it;
	Node new_node;
	if (current.x > 0) {
		new_node = Node(current.x - 1, current.y,
				map[current.y][current.x - 1]);
		if ((it = open.find(new_node)) != open.end()) {
			neighbors.push_back(*it);
		} else {
			neighbors.push_back(new_node);
		}
	}

	if (current.x < Size - 1) {
		new_node = Node(current.x + 1, current.y,
				map[current.y][current.x + 1]);
		if ((it = open.find(new_node)) != open.end()) {
			neighbors.push_back(*it);
		} else {
			neighbors.push_back(new_node);
		}
	}

	if (current.y > 0) {
		new_node = Node(current.x, current.y - 1,
				map[current.y - 1][current.x]);
		if ((it = open.find(new_node)) != open.end()) {
			neighbors.push_back(*it);
		} else {
			neighbors.push_back(new_node);
		}
	}

	if (current.y < Size - 1) {
		new_node = Node(current.x, current.y + 1,
				map[current.y + 1][current.x]);
		if ((it = open.find(new_node)) != open.end()) {
			neighbors.push_back(*it);
		} else {
			neighbors.push_back(new_node);
		}
	}

	return neighbors;
}

std::ostream& operator <<(std::ostream &stream, const Node &node) {
	stream << "Node(pos=";
	stream << (uint16_t) node.x;
	stream << ':';
	stream << (uint16_t) node.y;
	stream << ", cost=";
	stream << (uint16_t) node.cost;
	stream << '(';
	stream << node.cost_sum;
	stream << "), f=";
	stream << node.f;
	stream << ')';
	return stream;
}

size_t std::hash<Node>::operator ()(const Node to_hash) const {
	size_t hash_value = hash<uint8_t>()(to_hash.x);
	hash_value ^= hash<uint8_t>()(to_hash.y) << 1;
	hash_value >>= 1;
	hash_value ^= hash<uint16_t>()(to_hash.cost) << 1;
	return hash_value;
}
