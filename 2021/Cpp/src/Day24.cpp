/*
 * Day24.cpp
 *
 *  Created on: 19.11.2022
 *      Author: ToMe25
 */

#include "Day24.h"
#include <cmath>
#include <thread>

enum ExecMode {
	/**
	 * Interprets the given input assembly.
	 */
	INTERPRETE,
	/**
	 * Generates C code from the input assembly, compiles that, and runs it.
	 */
	COMPILE
};

enum RunType {
	/**
	 * Tests the input with all 14 digit numbers without zeros to find the highest and lowest match.
	 */
	SOLVE,
	/**
	 * Executes the input with the specified input digits and prints all register states.
	 */
	EXECUTE
};

/**
 * Whether the input assembly should be interpreted or compiled.
 */
const ExecMode EXEC_MODE = COMPILE;

/**
 * Whether the input assembly should be optimized.
 */
const bool OPTIMIZE = true;

/**
 * Whether the input program should be executed or solved.
 * Run type execute is not yet compatible with compiled execution.
 */
const RunType RUN_TYPE = SOLVE;

/**
 * The digits to use as input for the program.
 */
const char INPUT_DIGITS[] = { 1, 2 };

template<>
void DayRunner<24>::solve(std::ifstream input) {
	std::vector<Instruction> instructions;

	std::string line;
	while (std::getline(input, line)) {
		InstType type = InstType::NOP;
		bool err = false;
		switch (line[0]) {
		case 'n':
			if (line[1] == 'o') {
				type = InstType::NOP;
			} else if (line[1] == 'e') {
				type = InstType::NEQ;
			} else {
				err = true;
			}
			break;
		case 'i':
			type = InstType::INP;
			break;
		case 'a':
			type = InstType::ADD;
			break;
		case 's':
			if (line[1] == 'u') {
				type = InstType::SUB;
			} else if (line[1] == 'e') {
				type = InstType::SET;
			} else {
				err = true;
			}
			break;
		case 'm':
			if (line[1] == 'u') {
				type = InstType::MUL;
			} else if (line[1] == 'o') {
				type = InstType::MOD;
			} else {
				err = true;
			}
			break;
		case 'd':
			type = InstType::DIV;
			break;
		case 'e':
			type = InstType::EQL;
			break;
		default:
			err = true;
		}

		if (err) {
			std::cerr << "Line \"" << line
					<< "\" contained an invalid instruction type." << std::endl;
			continue;
		}

		uint8_t reg_a = 0;
		if (type != InstType::NOP) {
			reg_a = line[4] - 'w';
		}

		int32_t in_b = 0;
		bool const_b = true;
		if (type != InstType::NOP && type != InstType::INP) {
			if (line[6] <= 'z' && line[6] >= 'w') {
				const_b = false;
				in_b = line[6] - 'w';
			} else {
				in_b = std::stoi(line.substr(6));
			}
		}

		instructions.push_back(Instruction(type, reg_a, const_b, in_b));
	}

	if (RUN_TYPE == EXECUTE && EXEC_MODE == COMPILE) {
		std::cerr << "Run type EXECUTE does not yet support compiled execution."
				<< std::endl;
		return;
	}

	if (OPTIMIZE) {
		instructions = static_eval(instructions);
		instructions = dead_code_removal(instructions);
		instructions = merge_duplicate(instructions);
		instructions = delay_input(instructions);
	}

	if (RUN_TYPE == SOLVE
			&& std::count_if(instructions.begin(), instructions.end(),
					[](const Instruction &inst) {
						return inst.type == InstType::INP;
					}) != 14) {
		std::cerr << "The input did not contain 14 input instructions."
				<< std::endl;
		return;
	}

	std::filesystem::path tmpDir("tmp");
	std::optional<std::filesystem::path> tmpLib = std::nullopt;
	if (RUN_TYPE == SOLVE) {
		if (EXEC_MODE == COMPILE) {
			tmpLib = create_temp_lib(instructions, tmpDir);
			if (!tmpLib.has_value()) {
				return;
			}
		}

		int64_t part1 = -1;
		if (EXEC_MODE == COMPILE) {
			part1 = find_first_valid_compiled(tmpLib.value(), true);
		} else if (EXEC_MODE == INTERPRETE) {
			part1 = find_first_valid_interpreted(instructions, true);
		}
		std::cout << "The biggest valid serial number is " << part1 << '.'
				<< std::endl;

		int64_t part2 = -1;
		if (EXEC_MODE == COMPILE) {
			part2 = find_first_valid_compiled(tmpLib.value(), false);
		} else if (EXEC_MODE == INTERPRETE) {
			part2 = find_first_valid_interpreted(instructions, false);
		}
		std::cout << "The biggest valid serial number is " << part1 << '.'
				<< std::endl;
		std::cout << "The smallest valid serial number is " << part2 << '.'
				<< std::endl;
	} else if (RUN_TYPE == EXECUTE) {
		const long long int initial_regs[4] { 0 };
		long long int registers[4];
		run_program(instructions.data(), instructions.size(), initial_regs,
				registers, INPUT_DIGITS, sizeof(INPUT_DIGITS) / sizeof(char));
		std::cout << "The register values after running the program are w="
				<< registers[0] << ", x=" << registers[1] << ", y="
				<< registers[2] << ", z=" << registers[3] << '.' << std::endl;
	}

	if (EXEC_MODE == COMPILE) {
		delete_temp(tmpDir, tmpLib.value());
	}
}

