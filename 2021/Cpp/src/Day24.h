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
#include <iostream>

/**
 * An enum representing the different types of instructions the ALU for this day can handle.
 */
enum InstType {
	/**
	 * Special type marking an Instruction as null.
	 * This means this instruction was not correctly initialized.
	 */
	NUL,
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
	EQL
};

struct Instruction {
	InstType type;
	uint8_t reg_a;
	bool const_b;
	int32_t in_b;

	/**
	 * Creates a null instruction. Not to be executed.
	 */
	Instruction() :
			type(NUL), reg_a(0), const_b(true), in_b(0) {
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
 * Executes the given set of instructions with the given input.
 * The input is given to the program digit by digit.
 *
 * @param instv	A pointer to an Instruction array.
 * @param instc	The size of the instv array.
 * @param input	The input digits to handle.
 * @return	The current register values.
 */
std::array<int64_t, 4> run_programm(const Instruction *instv, const size_t instc,
		const uint8_t input[14]);

#endif /* DAY24_H_ */
