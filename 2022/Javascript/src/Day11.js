/**
 * Parses the monkeys from the input file as objects.
 *
 * @param lines The input file lines to parse.
 * @return A list containing the objects representing the monkeys.
 */
function parseMonkeys(lines) {
	var monkeys = []
	var targets = []
	var id;
	var items = []
	var op_mul;
	var op_const;
	var op_sec;
	var test_div;
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
			monkeys.push({
				id: id, items: items, op_mul: op_mul, op_const: op_const, op_sec: op_sec,
				rel_mod: false, rel_div: 3, test_div: test_div, inspected: 0
			})
			ln = 0
		} else {
			ln++
		}
	}

	for (let i in monkeys) {
		monkeys[i].true_tgt = monkeys[targets[i].true]
		monkeys[i].false_tgt = monkeys[targets[i].false]
	}

	return monkeys
}

/**
 * Executes a single turn for the given monkey.
 *
 * @param monkey The monkey to execute a turn for.
 */
function monkeyTurn(monkey) {
	for (let i in monkey.items) {
		let second = 1
		if (monkey.op_const) {
			second = monkey.op_sec
		} else {
			second = monkey.items[i]
		}

		if (monkey.op_mul) {
			monkey.items[i] *= second
		} else {
			monkey.items[i] += second
		}

		monkey.inspected++

		if (monkey.rel_mod) {
			monkey.items[i] %= monkey.rel_div
		} else {
			monkey.items[i] = Math.floor(monkey.items[i] / monkey.rel_div)
		}

		if (monkey.items[i] % monkey.test_div == 0) {
			monkey.true_tgt.items.push(monkey.items[i])
		} else {
			monkey.false_tgt.items.push(monkey.items[i])
		}
	}
	monkey.items = []
}

function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(11).split(os.EOL)

	var monkeys = parseMonkeys(lines)
	for (let i = 0; i < 20; i++) {
		for (let j in monkeys) {
			monkeyTurn(monkeys[j])
		}
	}

	var inspections = []
	for (let i in monkeys) {
		inspections.push(monkeys[i].inspected)
	}
	inspections.sort(function(a, b){return b - a})

	console.log(`The part 1 monkey business is ${inspections[0] * inspections[1]}.`)

	monkeys = parseMonkeys(lines)
	var lcm = monkeys[0].test_div;
	var gcd
	var a
	var b
	for (let i in monkeys) {
		gcd = monkeys[i].test_div
		a = lcm
		while (a != 0) {
			b = a
			a = gcd % a
			gcd = b
		}
		lcm = (monkeys[i].test_div * lcm) / gcd;
	}

	for (let i in monkeys) {
		monkeys[i].rel_mod = true
		monkeys[i].rel_div = lcm
	}

	for (let i = 0; i < 10000; i++) {
		for (let j in monkeys) {
			monkeyTurn(monkeys[j])
		}
	}

	inspections = []
	for (let i in monkeys) {
		inspections.push(monkeys[i].inspected)
	}
	inspections.sort(function(a, b){return b - a})

	console.log(`The part 2 monkey business is ${inspections[0] * inspections[1]}.`)
}

main()
