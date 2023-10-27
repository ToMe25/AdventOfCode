/*
 * Day11.cpp
 *
 *  Created on: Dec 11, 2022
 *      Author: ToMe25
 */

#include "Day11.h"
#include <algorithm>
#include <cctype>

aoc::Monkey::Monkey(const uint8_t id, const std::vector<uint64_t> start_items,
		const bool operation_multiply, const bool operation_const,
		const uint8_t operation_second, const bool relief_mod,
		const uint32_t relief_divisor, const uint8_t test_divisor,
		Monkey *true_target, Monkey *false_target) :
		id(id), op_mul(operation_multiply), op_const(operation_const), op_sec(
				operation_second), rel_mod(relief_mod), rel_div(relief_divisor), test_div(
				test_divisor), true_tgt(true_target), false_tgt(false_target), items(
				start_items) {
}

aoc::Monkey::Monkey(const Monkey &monkey) :
		id(monkey.id), op_mul(monkey.op_mul), op_const(monkey.op_const), op_sec(
				monkey.op_sec), rel_mod(monkey.rel_mod), rel_div(
				monkey.rel_div), test_div(monkey.test_div), true_tgt(
				monkey.true_tgt), false_tgt(monkey.false_tgt), items(
				monkey.items) {
}

void aoc::Monkey::set_targets(Monkey *true_target, Monkey *false_target) {
	true_tgt = true_target;
	false_tgt = false_target;
}

void aoc::Monkey::set_relief(const bool relief_mod,
		const uint32_t relief_divisor) {
	rel_mod = relief_mod;
	rel_div = relief_divisor;
}

uint64_t aoc::Monkey::get_inspected() const {
	return inspected;
}

uint32_t aoc::Monkey::get_test_divisor() const {
	return test_div;
}

void aoc::Monkey::turn() {
	for (size_t i = 0; i < items.size(); i++) {
		if (op_mul) {
			items[i] *= (op_const ? op_sec : items[i]);
		} else {
			items[i] += (op_const ? op_sec : items[i]);
		}
		inspected++;

		if (rel_mod) {
			items[i] %= rel_div;
		} else {
			items[i] /= rel_div;
		}

		if (items[i] % test_div == 0) {
			if (true_tgt == NULL) {
				std::cerr << "Monkey " << (uint16_t) id
						<< " tried to throw an item to a null monkey."
						<< std::endl;
			} else {
				true_tgt->items.push_back(items[i]);
			}
		} else {
			if (false_tgt == NULL) {
				std::cerr << "Monkey " << (uint16_t) id
						<< " tried to throw an item to a null monkey."
						<< std::endl;
			} else {
				false_tgt->items.push_back(items[i]);
			}
		}
	}
	items.clear();
}

