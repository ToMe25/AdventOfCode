function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(5).split(os.EOL)

	var crates = []
	var crates2 = undefined
	for (i in lines) {
		if (lines[i].length < 3) {
			continue
		} else if (lines[i].trim().startsWith('[')) {
			for (let j = 1; j < lines[i].length + 4; j += 4) {
				if (!lines[i].charAt(j).match(/[0-9a-z]/i)) {
					continue;
				}

				if (!crates[(j - 1) / 4]) {
					crates[(j - 1) / 4] = ""
				}
				crates[(j - 1) / 4] = lines[i][j] + crates[(j - 1) / 4]
			}
		} else if (lines[i].startsWith("move")) {
			if (!crates2) {
				crates2 = crates.slice()
			}

			const split = lines[i].split(' ');
			const count = Number(split[1])
			const start = Number(split[3]) - 1
			const target = Number(split[5]) - 1
			for (let j = 0; j < count; j++) {
				crates[target] += crates[start].charAt(crates[start].length - 1)
				crates[start] = crates[start].substring(0, crates[start].length - 1)
			}
			crates2[target] += crates2[start].substring(crates2[start].length - count)
			crates2[start] = crates2[start].substring(0, crates2[start].length - count)
		}
	}

	var part1res = ""
	for (i in crates) {
		part1res += crates[i][crates[i].length - 1]
	}

	console.log(`The part one top crates are ${part1res}.`)

	var part2res = ""
	for (i in crates2) {
		part2res += crates2[i][crates2[i].length - 1]
	}

	console.log(`The part two top crates are ${part2res}.`)
}

main()