std::ostream& operator <<(std::ostream &stream, const Instruction &inst) {
	stream << inst.type;
	if (inst.type != InstType::NOP) {
		stream << ' ' << (uint8_t) ('w' + inst.reg_a);

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

std::ostream& operator <<(std::ostream &stream, const InstType &type) {
	if (inst_type_names.count(type) != 0) {
		stream << inst_type_names.at(type);
	} else {
		stream << "unknown instruction type " << (int) type;
	}

	return stream;
}

std::vector<Instruction> static_eval(const std::vector<Instruction> &insts) {
	std::vector<Instruction> result;

	// Replace instruction chains that don't use input with set instructions.
	// In this case dirty means interacted with input.
	bool reg_dirty[4] { false };
	bool last_dirty[4] { false };
	long long int reg_state[4] { 0 };
	for (Instruction inst : insts) {
		if (inst.type == InstType::NOP) {
			// Remove all No-op instructions.
			continue;
		} else if (inst.type == InstType::MUL || inst.type == InstType::DIV) {
			if (inst.const_b && inst.in_b == 1) {
				// Remove multiply and divide instructions with one as a constant value.
				// Since they never have an effect.
				continue;
			} else if (!inst.const_b && !reg_dirty[inst.in_b]
					&& reg_state[inst.in_b] == 1) {
				continue;
			} else if (!reg_dirty[inst.reg_a] && reg_state[inst.reg_a] == 0) {
				// Multiplying or dividing 0 by anything has no effect.
				continue;
			}
		} else if (inst.type == InstType::MOD && !reg_dirty[inst.reg_a]
				&& reg_state[inst.reg_a] == 0) {
			// 0 modulo anything is always 0.
			continue;
		} else if (inst.type == InstType::ADD || inst.type == InstType::SUB) {
			if (inst.const_b && inst.in_b == 0) {
				// Remove add and subtract instructions with constant zero for the same reason.
				continue;
			} else if (!inst.const_b && !reg_dirty[inst.in_b]
					&& reg_state[inst.in_b] == 0) {
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
			} else if (!inst.const_b && !reg_dirty[inst.in_b]
					&& reg_state[inst.in_b] == 0) {
				reg_dirty[inst.reg_a] = false;
				last_dirty[inst.reg_a] = true;
			}
		} else if (inst.type == InstType::MOD) {
			// X modulo 1 is always 0, undirtying the register.
			if (inst.const_b && (inst.in_b == 1 || inst.in_b == -1)) {
				reg_dirty[inst.reg_a] = false;
				last_dirty[inst.reg_a] = true;
			} else if (!inst.const_b && !reg_dirty[inst.in_b]
					&& (reg_state[inst.in_b] == 1 || reg_state[inst.in_b] == -1)) {
				reg_dirty[inst.reg_a] = false;
				last_dirty[inst.reg_a] = true;
			}
		}

		if (reg_dirty[inst.reg_a]) {
			// Do not remove instructions using a dirty primary register.
			// Unless they clean it.
			if (inst.const_b || reg_dirty[inst.in_b]) {
				result.push_back(inst);
			} else {
				// If the secondary input is not dirty and not constant, its register will not be set.
				result.push_back(
						Instruction(inst.type, inst.reg_a, true,
								reg_state[inst.in_b]));
			}
			continue;
		}

		if (!inst.const_b && reg_dirty[inst.in_b]) {
			// Using a dirty secondary register dirties the primary one.
			if (reg_state[inst.reg_a] != 0 || last_dirty[inst.reg_a]) {
				if (inst.type != InstType::ADD || reg_state[inst.reg_a] != 0) {
					result.push_back(
							Instruction(InstType::SET, inst.reg_a, true,
									reg_state[inst.reg_a]));
				}
				last_dirty[inst.reg_a] = false;
			}

			reg_dirty[inst.reg_a] = true;
			if (inst.type == InstType::ADD && reg_state[inst.reg_a] == 0) {
				// If added to 0 replace set and add with single set.
				result.push_back(
						Instruction(InstType::SET, inst.reg_a, false,
								inst.in_b));
			} else {
				result.push_back(inst);
			}
			continue;
		}

		// Input instructions dirty their register without requiring its previous value.
		if (inst.type == InstType::INP) {
			reg_dirty[inst.reg_a] = true;
			result.push_back(inst);
			continue;
		}

		// Evaluate instructions when both their inputs are static.
		run_program(&inst, 1, reg_state, reg_state, NULL, 0);
	}

	// Set unused registers. Not setting them is dcrs job.
	for (uint8_t i = 0; i < 4; i++) {
		if (!reg_dirty[i] && (last_dirty[i] || reg_state[i] != 0)) {
			result.push_back(Instruction(InstType::SET, i, true, reg_state[i]));
		}
	}

	return result;
}

std::vector<Instruction> dead_code_removal(
		const std::vector<Instruction> &insts) {
	bool used[insts.size()] { false };
	bool reg_used[4] { false, false, false, true };
	if (RUN_TYPE == EXECUTE) {
		reg_used[0] = reg_used[1] = reg_used[2] = true;
	}

	for (size_t i = insts.size(); i > 0;) {
		i--;
		const Instruction inst = insts[i];
		used[i] = reg_used[inst.reg_a];
		if (used[i] && !inst.const_b) {
			reg_used[inst.in_b] = true;
		}

		// If something overrides a register its previous value isn't used.
		if (inst.type == InstType::INP) {
			reg_used[inst.reg_a] = false;
		} else if (inst.type == InstType::SET) {
			reg_used[inst.reg_a] = false;
		} else if (inst.type == InstType::MUL && inst.const_b
				&& inst.in_b == 0) {
			reg_used[inst.reg_a] = false;
		} else if (inst.type == InstType::MOD && inst.const_b
				&& (inst.in_b == 1 || inst.in_b == -1)) {
			reg_used[inst.reg_a] = false;
		}
	}

	std::vector<Instruction> result;
	for (size_t i = 0; i < insts.size(); i++) {
		if (used[i]) {
			result.push_back(insts[i]);
		}
	}

	return result;
}

std::vector<Instruction> merge_duplicate(
		const std::vector<Instruction> &insts) {
	std::vector<Instruction> result;
	int64_t last_mergable[4] { -1, -1, -1, -1 };
	for (size_t i = 0; i < insts.size(); i++) {
		const Instruction inst = insts[i];

		// Non constants can't be merged, except for the eql to neq merge.
		if (!inst.const_b && inst.type != InstType::EQL) {
			last_mergable[inst.reg_a] = -1;
			last_mergable[inst.in_b] = -1;
			result.push_back(inst);
			continue;
		}

		if (last_mergable[inst.reg_a] == -1) {
			if (inst.type == InstType::ADD || inst.type == InstType::SUB
					|| inst.type == InstType::MUL || inst.type == InstType::DIV
					|| inst.type == InstType::EQL) {
				last_mergable[inst.reg_a] = result.size();
			} else {
				last_mergable[inst.reg_a] = -1;
			}
			result.push_back(inst);
			continue;
		}

		const InstType lastType = result[last_mergable[inst.reg_a]].type;
		if (inst.type == lastType
				|| (inst.type == InstType::ADD && lastType == InstType::SUB)
				|| (inst.type == InstType::SUB && lastType == InstType::ADD)) {
			switch (lastType) {
			case InstType::ADD:
			case InstType::SUB:
				if (inst.type == lastType) {
					result[last_mergable[inst.reg_a]].in_b += inst.in_b;
				} else {
					int32_t sum = result[last_mergable[inst.reg_a]].in_b;
					if (lastType == InstType::SUB) {
						sum *= -1;
					}
					sum += inst.type == InstType::ADD ? inst.in_b : -inst.in_b;

					if (sum < 0) {
						result[last_mergable[inst.reg_a]].type = InstType::SUB;
						result[last_mergable[inst.reg_a]].in_b = -sum;
					} else {
						result[last_mergable[inst.reg_a]].type = InstType::ADD;
						result[last_mergable[inst.reg_a]].in_b = sum;
					}
				}
				break;
			case InstType::MUL:
			case InstType::DIV:
				result[last_mergable[inst.reg_a]].in_b *= inst.in_b;
				break;
			case InstType::EQL:
				if (inst.const_b && inst.in_b == 0) {
					result[last_mergable[inst.reg_a]].type = InstType::NEQ;
					last_mergable[inst.reg_a] = -1;
				} else {
					last_mergable[inst.reg_a] = result.size();
					result.push_back(inst);
				}
				break;
			}
		} else {
			last_mergable[inst.reg_a] = result.size();
			result.push_back(inst);
		}
	}

	return result;
}

std::vector<Instruction> delay_input(const std::vector<Instruction> &insts) {
	std::vector<Instruction> result;

	int64_t reg_inp[4] { -1, -1, -1, -1 };
	for (const Instruction inst : insts) {
		if (inst.type == InstType::INP) {
			reg_inp[inst.reg_a] = result.size();
		} else if (inst.type != InstType::NOP) {
			if (!inst.const_b && reg_inp[inst.in_b] >= 0
					&& reg_inp[inst.in_b] < reg_inp[inst.reg_a]) {
				result.push_back(
						Instruction(InstType::INP, inst.in_b, false, 0));
				reg_inp[inst.in_b] = -1;
			}

			if (reg_inp[inst.reg_a] >= 0) {
				result.push_back(
						Instruction(InstType::INP, inst.reg_a, false, 0));
				reg_inp[inst.reg_a] = -1;
			}

			if (!inst.const_b && reg_inp[inst.in_b] >= 0) {
				result.push_back(
						Instruction(InstType::INP, inst.in_b, false, 0));
				reg_inp[inst.in_b] = -1;
			}
			result.push_back(inst);
		}
	}

	return result;
}

void run_program(const Instruction instsv[], const size_t instsc,
		const long long int reg_vals[4], long long int *reg, const char inpv[],
		const size_t inpc) {
	reg[0] = reg_vals[0];
	reg[1] = reg_vals[1];
	reg[2] = reg_vals[2];
	reg[3] = reg_vals[3];

	size_t inp = 0;
	for (size_t i = 0; i < instsc; i++) {
		const Instruction inst = instsv[i];
		const int64_t in_b = inst.const_b ? inst.in_b : reg[inst.in_b];
		switch (inst.type) {
		case InstType::NOP:
			break;
		case InstType::INP:
			if (inp >= inpc) {
				std::cerr << "Trying to read input digit " << (inp + 1)
						<< " when only " << inpc << " were given." << std::endl;
				return;
			}
			reg[inst.reg_a] = inpv[inp++];
			break;
		case InstType::ADD:
			reg[inst.reg_a] += in_b;
			break;
		case InstType::SUB:
			reg[inst.reg_a] -= in_b;
			break;
		case InstType::MUL:
			reg[inst.reg_a] *= in_b;
			break;
		case InstType::DIV:
			reg[inst.reg_a] /= in_b;
			break;
		case InstType::MOD:
			reg[inst.reg_a] %= in_b;
			break;
		case InstType::EQL:
			reg[inst.reg_a] = reg[inst.reg_a] == in_b;
			break;
		case InstType::NEQ:
			reg[inst.reg_a] = reg[inst.reg_a] != in_b;
			break;
		case InstType::SET:
			reg[inst.reg_a] = in_b;
			break;
		default:
			std::cerr << "Received unknown instruction " << inst.type << '.'
					<< std::endl;
		}
	}
}

void find_first_runner_interpreted(const dynfunc funcs[14],
		const std::array<uint8_t, 3> digits, std::atomic<int64_t> *result,
		std::atomic<bool> *stop, const bool highest) {
	const uint16_t id = digits[0] * 100 + digits[1] * 10 + digits[2];
	std::array<uint8_t, 14> num_in { digits[0], digits[1], digits[2], 9, 9, 9,
			9, 9, 9, 9, 9, 9, 9, 9 };
	if (!highest) {
		num_in = { digits[0], digits[1], digits[2], 1, 1, 1, 1, 1, 1, 1, 1, 1,
				1, 1 };
	}

	long long int registers[14][4];
	const long long int start_reg[4] { 0 };
	funcs[0](start_reg, registers[0], (char) num_in[0]);
	for (uint8_t i = 1; i < 14; i++) {
		funcs[i](registers[i - 1], registers[i], (char) num_in[i]);
	}

	bool finished = false;
	while (registers[13][3] != 0 && !finished && !*stop) {
		bool print = false;
		for (uint8_t i = 13; i >= 2; i--) {
			if (i == 2) {
				// Would change the static third digit.
				finished = true;
				break;
			}

			if (highest && num_in[i] == 1) {
				if (i == 5) {
					print = true;
				}
				num_in[i] = 9;
			} else if (!highest && num_in[i] == 9) {
				if (i == 5) {
					print = true;
				}
				num_in[i] = 1;
			} else {
				if (highest) {
					num_in[i]--;
				} else {
					num_in[i]++;
				}

				funcs[i](registers[i - 1], registers[i], (char) num_in[i]);

				for (uint8_t j = i; j < 14; j++) {
					funcs[j](registers[j - 1], registers[j], (char) num_in[j]);
				}

				if (print) {
					uint64_t k = 0;
					for (int8_t j = i; j > 2; j--) {
						if (highest) {
							k += (9 - num_in[j]) * pow(9, i - j);
						} else {
							k += (num_in[j] - 1) * pow(9, i - j);
						}
					}
					std::cout << "Thread " << id << " checked "
							<< (uint64_t) (pow(9, 13 - i) * k) << " numbers."
							<< std::endl;
				}
				break;
			}
		}
	}

	if (!finished) {
		if (*stop) {
			*result = -1;
		} else {
			*result = 0;
			for (int i = 0; i < 14; i++) {
				*result += num_in[i] * pow(10, 13 - i);
			}
			std::cout << "Thread " << id << " found number " << *result << '.'
					<< std::endl;
		}
	} else {
		*result = -1;
		std::cout << "Thread " << id << " finished checking all its numbers."
				<< std::endl;
	}
}

void find_first_runner_compiled(const std::filesystem::path exe,
		const std::array<uint8_t, 3> digits, std::atomic<int64_t> *result,
		const bool highest) {
	std::ostringstream cmd;
	cmd << exe << ' ' << (uint16_t) digits[0] << ' ' << (uint16_t) digits[1]
			<< ' ' << (uint16_t) digits[2] << ' '
			<< (highest ? "true" : "false");
	const int status = std::system(cmd.str().c_str());
	if (status != 0) {
		std::cout << "Worker " << (uint16_t) digits[0] << (uint16_t) digits[1]
				<< (uint16_t) digits[2] << " failed with exit code " << status
				<< '.' << std::endl;
		*result = -1;
		return;
	}

	std::filesystem::path tmpF = exe;
	tmpF += digits[0] + '0';
	tmpF += digits[1] + '0';
	tmpF += digits[2] + '0';
	std::ifstream tmpFI(tmpF);
	long long int res;
	tmpFI >> res;
	*result = res;
	tmpFI.close();
	std::filesystem::remove(tmpF);
}

int64_t find_first_valid_interpreted(const std::vector<Instruction> insts,
		const bool highest) {
	size_t inps[15];
	uint8_t in_i = 0;
	for (size_t i = 0; i < insts.size(); i++) {
		if (insts[i].type == InstType::INP) {
			inps[in_i++] = i;
		}
	}
	inps[in_i] = insts.size();

	if (in_i != 14) {
		std::cerr << "Input does not have 14 input instructions." << std::endl;
		return -1;
	}

	dynfunc funcs[14];
	for (size_t i = 0; i < in_i; i++) {
		funcs[i] = [insts, inps, i](const long long int reg_vals[4],
				long long int *reg, const char inp) {
			run_program(&insts.data()[inps[i]], inps[i + 1] - inps[i], reg_vals,
					reg, &inp, 1);
		};
	}

	int64_t result = 0;
	std::atomic<bool> stop = false;
	std::atomic<int64_t> results[18] { 0 };
	std::thread threads[18];
	for (uint8_t i = highest ? 81 : 1; highest ? (i > 0) : (i <= 81);
			highest ? i-- : i++) {
		for (uint8_t j = 9; j > 0; j--) {
			const std::array<uint8_t, 3> digits { (uint8_t) ((i - 1) / 9 + 1),
					(uint8_t) ((i - 1) % 9 + 1), j };

			threads[(i % 2) * 9 + j - 1] = std::thread(
					find_first_runner_interpreted, funcs, digits,
					&results[(i % 2) * 9 + j - 1], &stop, highest);
		}

		if (highest && i == 81) {
			continue;
		} else if (!highest && i == 1) {
			continue;
		}

		bool second = (i % 2) == 0;
		for (int8_t j = (highest ? 8 : 0); highest ? (j >= 0) : (j < 9);
				highest ? j-- : j++) {
			if (threads[j + second * 9].joinable()) {
				threads[j + second * 9].join();
			}

			if (result == 0 && results[j + second * 9] != -1) {
				result = results[j + second * 9];
				stop = true;
			}
		}

		if (result != 0) {
			break;
		}
	}

	for (int8_t i = highest ? 17 : 0; highest ? (i >= 0) : (i < 18);
			highest ? i-- : i++) {
		if (threads[i].joinable()) {
			threads[i].join();
		}

		if (result == 0 && results[i] != -1) {
			result = results[i];
			stop = true;
		}
	}

	return result == 0 ? -1 : result;
}

int64_t find_first_valid_compiled(const std::filesystem::path exe,
		const bool highest) {
	int64_t result = 0;
	std::atomic<int64_t> results[18] { 0 };
	std::thread threads[18];
	for (uint8_t i = highest ? 81 : 1; highest ? (i > 0) : (i <= 81);
			highest ? i-- : i++) {
		if (result == 0) {
			for (uint8_t j = 9; j > 0; j--) {
				const std::array<uint8_t, 3> digits {
						(uint8_t) ((i - 1) / 9 + 1),
						(uint8_t) ((i - 1) % 9 + 1), j };

				// Use value -2 to mark that the thread is still running.
				results[(i % 2) * 9 + j - 1] = -2;
				threads[(i % 2) * 9 + j - 1] = std::thread(
						find_first_runner_compiled, exe, digits,
						&results[(i % 2) * 9 + j - 1], highest);
			}
		}

		if (highest && i == 81) {
			continue;
		} else if (!highest && i == 1) {
			continue;
		}

		bool second = (i % 2) == 0;
		for (int8_t j = (highest ? 8 : 0); highest ? (j >= 0) : (j < 9);
				highest ? j-- : j++) {
			if (threads[j + second * 9].joinable()) {
				threads[j + second * 9].join();
			}

			if (result == 0 && results[j + second * 9] != -1) {
				result = results[j + second * 9];
			}
		}

		for (int8_t j = (highest ? 8 : 0); highest ? (j >= 0) : (j < 9);
				highest ? j-- : j++) {
			// The thread is still running, we can't get its result.
			if (results[j + (!second) * 9] == -2) {
				break;
			}

			if (result == 0 && results[j + (!second) * 9] != -1) {
				result = results[j + (!second) * 9];
				break;
			}
		}

		if (result != 0) {
			break;
		}
	}

	for (int8_t i = highest ? 17 : 0; highest ? (i >= 0) : (i < 18);
			highest ? i-- : i++) {
		if (threads[i].joinable()) {
			threads[i].join();
		}

		if (result == 0 && results[i] != -1) {
			result = results[i];
		}
	}

	return result == 0 ? -1 : result;
}

bool compile_instructions(const std::vector<Instruction> insts,
		const std::filesystem::path tmpDir) {
	std::filesystem::path tmpC(tmpDir);
	tmpC += std::filesystem::path::preferred_separator;
	tmpC += "tmp.c";

	if (std::filesystem::exists(tmpC)
			&& !std::filesystem::is_regular_file(tmpC)) {
		std::cerr << tmpC << " exists but isn't a file." << std::endl;
		return false;
	} else if (std::filesystem::exists(tmpC)
			&& !std::filesystem::remove(tmpC)) {
		std::cerr << "Failed to remove " << tmpC << '.' << std::endl;
		return false;
	}

	std::ofstream tmpCO(tmpC);
	tmpCO << "#include <stdio.h>" << std::endl;
	tmpCO << "#include <stdlib.h>" << std::endl;
	tmpCO << "#include <string.h>" << std::endl << std::endl;
	tmpCO << "int main(const int argc, char *argv[]) {" << std::endl;
	tmpCO << "    if (argc < 5) {" << std::endl;
	tmpCO
			<< "        printf(\"Error: This program requires three input digits and a boolean to operate.\\n\");"
			<< std::endl;
	tmpCO << "        printf(\"Format: DIGIT_1 DIGIT_2 DIGIT_3 REVERSE\\n\");"
			<< std::endl;
	tmpCO << "        return 1;" << std::endl;
	tmpCO << "    }" << std::endl << std::endl;
	tmpCO << "    unsigned char const_inputs[3];" << std::endl;
	tmpCO << "    for (char i = 0; i < 3; i++) {" << std::endl;
	tmpCO
			<< "        if (argv[i + 1][0] < '0' || argv[i + 1][0] > '9' || strlen(argv[i + 1]) != 1) {"
			<< std::endl;
	tmpCO
			<< "            printf(\"DIGIT_%d wasn't a digit. Was %s.\\n\", i, argv[i + 1]);"
			<< std::endl;
	tmpCO << "            return 1;" << std::endl;
	tmpCO << "        }" << std::endl;
	tmpCO << "        const_inputs[i] = argv[i + 1][0] - '0';" << std::endl;
	tmpCO << "    }" << std::endl << std::endl;
	tmpCO << "    for (long long int i = 0; i < strlen(argv[4]); i++) {"
			<< std::endl;
	tmpCO << "        if (argv[4][i] >= 'A' && argv[4][i] <= 'Z') {"
			<< std::endl;
	tmpCO << "            argv[4][i] += 32;" << std::endl;
	tmpCO << "        }" << std::endl;
	tmpCO << "    }" << std::endl << std::endl;
	tmpCO
			<< "    if (strcmp(argv[4], \"true\") != 0 && strcmp(argv[4], \"false\") != 0) {"
			<< std::endl;
	tmpCO
			<< "        printf(\"REVERSE wasn't a valid boolean. Was \\\"%s\\\".\\n\", argv[4]);"
			<< std::endl;
	tmpCO << "        return 1;" << std::endl;
	tmpCO << "    }" << std::endl;
	tmpCO << "    const char reverse = strcmp(argv[4], \"true\") == 0;"
			<< std::endl << std::endl;
	tmpCO << "    char *ofp = (char*) malloc(strlen(argv[0]) + 4);"
			<< std::endl;
	tmpCO << "    strcpy(ofp, argv[0]);" << std::endl;
	tmpCO << "    ofp[strlen(argv[0])] = const_inputs[0] + '0';" << std::endl;
	tmpCO << "    ofp[strlen(argv[0]) + 1] = const_inputs[1] + '0';"
			<< std::endl;
	tmpCO << "    ofp[strlen(argv[0]) + 2] = const_inputs[2] + '0';"
			<< std::endl;
	tmpCO << "    ofp[strlen(argv[0]) + 3] = 0;" << std::endl;
	tmpCO << "    FILE *of = fopen(ofp, \"w\");" << std::endl;
	tmpCO << "    if (!of) {" << std::endl;
	tmpCO
			<< "        printf(\"Failed to open output file \\\"%s\\\".\\n\", ofp);"
			<< std::endl;
	tmpCO << "        return 2;" << std::endl;
	tmpCO << "    }" << std::endl;
	tmpCO << "    free(ofp);" << std::endl << std::endl;
	tmpCO << "    long long int reg_h[4];" << std::endl << std::endl;

	const char loop_start[] =
			"for (char I = reverse ? 9 : 1; reverse ? (I > 0) : (I < 10); reverse ? I-- : I++) {";

	int16_t method_idx = 0;
	for (size_t i = 0; i < insts.size(); i++) {
		const Instruction inst = insts[i];

		if (inst.type == InstType::INP) {
			method_idx++;
		}

		if (method_idx > 3 && inst.type == InstType::INP) {
			for (size_t i = 0; i < std::max(1, method_idx - 3); i++) {
				tmpCO << "    ";
			}
			std::string loop(loop_start);
			std::replace(loop.begin(), loop.end(), 'I',
					(char) ('i' + method_idx - 4));
			tmpCO << loop << std::endl;

			for (size_t i = 0; i < std::max(1, method_idx - 2); i++) {
				tmpCO << "    ";
			}
			tmpCO << "long long int reg_"
					<< std::max('h', (char) ('i' + method_idx - 4)) << "[4];"
					<< std::endl;

			bool regs_used[4] { false };
			bool regs_checked[4] { false };
			for (size_t j = i; j < insts.size(); j++) {
				if (!regs_checked[insts[j].reg_a]) {
					if (insts[j].type == InstType::INP
							|| insts[j].type == InstType::SET) {
						regs_used[insts[j].reg_a] = false;
						regs_checked[insts[j].reg_a] = true;
					} else if (insts[j].type != InstType::NOP) {
						regs_used[insts[j].reg_a] = true;
						regs_checked[insts[j].reg_a] = true;
					}
				}

				if (insts[j].type != InstType::NOP
						&& insts[j].type != InstType::INP && !insts[j].const_b
						&& !regs_checked[insts[j].in_b]) {
					regs_used[insts[j].in_b] = true;
					regs_checked[insts[j].in_b] = true;
				}

				if (regs_checked[0] && regs_checked[1] && regs_checked[2]
						&& regs_checked[3]) {
					break;
				}
			}

			for (uint16_t j = 0; j < 4; j++) {
				if (regs_used[j]) {
					for (size_t i = 0; i < std::max(1, method_idx - 2); i++) {
						tmpCO << "    ";
					}
					tmpCO << "reg_"
							<< std::max('h', (char) ('i' + method_idx - 4))
							<< '[' << j;
					tmpCO << "] = reg_"
							<< std::max('h', (char) ('i' + method_idx - 5))
							<< '[' << j << "];" << std::endl;
				}
			}
		}

		const char reg_idx = std::max('h', (char) ('i' + method_idx - 4));
		if (inst.type != InstType::NOP) {
			for (size_t i = 0; i < std::max(1, method_idx - 2); i++) {
				tmpCO << "    ";
			}
			tmpCO << "reg_" << reg_idx << '[' << (uint16_t) inst.reg_a << ']';
		}

		switch (inst.type) {
		case InstType::NOP:
			break;
		case InstType::INP:
			if (method_idx < 4) {
				tmpCO << " = const_inputs[" << method_idx - 1 << ']';
			} else {
				tmpCO << " = " << (char) ('i' + method_idx - 4);
			}
			break;
		case InstType::ADD:
			tmpCO << " += ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		case InstType::SUB:
			tmpCO << " -= ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		case InstType::MUL:
			tmpCO << " *= ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		case InstType::DIV:
			tmpCO << " /= ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		case InstType::MOD:
			tmpCO << " %= ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		case InstType::EQL:
			tmpCO << " = reg_" << reg_idx << '[' << (uint16_t) inst.reg_a
					<< "] == ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		case InstType::NEQ:
			tmpCO << " = reg_" << reg_idx << '[' << (uint16_t) inst.reg_a
					<< "] != ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		case InstType::SET:
			tmpCO << " = ";
			if (inst.const_b) {
				tmpCO << inst.in_b;
			} else {
				tmpCO << "reg_" << reg_idx << '[' << inst.in_b << ']';
			}
			break;
		}

		if (inst.type != InstType::NOP) {
			tmpCO << ';' << std::endl;
		}
	}

	tmpCO << std::endl;
	tmpCO
			<< "                                                if (reg_s[3] == 0) {"
			<< std::endl;
	tmpCO << "                                                    "
			<< "printf(\"Worker %d%d%d found valid number: %d%d%d%d%d%d%d%d%d%d%d%d%d%d\\n\", "
			<< "const_inputs[0], const_inputs[1], const_inputs[2], const_inputs[0], const_inputs[1], const_inputs[2], i, j, k, l, m, n, o, p, q, r, s);"
			<< std::endl;
	tmpCO << "                                                    "
			<< "fprintf(of, \"%d%d%d%d%d%d%d%d%d%d%d%d%d%d\", "
			<< "const_inputs[0], const_inputs[1], const_inputs[2], i, j, k, l, m, n, o, p, q, r, s);"
			<< std::endl;
	tmpCO << "                                                    return 0;"
			<< std::endl;
	for (uint8_t i = 12; i > 0; i--) {
		for (uint8_t j = 0; j < i; j++) {
			tmpCO << "    ";
		}
		tmpCO << '}' << std::endl;
	}
	tmpCO
			<< "    printf(\"Worker %d%d%d couldn't find a valid number.\\n\", const_inputs[0], const_inputs[1], const_inputs[2]);"
			<< std::endl;
	tmpCO << "    fprintf(of, \"-1\");" << std::endl;
	tmpCO << "    fclose(of);" << std::endl;
	tmpCO << '}' << std::endl;
	tmpCO.close();

	std::filesystem::path tmpM(tmpDir);
	tmpM += std::filesystem::path::preferred_separator;
	tmpM += "tmp.Makefile";

	if (std::filesystem::exists(tmpM)
			&& !std::filesystem::is_regular_file(tmpM)) {
		std::cerr << tmpM << " exists but isn't a file." << std::endl;
		return false;
	} else if (std::filesystem::exists(tmpM)
			&& !std::filesystem::remove(tmpM)) {
		std::cerr << "Failed to remove " << tmpM << '.' << std::endl;
		return false;
	}

	std::ofstream tmpMO(tmpM);
	tmpMO << "PROJECT_ROOT = $(dir $(abspath $(lastword $(MAKEFILE_LIST))))"
			<< std::endl;
	tmpMO << "OBJS := $(PROJECT_ROOT)tmp.o" << std::endl;
	tmpMO << "CFLAGS += -O3" << std::endl;
	tmpMO << "$(PROJECT_ROOT)tmp: $(OBJS)" << std::endl;
	tmpMO << "\t$(CC) $(LDFLAGS) -o $@ $^" << std::endl;
	tmpMO << "$(PROJECT_ROOT)%.o: $(PROJECT_ROOT)%.c" << std::endl;
	tmpMO << "\t$(CC) -c $(CFLAGS) -o $@ $<" << std::endl;
	tmpMO.close();

	std::string cmd = std::string("make -f ").append(tmpM);
	std::cout << "Running \"" << cmd << "\"." << std::endl;
	int status = std::system(cmd.c_str());
	if (status != 0) {
		std::cerr << "Make command failed with status code " << status << '.'
				<< std::endl;
		return false;
	} else {
		std::cout << "Make finished." << std::endl;
	}

	return true;
}

std::optional<std::filesystem::path> create_temp_lib(
		const std::vector<Instruction> instructions,
		const std::filesystem::path tmpDir) {
	if (std::filesystem::exists(tmpDir)
			&& !std::filesystem::is_directory(tmpDir)) {
		std::cerr << std::filesystem::canonical(tmpDir)
				<< " exists but isn't a directory." << std::endl;
		return std::nullopt;
	} else if (!std::filesystem::exists(tmpDir)
			&& !std::filesystem::create_directories(tmpDir)) {
		std::cerr << tmpDir << " does not exist and can't be created."
				<< std::endl;
		return std::nullopt;
	}

	if (!compile_instructions(instructions, tmpDir)) {
		return std::nullopt;
	}

	std::filesystem::path tmpLib(tmpDir);
	tmpLib += std::filesystem::path::preferred_separator;
	tmpLib += "tmp";
	if (!std::filesystem::exists(tmpLib)) {
		std::cerr << "Make didn't create library " << tmpLib << '.'
				<< std::endl;
		return std::nullopt;
	}

	return std::optional<std::filesystem::path> { tmpLib };
}

bool delete_temp(const std::filesystem::path tmpDir,
		const std::filesystem::path tmpLib) {
	std::filesystem::remove(tmpLib);
	std::filesystem::path tmpO(tmpDir);
	tmpO += std::filesystem::path::preferred_separator;
	tmpO += "tmp.o";
	std::filesystem::remove(tmpO);
	std::filesystem::path tmpM(tmpDir);
	tmpM += std::filesystem::path::preferred_separator;
	tmpM += "tmp.Makefile";
	std::filesystem::remove(tmpM);
	std::filesystem::path tmpC(tmpDir);
	tmpC += std::filesystem::path::preferred_separator;
	tmpC += "tmp.c";
	std::filesystem::remove(tmpC);
	if (std::filesystem::is_empty(tmpDir)) {
		std::filesystem::remove(tmpDir);
		return true;
	} else {
		std::cout << "Couldn't remove tmp directory because it wasn't empty."
				<< std::endl;
		return false;
	}
}
