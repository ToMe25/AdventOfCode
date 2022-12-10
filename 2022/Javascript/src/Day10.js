function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(10).split(os.EOL)

	console.log("Display:")
	var regX = 1
	var clock = 0
	var score = 0
	var line = ""
	for (let i in lines) {
		if (lines[i].length == 0) {
			continue
		}

		if (clock % 40 >= regX - 1 && clock % 40 <= regX + 1) {
			line += '#'
		} else {
			line += '.'
		}
		clock++
		if (clock % 40 == 0) {
			console.log(line)
			line = ""
		} else if (clock % 40 == 20) {
			score += clock * regX
		}

		if (lines[i].startsWith("addx")) {
			if (clock % 40 >= regX - 1 && clock % 40 <= regX + 1) {
				line += '#'
			} else {
				line += '.'
			}
			clock++
			if (clock % 40 == 0) {
				console.log(line)
				line = ""
			} else if (clock % 40 == 20) {
				score += clock * regX
			}

			regX += Number(lines[i].substring(5))
		}
	}

	console.log(`The part 1 score is ${score}.`)
}

main()
