/*
 * Day24.h
 *
 *  Created on: 19.11.2022
 *      Author: ToMe25
 */

#ifndef DAY24_H_
#define DAY24_H_

#include "Main.h"
#include <array>
#include <vector>
#include <iostream>

/**
 * An enum representing the different types of instructions the ALU for this day can handle.
 */
enum InstType {
	/**
	 * No-op instruction. Does not do anything.
	 */
	NOP,
	/**
	 * Read an input number and write it to the register.
	 */
	INP,
	/**
	 * Adds input b to register a.
	 */
	ADD,
	/**
	 * Multiplies register a by input b.
	 */
	MUL,
	/**
	 * Divides register a by input b.
	 */
	DIV,
	/**
	 * Sets register a to a mod b.
	 */
	MOD,
	/**
	 * Set a to 1 if a == b, or 0 otherwise.
	 */
	SUB,
	/**
	 * Set a to b.
	 */
	EQL,
	/**
	 * Subtract b from a.
	 */
	SET
};

struct Instruction {
	/**
	 * The instruction type of this instruction.
	 * Aka how the two values should interact.
	 */
	InstType type;
	/**
	 * The primary register this instruction writes to and potentially reads from.
	 */
	uint8_t reg_a;
	bool const_b;
	int32_t in_b;

	/**
	 * Creates a null instruction. Not to be executed.
	 */
	Instruction() :
			type(NOP), reg_a(0), const_b(true), in_b(0) {
	}

	/**
	 * Creates a new instruction instance to be executed.
	 *
	 * @param t		The type of this instruction.
	 * @param a		The register to read from and write to.
	 * @param cb	Whether input b is a constant or a register.
	 * @param b		The register index or value of input b.
	 */
	Instruction(InstType t, uint8_t a, bool cb, int32_t b) :
			type(t), reg_a(a), const_b(cb), in_b(b) {
	}
};

/**
 * Writes a string representation of the given instruction to the given output stream.
 *
 * @param stream	The ostream to write to.
 * @param inst		The instruction to write to the output stream.
 * @return	The given ostream.
 */
std::ostream& operator <<(std::ostream &stream, const Instruction &inst);

/**
 * Optimizes the given program to make it contain fewer instructions.
 * This is done by replacing all sets of instructions that do not use anything
 * touched by input with a single set instruction.
 * Also removes instructions that do nothing.
 *
 * @param insts	The initial instructions of the program to optimize.
 * @return	An optimized list of instructions achieving the same result.
 */
std::vector<Instruction> static_calcs(const std::vector<Instruction> &insts);

/**
 * Removes sets of instructions that calculate something that never gets used.
 *
 * @param insts	The previous instruction set to be optimized.
 * @return	The new hopefully shorter instruction set.
 */
std::vector<Instruction> dead_code_removal(const std::vector<Instruction> &insts);

/**
 * Executes the given set of instructions with the given input.
 * The input is given to the program digit by digit.
 *
 * @param instsv	The instructions to execute.
 * @param instsc	The number of instructions in instsv.
 * @param inputsv	The input numbers to handle.
 * @param inputsc	The number of of inputs.
 * @return	The current register values.
 */
std::array<int64_t, 4> run_programm(const Instruction instsv[],
		const size_t instsc, const uint8_t inputsv[], const size_t inputsc);

#endif /* DAY24_H_ */
