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
	std::vector<Instruction> instructions;

	std::string line;
	while (std::getline(input, line)) {
		InstType type = InstType::NOP;
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

		if (type == InstType::NOP) {
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

		instructions.push_back(Instruction(type, reg_a, const_b, in_b));
	}

	instructions = optimize(instructions);

	std::array<int64_t, 4> result { 0, 0, 0, 1 };
	uint8_t num_in[14] { 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9 };
	while (result[3] != 0) {
		result = run_programm(instructions, num_in);

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
						k += (9 - num_in[j]) * pow(9, i - j);
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
	case InstType::NOP:
		stream << "nop";
		break;
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
	case InstType::SUB:
		stream << "sub ";
		break;
	case InstType::EQL:
		stream << "eql ";
		break;
	case InstType::SET:
		stream << "set ";
		break;
	default:
		stream << inst.type << ' ';
	}

	if (inst.type != InstType::NOP) {
		stream << (uint8_t) ('w' + inst.reg_a);

		if (inst.type != InstType::INP) {
			stream << ' ';
			if (inst.const_b) {
				stream << inst.in_b;
			} else {
				stream << (int8_t) ('w' + inst.in_b);
			}
		}
	}
	return stream;
}

std::vector<Instruction> optimize(const std::vector<Instruction> insts) {
	std::vector<Instruction> result;

	// First replace instruction chains that don't use input with set instructions.
	// In this case dirty means interacted with input.
	bool reg_dirty[4] {false};
	bool last_dirty[4] {false};
	int64_t reg_state[4] { 0 };
	for (Instruction inst : insts) {
		if (inst.type == InstType::NOP) {
			// Remove all No-op instructions.
			continue;
		} else if (inst.type == InstType::MUL || inst.type == InstType::DIV) {
			if (inst.const_b && inst.in_b == 1) {
				// Remove multiply and divide instructions with one as a constant value.
				// Since they never have an effect.
				continue;
			} else if (!reg_dirty[inst.reg_a] && reg_state[inst.reg_a] == 0) {
				// Multiplying or dividing 0 by anything has no effect.
				continue;
			}
		} else if (inst.type == InstType::MOD && !reg_dirty[inst.reg_a] && reg_state[inst.reg_a] == 0) {
			// 0 modulo anything is always 0.
			continue;
		} else if (inst.type == InstType::ADD || inst.type == InstType::SUB) {
			if (inst.const_b && inst.in_b == 0) {
				// Remove add and subtract instructions with constant zero for the same reason.
				continue;
			}
		}

		if (inst.type == InstType::SET && inst.const_b) {
			// Setting a register to a const or a not dirty register cleans it.
			reg_dirty[inst.reg_a] = false;
			last_dirty[inst.reg_a] = true;
		} else if (inst.type == InstType::MUL) {
			// Multiplying a register by 0 cleans it.
			if (inst.const_b && inst.in_b == 0) {
				reg_dirty[inst.reg_a] = false;
				last_dirty[inst.reg_a] = true;
			} else if (!inst.const_b && !reg_dirty[inst.in_b] && reg_state[inst.in_b] == 0) {
				reg_dirty[inst.reg_a] = false;
				last_dirty[inst.reg_a] = true;
			}
		} else if (inst.type == InstType::MOD) {
			// X modulo 1 is always 0, undirtying the register.
			if (inst.const_b && inst.in_b == 1) {
				reg_dirty[inst.reg_a] = false;
				last_dirty[inst.reg_a] = true;
			} else if (!inst.const_b && !reg_dirty[inst.in_b] && reg_state[inst.in_b] == 1) {
				reg_dirty[inst.reg_a] = false;
				last_dirty[inst.reg_a] = true;
			}
		}

		if (reg_dirty[inst.reg_a]) {
			// Do not remove instructions using a dirty primary register.
			// Unless they clean it.
			result.push_back(inst);
			continue;
		}

		if (!inst.const_b && reg_dirty[inst.in_b]) {
			// Using a dirty secondary register dirties the primary one.
			if (reg_state[inst.reg_a] != 0 || last_dirty[inst.reg_a]) {
				if (inst.type != InstType::ADD || reg_state[inst.reg_a] != 0) {
					result.push_back(
							Instruction(SET, inst.reg_a, true,
									reg_state[inst.reg_a]));
				}
				last_dirty[inst.reg_a] = false;
			}

			reg_dirty[inst.reg_a] = true;
			if (inst.type == InstType::ADD && reg_state[inst.reg_a] == 0) {
				// If added to 0 replace set and add with single set.
				result.push_back(Instruction(SET, inst.reg_a, false, inst.in_b));
			} else {
				result.push_back(inst);
			}
			continue;
		}

		switch (inst.type) {
		case InstType::INP:
			reg_dirty[inst.reg_a] = true;
			result.push_back(inst);
			break;
		case InstType::ADD:
			reg_state[inst.reg_a] +=
					inst.const_b ? inst.in_b : reg_state[inst.in_b];
			break;
		case InstType::MUL:
			reg_state[inst.reg_a] *=
					inst.const_b ? inst.in_b : reg_state[inst.in_b];
			break;
		case InstType::DIV:
			reg_state[inst.reg_a] /=
					inst.const_b ? inst.in_b : reg_state[inst.in_b];
			break;
		case InstType::MOD:
			reg_state[inst.reg_a] %=
					inst.const_b ? inst.in_b : reg_state[inst.in_b];
			break;
		case InstType::SUB:
			reg_state[inst.reg_a] -=
					inst.const_b ? inst.in_b : reg_state[inst.in_b];
			break;
		case InstType::EQL:
			reg_state[inst.reg_a] = reg_state[inst.reg_a]
					== (inst.const_b ? inst.in_b : reg_state[inst.in_b]);
			break;
		case InstType::SET:
			reg_state[inst.reg_a] =
					inst.const_b ? inst.in_b : reg_state[inst.in_b];
			break;
		default:
			std::cerr << "Received unknown instruction " << inst.type << '.'
					<< std::endl;
		}
	}

	return result;
}

std::array<int64_t, 4> run_programm(const std::vector<Instruction> &insts, const uint8_t input[14]) {
	std::array<int64_t, 4> registers { 0, 0, 0, 0 };
	size_t in_i = 0;
	for (size_t i = 0; i < insts.size(); i++) {
		const Instruction inst = insts[i];
		switch (inst.type) {
		case InstType::NOP:
			break;
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
		case InstType::SUB:
			registers[inst.reg_a] -= inst.const_b ? inst.in_b : registers[inst.in_b];
			break;
		case InstType::EQL:
			registers[inst.reg_a] = registers[inst.reg_a]
					== (inst.const_b ? inst.in_b : registers[inst.in_b]);
			break;
		case InstType::SET:
			registers[inst.reg_a] = inst.const_b ? inst.in_b : registers[inst.in_b];
			break;
		default:
			std::cerr << "Received unknown instruction " << inst.type << '.'
					<< std::endl;
		}
	}

	return registers;
}