std::vector<aoc::Monkey> aoc::parse_monkeys(std::vector<std::string> lines) {
	std::vector<aoc::Monkey> monkeys;
	std::vector<std::pair<uint8_t, uint8_t>> targets;
	uint8_t id = 0;
	std::vector<uint64_t> items;
	bool operation_multiply = false;
	// -1 to use old value.
	int8_t operation_second = 0;
	uint8_t test_divisor = 0;
	//Which of the 5 lines of a monkey definition are we parsing
	uint8_t ln = 0;
	for (std::string line : lines) {
		if (line.length() == 0) {
			continue;
		}

		bool err = false;
		switch (ln) {
		case 0:
			if (line.substr(0, 6) != "Monkey") {
				std::cerr << "Trying to parse invalid id line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				id = std::stoi(line.substr(7, line.length() - 8));
			}
			break;
		case 1:
			if (line.substr(0, 14) != "Starting items") {
				std::cerr << "Trying to parse invalid items line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				items = std::vector<uint64_t>();
				line = line.substr(16);
				size_t comma;
				while ((comma = line.find(',')) != std::string::npos) {
					items.push_back(std::stoi(line.substr(0, comma)));
					line = line.substr(comma + 2);
				}
				items.push_back(std::stoi(line));
			}
			break;
		case 2:
			if (line.substr(0, 9) != "Operation") {
				std::cerr << "Trying to parse invalid operation line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				operation_multiply = line[21] == '*';
				line = line.substr(23);
				if (line == "old") {
					operation_second = -1;
				} else {
					operation_second = std::stoi(line);
				}
			}
			break;
		case 3:
			if (line.substr(0, 4) != "Test") {
				std::cerr << "Trying to parse invalid test line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				test_divisor = std::stoi(line.substr(19));
			}
			break;
		case 4:
			if (line.substr(0, 7) != "If true") {
				std::cerr << "Trying to parse invalid if true line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				targets.push_back( { std::stoi(line.substr(25)), 0 });
			}
			break;
		case 5:
			if (line.substr(0, 8) != "If false") {
				std::cerr << "Trying to parse invalid if false line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				targets.back().second = std::stoi(line.substr(26));
			}
			break;
		}

		if (err) {
			ln = 0;
		} else if (ln == 5) {
			monkeys.push_back(
					aoc::Monkey(id, items, operation_multiply,
							operation_second != -1,
							operation_second == -1 ? 0 : operation_second,
							false, 3, test_divisor));
			ln = 0;
		} else {
			ln++;
		}
	}

	for (size_t i = 0; i < monkeys.size(); i++) {
		monkeys[i].set_targets(&monkeys[targets[i].first],
				&monkeys[targets[i].second]);
	}

	return monkeys;
}

std::ostream& aoc::operator<<(std::ostream &stream, const aoc::Monkey &monkey) {
	stream << "Monkey " << (uint16_t) monkey.id << ": ";
	for (size_t i = 0; i < monkey.items.size(); i++) {
		stream << monkey.items[i];
		if (i < monkey.items.size() - 1) {
			stream << ", ";
		}
	}
	return stream;
}

std::string day11part1(std::ifstream input) {
	std::vector<std::string> lines;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() > 0) {
			line.erase(line.begin(),
					std::find_if_not(line.begin(), line.end(),
							(int (*)(int)) (std::isspace)));
			lines.push_back(line);
		}
	}
	std::vector<aoc::Monkey> monkeys = aoc::parse_monkeys(lines);

	for (uint8_t i = 0; i < 20; i++) {
		for (aoc::Monkey &monkey : monkeys) {
			monkey.turn();
		}
	}

	std::vector<uint32_t> inspections;
	for (const aoc::Monkey &m : monkeys) {
		inspections.push_back(m.get_inspected());
	}
	std::sort(inspections.begin(), inspections.end(), std::greater());

	return std::to_string(inspections[0] * inspections[1]);
}

std::string day11part2(std::ifstream input) {
	std::vector<std::string> lines;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() > 0) {
			line.erase(line.begin(),
					std::find_if_not(line.begin(), line.end(),
							(int (*)(int)) (std::isspace)));
			lines.push_back(line);
		}
	}
	std::vector<aoc::Monkey> monkeys = aoc::parse_monkeys(lines);

	uint64_t lcm = monkeys[0].get_test_divisor();
	uint64_t gcd, a, b;
	for (const aoc::Monkey &monkey : monkeys) {
		gcd = monkey.get_test_divisor();
		a = lcm;
		while (a != 0) {
			b = a;
			a = gcd % a;
			gcd = b;
		}
		lcm = (monkey.get_test_divisor() * lcm) / gcd;
	}

	for (aoc::Monkey &m : monkeys) {
		m.set_relief(true, lcm);
	}

	for (uint16_t i = 0; i < 10000; i++) {
		for (aoc::Monkey &monkey : monkeys) {
			monkey.turn();
		}
	}

	std::vector<uint64_t> inspections;
	for (const aoc::Monkey &m : monkeys) {
		inspections.push_back(m.get_inspected());
	}
	std::sort(inspections.begin(), inspections.end(), std::greater());

	return std::to_string(inspections[0] * inspections[1]);
}

bool d11p1 = aoc::registerPart1(11, &day11part1);
bool d11p2 = aoc::registerPart2(11, &day11part2);
