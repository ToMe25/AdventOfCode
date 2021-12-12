/*
 * Day12.cpp
 *
 *  Created on: 12.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <algorithm>
#include <cctype>
#include <map>
#include <iostream>
#include <vector>
#include <queue>

template <>
void DayRunner<12>::solve(std::ifstream input) {
	std::map<std::string, std::vector<std::string>> connections;
	std::string line;
	std::string first, second;
	while (input >> line) {
		size_t pos = line.find('-');
		first = line.substr(0, pos);
		second = line.substr(pos + 1);

		if (std::find(connections[first].begin(), connections[first].end(), second) == connections[first].end()) {
			connections[first].push_back(second);
		}

		if (std::find(connections[second].begin(), connections[second].end(), first) == connections[second].end()) {
			connections[second].push_back(first);
		}
	}

	std::vector<std::vector<std::string>> routes;
	std::queue<std::vector<std::string>> to_check;
	to_check.push({ "start" });
	while (!to_check.empty()) {
		std::vector<std::string> current = to_check.front();
		to_check.pop();
		for (std::string neighbor : connections[current.back()]) {
			if (neighbor == "end") {
				std::vector<std::string> copy = std::vector<std::string>(current);
				copy.push_back(neighbor);
				routes.push_back(copy);
			} else if (std::isupper(neighbor[0]) || std::find(current.begin(), current.end(), neighbor) == current.end()) {
				std::vector<std::string> copy = std::vector<std::string>(current);
				copy.push_back(neighbor);
				to_check.push(copy);
			}
		}
	}

	std::cout << "There are a total of " << routes.size() << " routes without visiting a single small cave twice." << std::endl;

	routes.resize(0);
	std::queue<std::pair<std::vector<std::string>, bool>> to_check_2;
	to_check_2.push({ { "start" }, false });
	while (!to_check_2.empty()) {
		std::pair<std::vector<std::string>, bool> current_pair = to_check_2.front();
		std::vector<std::string> current = current_pair.first;
		to_check_2.pop();
		for (std::string neighbor : connections[current.back()]) {
			if (neighbor == "end") {
				std::vector<std::string> copy = std::vector<std::string>(current);
				copy.push_back(neighbor);
				routes.push_back(copy);
			} else if (std::isupper(neighbor[0]) || std::find(current.begin(), current.end(), neighbor) == current.end()) {
				std::vector<std::string> copy = std::vector<std::string>(current);
				copy.push_back(neighbor);
				to_check_2.push({ copy, current_pair.second });
			} else if (!current_pair.second && neighbor != "start") {
				std::vector<std::string> copy = std::vector<std::string>(current);
				copy.push_back(neighbor);
				to_check_2.push({ copy, true });
			}
		}
	}

	std::cout << "There are a total of " << routes.size() << " routes when visiting a single small cave twice." << std::endl;
}
