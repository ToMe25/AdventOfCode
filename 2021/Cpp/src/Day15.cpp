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
	Node map[MAP_SIZE][MAP_SIZE];
	while (input >> line) {
		for (char c : line) {
			map[y][x] = Node(x, y, c - '0');
			x++;
		}
		x = 0;
		y++;
	}

	Node goal = find_path<MAP_SIZE>(map);

	std::cout << "The cost for the easiest path for part one has a cost of "
			<< goal.cost_sum << '.' << std::endl;

	Node map_2[MAP_SIZE * 5][MAP_SIZE * 5];
	for (uint8_t i = 0; i < 5; i++) {
		for (uint8_t j = 0; j < 5; j++) {
			for (uint8_t y = 0; y < MAP_SIZE; y++) {
				for (uint8_t x = 0; x < MAP_SIZE; x++) {
					uint8_t cost = (map[y][x].cost + i + j) % 9;
					if (cost == 0) {
						cost = 9;
					}
					map_2[i * MAP_SIZE + y][j * MAP_SIZE + x] = Node(
							j * MAP_SIZE + x, i * MAP_SIZE + y, cost);
				}
			}
		}
	}

	goal = find_path<MAP_SIZE * 5>(map_2);

	std::cout << "The cost for the easiest path for part two has a cost of "
			<< goal.cost_sum << '.' << std::endl;
}

template<size_t Size>
Node find_path(Node map[Size][Size]) {
	std::multiset<Node*, node_pointer_less> open;
	std::unordered_set<const Node*> closed;

	open.insert(&map[0][0]);

	std::multiset<Node*>::iterator it;
	const Node *current = NULL;
	while (open.size() > 0) {
		it = open.begin();
		current = *it;
		open.erase(it);

		if (current->y == Size - 1 && current->x == Size - 1) {
			break;
		}

		closed.insert(current);

		for (Node *neighbor : get_neighbors<Size>(current, map)) {
			if (closed.count(neighbor) == 0) {
				uint16_t tentative = current->cost_sum + neighbor->cost;

				if (neighbor->cost_sum > 0 && neighbor->cost_sum <= tentative) {
					continue;
				}

				neighbor->predecessor = current;
				neighbor->cost_sum = tentative;
				neighbor->f = tentative + Size * 2 - 2 - neighbor->x - neighbor->y;

				open.insert(neighbor);
			}
		}
	}

	return *current;
}

template<size_t Size>
std::list<Node*> get_neighbors(const Node *current,
		Node map[Size][Size]) {
	std::list<Node*> neighbors;

	if (current->x > 0) {
		neighbors.push_back(&map[current->y][current->x - 1]);
	}

	if (current->x < Size - 1) {
		neighbors.push_back(&map[current->y][current->x + 1]);
	}

	if (current->y > 0) {
		neighbors.push_back(&map[current->y - 1][current->x]);
	}

	if (current->y < Size - 1) {
		neighbors.push_back(&map[current->y + 1][current->x]);
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
