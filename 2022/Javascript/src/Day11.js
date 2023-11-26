/**
 * An object representing a Monkey from the input data.
 */
class Monkey {
	#id
	#op_mul
	#op_const
	#op_sec
	#rel_mod
	#rel_div
	#test_div
	#true_tgt
	#false_tgt
	#items
	#inspected

	/**
	 * Creates a new monkey with the given properties.
	 *
	 * @param id                 The numeric id of this monkey.
	 * @param start_items        The items this monkey holds when it is first created.
	 * @param operation_multiply Whether this monkey should multiply the items value, instead of adding to it.
	 * @param operation_const    Whether the second value for the item value modification is a constant.
	 * @param operation_second   The second value for the operation. Ignored if operation_const is false.
	 * @param relief_mod         Whether the worry reduction should be a modulo operation instead of a division.
	 * @param relief_divisor     The divisor for the worry reduction operation.
	 * @param test_divisor       The divisor used to check which monkey to throw items to.
	 * @param true_target        The monkey to throw items for which the test returns true to.
	 * @param false_target       The monkey to throw items for which the test returns false to.
	 */
	constructor(id, start_items, operation_multiply, operation_const, operation_second, relief_mod,
		relief_divisor, test_divisor, true_target, false_target) {
		this.#id = id
		this.#op_mul = operation_multiply
		this.#op_const = operation_const
		this.#op_sec = operation_second
		this.#rel_mod = relief_mod
		this.#rel_div = relief_divisor
		this.#test_div = test_divisor
		this.#true_tgt = true_target
		this.#false_tgt = false_target
		this.#items = start_items
		this.#inspected = 0
	}

	/**
	 * Executes a single turn for this monkey.
	 */
	turn() {
		for (let i in this.#items) {
			let second = 1
			if (this.#op_const) {
				second = this.#op_sec
			} else {
				second = this.#items[i]
			}

			if (this.#op_mul) {
				this.#items[i] *= second
			} else {
				this.#items[i] += second
			}

			this.#inspected++

			if (this.#rel_mod) {
				this.#items[i] %= this.#rel_div
			} else {
				this.#items[i] = Math.floor(this.#items[i] / this.#rel_div)
			}

			if (this.#items[i] % this.#test_div == 0) {
				this.#true_tgt.#items.push(this.#items[i])
			} else {
				this.#false_tgt.#items.push(this.#items[i])
			}
		}
		this.#items = []
	}

	/**
	 * Set the targets to throw items to.
	 *
	 * @param true_target  The target monkey for items for which the test succeeds.
	 * @param false_target The target monkey for items for which the test fails.
	 */
	set_targets(true_target, false_target) {
		if (true_target != undefined && !(true_target instanceof Monkey)) {
			throw new TypeError("true_target has to be a Monkey")
		}
		if (false_target != undefined && !(false_target instanceof Monkey)) {
			throw new TypeError("false_target has to be a Monkey")
		}

		this.#true_tgt = true_target
		this.#false_tgt = false_target
	}

	/**
	 * The number of items this monkey has already inspected.
	 */
	get inspected() {
		return this.#inspected
	}

	/**
	 * The divisor to use to determine which monkey an item should be thrown to.
	 */
	get test_divisor() {
		return this.#test_div
	}

	/**
	 * Sets how the worry value should be reduced after a monkey inspected an item.
	 *
	 * @param relief_mod     Whether the worry reduction should be a modulo operation instead of a division.
	 * @param relief_divisor The divisor for the worry reduction operation.
	 */
	set_relief(relief_mod, relief_divisor) {
		this.#rel_mod = relief_mod
		this.#rel_div = relief_divisor
	}

	/**
	 * Converts this monkey to its string representation.
	 * 
	 * @return The string representation of this monkey.
	 */
	toString() {
		return `Monkey ${this.#id}: ${this.#items.join(", ")}`
	}
}

