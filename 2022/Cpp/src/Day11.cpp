/*
 * Day11.cpp
 *
 *  Created on: Dec 11, 2022
 *      Author: ToMe25
 */

#include "Day11.h"
#include <cctype>

aoc::Monkey::Monkey(const uint8_t id, const std::vector<uint32_t> start_items,
		const bool operation_multiply, const bool operation_const,
		const uint8_t operation_second, const uint8_t test_divisor,
		Monkey *true_target, Monkey *false_target) :
		id(id), items(start_items), op_mul(operation_multiply), op_const(
				operation_const), op_sec(operation_second), test_div(
				test_divisor), true_tgt(true_target), false_tgt(false_target) {
}

aoc::Monkey::Monkey(const Monkey &monkey) :
		id(monkey.id), items(monkey.items), op_mul(monkey.op_mul), op_const(
				monkey.op_const), op_sec(monkey.op_sec), test_div(
				monkey.test_div), true_tgt(monkey.true_tgt), false_tgt(
				monkey.false_tgt) {
}

void aoc::Monkey::set_targets(Monkey *true_target, Monkey *false_target) {
	true_tgt = true_target;
	false_tgt = false_target;
}

uint32_t aoc::Monkey::get_inspected() const {
	return inspected;
}

void aoc::Monkey::turn() {
	for (size_t i = 0; i < items.size(); i++) {
		if (op_mul) {
			items[i] *= (op_const ? op_sec : items[i]);
		} else {
			items[i] += (op_const ? op_sec : items[i]);
		}
		inspected++;
		items[i] /= 3;
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
	std::vector<aoc::Monkey> monkeys;
	std::vector<std::pair<uint8_t, uint8_t>> targets;
	uint8_t id;
	std::vector<uint32_t> items;
	bool operation_multiply;
	// -1 to use old value.
	int8_t operation_second;
	uint8_t test_divisor;
	//Which of the 5 lines of a monkey definition are we parsing
	uint8_t ln = 0;
	std::string line;
	while (std::getline(input, line)) {
		line.erase(line.begin(),
				std::find_if_not(line.begin(), line.end(),
						(int (*)(int)) (std::isspace)));
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
				id = std::stoi(line.substr(7));
			}
			break;
		case 1:
			if (line.substr(0, 14) != "Starting items") {
				std::cerr << "Trying to parse invalid items line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				items = std::vector<uint32_t>();
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
				std::cerr << "Trying to parse invalid test line \"" << line
						<< "\"." << std::endl;
				err = true;
			} else {
				targets.push_back( { std::stoi(line.substr(25)), 0 });
			}
			break;
		case 5:
			if (line.substr(0, 8) != "If false") {
				std::cerr << "Trying to parse invalid test line \"" << line
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
							test_divisor));
			ln = 0;
		} else {
			ln++;
		}
	}

	for (size_t i = 0; i < monkeys.size(); i++) {
		monkeys[i].set_targets(&monkeys[targets[i].first], &monkeys[targets[i].second]);
	}

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

bool d11p1 = aoc::registerPart1(11, &day11part1);
