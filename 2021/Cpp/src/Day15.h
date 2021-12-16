/*
 * Day15.h
 *
 *  Created on: 15.12.2021
 *      Author: ToMe25
 */

#ifndef DAY15_H_
#define DAY15_H_

#include "Main.h"
#include <list>

struct Node {
	uint16_t x;
	uint16_t y;
	uint8_t cost;
	uint16_t cost_sum = 0;
	uint16_t f = 0;
	const Node *predecessor = NULL;

	/**
	 * Creates a new empty node with 0 as every value.
	 */
	Node(): x(0), y(0), cost(0) {
	}

	/**
	 * Creates a new node with the given values.
	 *
	 * @param x		The x coordinate of the new node.
	 * @param y 	The y coordinate of the new node.
	 * @param cost	The cost for entering the new node.
	 */
	Node(uint16_t x, uint16_t y, uint8_t cost) :
			x(x), y(y), cost(cost) {
	}

	/**
	 * Destroys this node.
	 */
	virtual ~Node() {
	}

	/**
	 * Checks whether this node and the given other node are equal.
	 * This is done by comparing the x and y coordinate, as well as the cost.
	 *
	 * @param other	The node to compare this node with.
	 * @return	True if the nodes are equal.
	 */
	bool operator ==(const Node &other) const {
		return x == other.x && y == other.y && cost == other.cost;
	}

	/**
	 * Checks whether this node is less the the given other node.
	 * This is done by comparing the f value.
	 *
	 * @param other	The node to compare this node with.
	 * @return	True if this node is less then the other node.
	 */
	bool operator <(const Node &other) const {
		return f < other.f;
	}

	/**
	 * Checks whether this node is greater the the given other node.
	 * This is done by comparing the f value.
	 *
	 * @param other	The node to compare this node with.
	 * @return	True if this node is greater then the other node.
	 */
	bool operator >(const Node &other) const {
		return f > other.f;
	}
};

namespace std {
template<>
struct hash<Node> {
	/**
	 * Creates a hash for the given node.
	 *
	 * @param to_hash	The node for which to calculate the hash.
	 * @return The hash value for the given node.
	 */
	size_t operator ()(const Node to_hash) const;
};
}

/**
 * Prints a human readable string representation of the given node to the given ostream.
 * @param stream	The ostream to print to.
 * @param node		The node to convert and print.
 * @return	The given ostream.
 */
std::ostream &operator <<(std::ostream &stream, const Node &node);

struct node_pointer_less {
	/**
	 * Compares the nodes from the two given pointers.
	 *
	 * @param A	The first node to compare.
	 * @param B	The second node to compare.
	 * @return	True if the node from A is less then the node from B.
	 */
	bool operator ()(const Node *A, const Node *B) const {
		return *A < *B;
	}
};

/**
 * Uses a standard A* algorithm to find the best path from the top left to the bottom right of the map.
 *
 * @tparam Size	The x and y size of the map.
 * @param map	The map on which to search for the best path.
 * @return	The node for the bottom right position.
 */
template<size_t Size>
Node find_path(Node map[Size][Size]);

/**
 * Gets the nodes for the fields neighboring the given node.
 *
 * @tparam Size	The x and y size of the map.
 * @param current	The node for which to look for neighbors.
 * @param map		The map in which to check the cost for the neighbors.
 * 					Used to prevent creating duplicate nodes for the same position.
 * @return	A list containing those of the four neighbors of the given node that exist.
 */
template<size_t Size>
std::list<Node*> get_neighbors(Node *current,
		const Node map[Size][Size]);

#endif /* DAY15_H_ */
