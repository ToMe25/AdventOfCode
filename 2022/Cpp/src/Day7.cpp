/*
 * Day7.cpp
 *
 *  Created on: Dec 7, 2022
 *      Author: ToMe25
 */

#include "Day7.h"

aoc::FSNode::FSNode(FSNode *const parent, const std::string name,
		const size_t size, const bool directory) :
		parent(parent), name(name), size(size), directory(directory) {
	if (parent != NULL) {
		parent->addChild(this);
	}
}

void aoc::FSNode::addChild(FSNode *child) {
	if (!directory) {
		std::cerr << "Trying to add child node to a file." << std::endl;
		return;
	}

	if (child == NULL) {
		std::cerr << "Trying to add NULL child to node." << std::endl;
		return;
	}

	children[child->name] = child;
	size += child->size;
	FSNode *current = parent;
	while (current != NULL) {
		current->size += child->size;
		current = current->parent;
	}
}

aoc::FSNode* aoc::FSNode::getChildForName(const std::string name) const {
	if (children.count(name) == 0) {
		return NULL;
	} else {
		return children.at(name);
	}
}

std::vector<aoc::FSNode*> aoc::FSNode::getChildren() const {
	std::vector<FSNode*> result;
	for (std::pair<std::string, FSNode*> entry : children) {
		result.push_back(entry.second);
	}
	return result;
}

std::ostream& operator<<(std::ostream &stream, const aoc::FSNode &node) {
	std::vector<std::pair<const aoc::FSNode*, size_t>> stack;
	stack.push_back( { &node, 0 });
	while (stack.size() > 0) {
		const aoc::FSNode *current = stack.back().first;
		const size_t depth = stack.back().second;
		stack.pop_back();

		for (size_t i = 0; i < depth; i++) {
			stream << "  ";
		}
		stream << "- " << current->name << " (type=dir, size=" << current->size << ')'
				<< std::endl;

		for (aoc::FSNode *node : current->getChildren()) {
			if (node == NULL) {
				std::cerr << "Trying to print NULL child node." << std::endl;
				continue;
			}

			if (node->directory) {
				stack.push_back( { node, depth + 1 });
			} else {
				for (size_t i = 0; i < depth + 1; i++) {
					stream << "  ";
				}
				stream << "- " << node->name << " (type=file, size=" << node->size << ')'
						<< std::endl;
			}
		}
	}

	return stream;
}

std::pair<std::string, std::string> day7Combined(std::ifstream input) {
	aoc::FSNode *root = new aoc::FSNode(NULL, "/", 0, true);
	aoc::FSNode *current = root;

	std::string line;
	while (std::getline(input, line)) {
		const size_t start_pos = line.find_first_not_of(" \t");

		if (line.length() == 0 || start_pos == std::string::npos) {
			continue;
		}
		line = line.substr(start_pos,
				line.find_last_not_of(" \t\r\n") - start_pos + 1);

		// Input is a command
		if (line[0] == '$') {
			const std::string cmd = line.substr(2, 2);
			if (cmd == "cd") {
				const std::string dir = line.substr(5);
				if (dir == "/") {
					current = root;
				} else if (dir == "..") {
					current = current->parent;
					if (current == NULL) {
						std::cerr << "Trying to go to a parent of root."
								<< std::endl;
						current = root;
					}
				} else {
					current = current->getChildForName(dir);
					if (current == NULL) {
						std::cerr << "Trying to go to nonexistent directory "
								<< dir << '.' << std::endl;
						current = root;
					} else if (!current->directory) {
						std::cerr << "Trying to enter a file." << std::endl;
						current = current->parent;
					}
				}
			} else if (cmd == "ls") {
				// Do nothing.
			}
		} else {
			if (line.substr(0, 3) == "dir") {
				new aoc::FSNode(current, line.substr(4), 0, true);
			} else {
				const size_t space = line.find(' ');
				if (space == std::string::npos) {
					std::cerr << "File declaration didn't contain a space."
							<< std::endl;
					continue;
				}

				new aoc::FSNode(current, line.substr(space + 1),
						std::stoi(line.substr(0, space)), false);
			}
		}
	}

	size_t part1sum = 0;
	std::vector<const aoc::FSNode*> stack;
	stack.push_back(root);
	while (stack.size() > 0) {
		const aoc::FSNode *current = stack.back();
		stack.pop_back();
		if (current->size <= 100000) {
			part1sum += current->size;
		}

		for (const aoc::FSNode *node : current->getChildren()) {
			if (node->directory) {
				stack.push_back(node);
			}
		}
	}

	const size_t missingSpace = 30000000 - (70000000 - root->size);
	size_t min = root->size;
	stack.push_back(root);
	while (stack.size() > 0) {
		const aoc::FSNode *current = stack.back();
		stack.pop_back();
		if (current->size >= missingSpace && current->size < min) {
			min = current->size;
		}

		for (const aoc::FSNode *node : current->getChildren()) {
			if (node->directory) {
				stack.push_back(node);
			}
		}
	}

	return {std::to_string(part1sum), std::to_string(min)};
}

bool d7c = aoc::registerCombined(7, &day7Combined);
