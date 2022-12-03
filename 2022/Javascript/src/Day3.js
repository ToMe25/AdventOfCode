function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(3).split(os.EOL)

	const A = 'A'.charCodeAt(0)
	const a = 'a'.charCodeAt(0)
	const Z = 'Z'.charCodeAt(0)
	var part1sum = 0
	for (let i in lines) {
		const line = lines[i]
		if (line.length < 2) {
			continue
		}

		const end1 = line.length / 2
		const str2 = line.substring(end1)
		for (let i = 0; i < end1; i++) {
			if (str2.includes(line[i])) {
				const c = line.charCodeAt(i)
				part1sum += (c > Z ? c - a : c - A + 26) + 1
				break;
			}
		}
	}

	console.log(`The part 1 priority sum is ${part1sum}.`)

	var part2sum = 0
	var chars = ""
	for (let i in lines) {
		const line = lines[i]
		if (line.length < 2) {
			continue
		}

		let newchars = ""
		for (j in line) {
			let c = line[j]
			if (!newchars.includes(c) && (i % 3 == 0 || chars.includes(c))) {
				newchars += c
			}
		}

		chars = newchars

		if (i % 3 == 2) {
			const c = chars.charCodeAt(0);
			part2sum += (c > Z ? c - a : c - A + 26) + 1
		}
	}

	console.log(`The part 1 priority sum is ${part2sum}.`)
}

main()
