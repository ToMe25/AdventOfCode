/*
 * Day24.cpp
 *
 *  Created on: 19.11.2022
 *      Author: ToMe25
 */

#include "Day24.h"
#include <cmath>
#include <iostream>
#include <vector>

template<>
void DayRunner<24>::solve(std::ifstream input) {
	std::vector<std::string> lines;

	std::string line;
	while (std::getline(input, line)) {
		if (line.length() > 0) {
			lines.push_back(line);
		}
	}

	size_t instrs = lines.size();
	Instruction instructions[instrs];
	for (size_t i = 0; i < instrs; i++) {
		line = lines[i];
		InstType type = InstType::NUL;
		switch (line[0]) {
		case 'i':
			type = InstType::INP;
			break;
		case 'a':
			type = InstType::ADD;
			break;
		case 'm':
			if (line[1] == 'u') {
				type = InstType::MUL;
			} else if (line[1] == 'o') {
				type = InstType::MOD;
			} else {
				std::cerr << "Line \"" << line
						<< "\" contained an invalid instruction type."
						<< std::endl;
			}
			break;
		case 'd':
			type = InstType::DIV;
			break;
		case 'e':
			type = InstType::EQL;
			break;
		default:
			std::cerr << "Line \"" << line
					<< "\" contained an invalid instruction type." << std::endl;
		}

		if (type == InstType::NUL) {
			continue;
		}

		uint8_t reg_a = line[4] - 'w';

		int32_t in_b = 0;
		bool const_b = true;
		if (type != InstType::INP) {
			if (line[6] <= 'z' && line[6] >= 'w') {
				const_b = false;
				in_b = line[6] - 'w';
			} else {
				in_b = std::stoi(line.substr(6));
			}
		}

		instructions[i] = Instruction(type, reg_a, const_b, in_b);
	}

	std::array<int64_t, 4> result { 0, 0, 0, 1 };
	uint8_t num_in[14] { 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9 };
	while (result[3] != 0) {
		result = run_programm(instructions, instrs, num_in);

		bool print = false;
		for (int8_t i = 13; i >= 0; i--) {
			if (num_in[i] == 1) {
				if (i == 8) {
					print = true;
				}
				num_in[i] = 9;
			} else {
				num_in[i]--;
				if (print) {
					uint64_t k = 0;
					for (int8_t j = i; j >= 0; j--) {
						k += (9 - num_in[j]) * pow(10, i - j);
					}
					std::cout << "Finished " << (uint64_t) (pow(9, 13 - i) * k)
							<< " numbers." << std::endl;
				}
				break;
			}
		}
	}

	uint64_t part1 = 0;
	for (int i = 0; i < 14; i++) {
		part1 += num_in[i] * pow(10, 13 - i);
	}
	std::cout << "The biggest valid serial number is " << part1 << '.'
			<< std::endl;
}

std::ostream& operator <<(std::ostream &stream, const Instruction &inst) {
	switch (inst.type) {
	case InstType::INP:
		stream << "inp ";
		break;
	case InstType::ADD:
		stream << "add ";
		break;
	case InstType::MUL:
		stream << "mul ";
		break;
	case InstType::DIV:
		stream << "div ";
		break;
	case InstType::MOD:
		stream << "mod ";
		break;
	case InstType::EQL:
		stream << "eql ";
		break;
	case InstType::NUL:
		stream << "nul ";
		break;
	default:
		stream << inst.type << ' ';
	}

	stream << (uint8_t) ('w' + inst.reg_a);

	if (inst.type != InstType::INP) {
		stream << ' ';
		if (inst.const_b) {
			stream << inst.in_b;
		} else {
			stream << (int8_t) ('w' + inst.in_b);
		}
	}
	return stream;
}

std::array<int64_t, 4> run_programm(const Instruction *instv,
		const size_t instc, const uint8_t input[14]) {
	std::array<int64_t, 4> registers { 0, 0, 0, 0 };
	size_t in_i = 0;
	for (size_t i = 0; i < instc; i++) {
		const Instruction inst = instv[i];
		switch (inst.type) {
		case InstType::INP:
			registers[inst.reg_a] = input[in_i++];
			break;
		case InstType::ADD:
			registers[inst.reg_a] +=
					inst.const_b ? inst.in_b : registers[inst.in_b];
			break;
		case InstType::MUL:
			registers[inst.reg_a] *=
					inst.const_b ? inst.in_b : registers[inst.in_b];
			break;
		case InstType::DIV:
			registers[inst.reg_a] /=
					inst.const_b ? inst.in_b : registers[inst.in_b];
			break;
		case InstType::MOD:
			registers[inst.reg_a] %=
					inst.const_b ? inst.in_b : registers[inst.in_b];
			break;
		case InstType::EQL:
			registers[inst.reg_a] = registers[inst.reg_a]
					== (inst.const_b ? inst.in_b : registers[inst.in_b]);
			break;
		default:
			std::cerr << "Received unknown instruction " << inst.type << '.'
					<< std::endl;
		}
	}

	return registers;
}