/**
 * Parses the monkeys from the input file as objects.
 *
 * @param lines The input file lines to parse.
 * @return A list containing the objects representing the monkeys.
 */
function parseMonkeys(lines) {
	var monkeys = []
	var targets = []
	var id
	var items = []
	var op_mul
	var op_const
	var op_sec
	var test_div
	// Which of the 5 lines of a monkey definition are we parsing
	var ln = 0;
	for (let i in lines) {
		if (lines[i].length == 0) {
			continue
		}

		lines[i] = lines[i].trim()

		let err = false
		switch (ln) {
			case 0:
				if (!lines[i].startsWith("Monkey")) {
					console.error(`Trying to parse invalid id line "${lines[i]}.`)
					err = true
				} else {
					id = Number(lines[i].substring(7, lines[i].length - 1))
				}
				break
			case 1:
				if (!lines[i].startsWith("Starting items")) {
					console.error(`Trying to parse invalid items line "${lines[i]}".`)
					err = true
				} else {
					items = []
					let split = lines[i].substring(16).split(", ")
					for (let j in split) {
						items.push(Number(split[j]))
					}
				}
				break
			case 2:
				if (!lines[i].startsWith("Operation")) {
					console.error(`Trying to parse invalid operation line "${lines[i]}.`)
					err = true
				} else {
					op_mul = lines[i][21] == '*'
					let nr = lines[i].substr(23)
					op_const = nr != "old"
					op_sec = op_const ? Number(nr) : 0
				}
				break
			case 3:
				if (!lines[i].startsWith("Test")) {
					console.error(`Trying to parse invalid test line "${lines[i]}.`)
					err = true
				} else {
					test_div = Number(lines[i].substring(19))
				}
				break
			case 4:
				if (!lines[i].startsWith("If true")) {
					console.error(`Trying to parse invalid if true line "${lines[i]}.`)
					err = true
				} else {
					targets.push({ true: Number(lines[i].substring(25)) })
				}
				break
			case 5:
				if (!lines[i].startsWith("If false")) {
					console.error(`Trying to parse invalid if false line "${lines[i]}.`)
					err = true
				} else {
					targets[targets.length - 1].false = Number(lines[i].substring(26))
				}
				break;
		}

		if (err) {
			ln = 0
		} else if (ln == 5) {
			monkeys.push(new Monkey(
				id, items, op_mul, op_const, op_sec, false, 3, test_div, undefined, undefined))
			ln = 0
		} else {
			ln++
		}
	}

	for (let i in monkeys) {
		monkeys[i].set_targets(monkeys[targets[i].true], monkeys[targets[i].false])
	}

	return monkeys
}

function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(11).split(os.EOL)

	var monkeys = parseMonkeys(lines)
	for (let i = 0; i < 20; i++) {
		for (let j in monkeys) {
			monkeys[j].turn()
		}
	}

	var inspections = []
	for (let i in monkeys) {
		inspections.push(monkeys[i].inspected)
	}
	inspections.sort(function(a, b) { return b - a })

	console.log(`The part 1 monkey business is ${inspections[0] * inspections[1]}.`)

	monkeys = parseMonkeys(lines)
	var lcm = monkeys[0].test_divisor;
	var gcd
	var a
	var b
	for (let i in monkeys) {
		gcd = monkeys[i].test_divisor
		a = lcm
		while (a != 0) {
			b = a
			a = gcd % a
			gcd = b
		}
		lcm = (monkeys[i].test_divisor * lcm) / gcd;
	}

	for (let i in monkeys) {
		monkeys[i].set_relief(true, lcm)
	}

	for (let i = 0; i < 10000; i++) {
		for (let j in monkeys) {
			monkeys[j].turn()
		}
	}

	inspections = []
	for (let i in monkeys) {
		inspections.push(monkeys[i].inspected)
	}
	inspections.sort(function(a, b) { return b - a })

	console.log(`The part 2 monkey business is ${inspections[0] * inspections[1]}.`)
}

main()
