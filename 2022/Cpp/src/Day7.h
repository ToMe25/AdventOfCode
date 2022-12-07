/*
 * Day7.h
 *
 *  Created on: Dec 7, 2022
 *      Author: ToMe25
 */

#ifndef DAY7_H_
#define DAY7_H_

#include "Main.h"

namespace aoc {

/**
 * A tree node representing a filesystem node, aka a file or directory.
 */
class FSNode {
public:
	/**
	 * A pointer to this nodes parent directory node.
	 * Only NULL for the root node.
	 */
	FSNode *const parent;

	/**
	 * The name of this file or directory.
	 */
	const std::string name;

	/**
	 * The size of this node, and all its children.
	 */
	size_t size;

	/**
	 * Whether this node is a directory.
	 * Since this is a simple FS, if it isn't one it has to be a file.
	 */
	bool directory;

	/**
	 * Creates a new filesystem node.
	 * Adds this node to parent as a child.
	 */
	FSNode(FSNode *const parent, const std::string name, const size_t size,
			const bool directory);

	/**
	 * Adds a child node to this node, and recursively updates the sizes of this node and all its parents.
	 */
	void addChild(FSNode *child);

	/**
	 * Gets the child node with the given name, or NULL if there is no child with that name.
	 */
	FSNode* getChildForName(const std::string name) const;

	/**
	 * Returns a vector containing pointers to all the child nodes of this node.
	 */
	std::vector<FSNode *> getChildren() const;

private:
	/**
	 * A vector containing pointers to all this nodes children.
	 * Should never contain NULL.
	 */
	std::unordered_map<std::string, FSNode*> children;
};

}

/**
 * Prints the given node and all its children to the given output stream.
 */
std::ostream& operator<<(std::ostream &stream, const aoc::FSNode &node);

#endif /* DAY7_H_ */
