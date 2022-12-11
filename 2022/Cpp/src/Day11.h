/*
 * Day11.h
 *
 *  Created on: Dec 11, 2022
 *      Author: ToMe25
 */

#ifndef DAY11_H_
#define DAY11_H_

#include "Main.h"

namespace aoc {
class Monkey {
private:
	/**
	 * This monkeys numeric id.
	 */
	const uint8_t id;

	/**
	 * True if the operation on the items is multiplication instead of addition.
	 */
	const bool op_mul;

	/**
	 * Whether the second number for the operation is a constant, or the old value.
	 */
	const bool op_const;

	/**
	 * The constant to use as a second value for the operation, if op_const is true.
	 */
	const uint8_t op_sec;

	/**
	 * The divisor to use to determine which monkey an item should be thrown to.
	 */
	const uint8_t test_div;

	/**
	 * The monkey to throw items to for which the test succeeds.
	 */
	Monkey *true_tgt;

	/**
	 * The monkey to throw items to for which the test fails.
	 */
	Monkey *false_tgt;

	/**
	 * The items this monkey currently holds.
	 */
	std::vector<uint32_t> items;

	/**
	 * The number of items this monkey has already inspected.
	 */
	uint32_t inspected = 0;

public:
	/**
	 * Creates a new monkey with the given properties.
	 *
	 * @param id					The numeric id of this monkey.
	 * @param start_items			The items this monkey holds when it is first created.
	 * @param operation_multiply	Whether this monkey should multiply the items value, instead of adding to it.
	 * @param operation_const		Whether the second value for the item value modification is a constant.
	 * @param operation_second		The second value for the operation. Ignored if operation_const is false.
	 * @param test_divisor			The divisor used to check which monkey to throw items to.
	 * @param true_target			The monkey to throw items for which the test returns true to.
	 * @param false_target			The monkey to throw items for which the test returns false to.
	 */
	Monkey(const uint8_t id, const std::vector<uint32_t> start_items,
			const bool operation_multiply, const bool operation_const,
			const uint8_t operation_second, const uint8_t test_divisor,
			Monkey *true_target = NULL, Monkey *false_target = NULL);

	/**
	 * Copies the given monkey to create a new one.
	 *
	 * @param monkey	The monkey to copy.
	 */
	Monkey(const Monkey &monkey);

	/**
	 * Executes a turn for this monkey.
	 * First inspects an item and modifies its worry value.
	 * Then divides its worry value.
	 * Lastly checks which monkey to throw it to, and throws it to that monkey.
	 */
	void turn();

	/**
	 * Set the targets to throw items to.
	 *
	 * @param true_target	The target monkey for items for which the test succeeds.
	 * @param false_target	The target monkey for items for which the test fails.
	 */
	void set_targets(Monkey *true_target, Monkey *false_target);

	/**
	 * Get the number of items this monkey has already inspected.
	 *
	 * @return	The number of items this monkey has already inspected.
	 */
	uint32_t get_inspected() const;

	/**
	 * Print this monkeys id and how many items it has already inspected to the given output stream.
	 *
	 * @param stream	The stream to print the monkey to.
	 * @param monkey	The monkey to print to the stream.
	 * @return	The stream the monkey was printed to.
	 */
	friend std::ostream& operator<<(std::ostream &stream, const Monkey &monkey);
};

/**
 * Print this monkeys id and how many items it has already inspected to the given output stream.
 *
 * @param stream	The stream to print the monkey to.
 * @param monkey	The monkey to print to the stream.
 * @return	The stream the monkey was printed to.
 */
std::ostream& operator<<(std::ostream &stream, const aoc::Monkey &monkey);
}

#endif /* DAY11_H_ */
