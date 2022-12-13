/**
 * A function testing whether a character is a digit.
 *
 * @param char The character to check.
 * @return True if the character is between 0 and 9.
 */
function is_digit(char) {
	return char >= '0' && char <= '9'
}

/**
 * Compares the given two list(in string form).
 *
 * @param a The first of the two lists to compare.
 * @param b The second of the lists.
 * @return -1 if a comes before b, 1 if b comes before a, and 0 if they are equal.
 */
function compare_lists(a, b) {
	var ia = 0
	var ib = 0
	while (ia < a.length && ib < b.length) {
		let ca = a[ia]
		let cb = b[ib]
		if (ca == cb) {
			ia++
			ib++
			continue
		}

		if (ca == ']') {
			return -1
		} else if (cb == ']') {
			return 1
		} else if (ca == '[') {
			let layers = 0
			while (a[ia] == '[') {
				ia++
				layers++
			}
			ca = a[ia]

			if (ca == ']') {
				return -1
			} else if (cb == ']') {
				return 1
			} else if (!is_digit(a[ia + 1]) && is_digit(b[ib + 1])) {
				return -1
			} else if (is_digit(a[ia + 1]) && !is_digit(b[ib + 1])) {
				return 1
			} else if (ca < cb) {
				return -1
			} else if (ca > cb) {
				return 1
			} else if (ca == cb) {
				ca = a[++ia]
				cb = b[++ib]
				if (is_digit(ca) && is_digit(cb)) {
					if (ca < cb) {
						return -1
					} else if (ca > cb) {
						return 1
					} else if (ca == cb && a[ia + 1] == ']') {
						ia += layers + 1
						ib++
						continue
					}
				} else if (ca == ']') {
					ia += layers + 1
					ib++
					continue
				}
			}
		} else if (cb == '[') {
			let layers = 0
			while (b[ib] == '[') {
				ib++
				layers++
			}
			cb = b[ib]

			if (ca == ']') {
				return -1
			} else if (cb == ']') {
				return 1
			} else if (!is_digit(a[ia + 1]) && is_digit(b[ib + 1])) {
				return -1
			} else if (is_digit(a[ia + 1]) && !is_digit(b[ib + 1])) {
				return 1
			} else if (ca < cb) {
				return -1
			} else if (ca > cb) {
				return 1
			} else if (ca == cb) {
				ca = a[++ia]
				cb = b[++ib]
				if (is_digit(ca) && is_digit(cb)) {
					if (ca < cb) {
						return -1
					} else if (ca > cb) {
						return 1
					} else if (ca == cb && b[ib + 1] == ']') {
						ia++
						ib += layers + 1
						continue
					}
				} else if (ca == ']') {
					ia++
					ib += layers + 1
					continue
				}
			}
		} else if (is_digit(ca) && is_digit(cb)) {
			if (!is_digit(a[ia + 1]) && is_digit(b[ib + 1])) {
				return -1
			} else if (is_digit(a[ia + 1]) && !is_digit(b[ib + 1])) {
				return 1
			} else if (ca < cb) {
				return -1
			} else if (ca > cb) {
				return 1
			}
		} else if (ca == ',') {
			return -1
		} else if (cb == ',') {
			return 1
		}
	}
	return -1
}

function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(13).split(os.EOL)

	var part1sum = 0
	var last_line;
	var index = 0
	for (let i in lines) {
		if (lines[i].length == 0) {
			continue
		}

		if (!last_line) {
			last_line = lines[i]
			index++
		} else {
			if (compare_lists(last_line, lines[i]) == -1) {
				part1sum += index
			}
			last_line = undefined
		}
	}

	console.log(`The sum of already correctly ordered pairs is ${part1sum}.`)

	const packets = lines.filter(line => line.length > 0)
	packets.push("[[2]]")
	packets.push("[[6]]")
	packets.sort(compare_lists)

	const part2 = (packets.indexOf("[[2]]") + 1) * (packets.indexOf("[[6]]") + 1)
	console.log(`The decode key is ${part2}.`)
}

main()
