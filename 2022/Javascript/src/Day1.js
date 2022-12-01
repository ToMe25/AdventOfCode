function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(1).split(os.EOL)

	var max = new Array(3)
	var current = 0
	for (var i in lines) {
		var line = lines[i]

		if (line.length > 0) {
			current += Number(line)
		} else {
			if (max[0] == undefined || current >= max[0]) {
				max[2] = max[1]
				max[1] = max[0]
				max[0] = current
			} else if (max[1] == undefined || current >= max[1]) {
				max[2] = max[1]
				max[1] = current
			} else if (max[2] == undefined || current > max[2]) {
				max[2] = current
			}
			current = 0
		}
	}

	if (max[0] == undefined || current >= max[0]) {
		max[2] = max[1]
		max[1] = max[0]
		max[0] = current
	} else if (max[1] == undefined || current >= max[1]) {
		max[2] = max[1]
		max[1] = current
	} else if (max[2] == undefined || current > max[2]) {
		max[2] = current
	}

	console.log(`The elf with the most food carries ${max[0]} calories.`)
	console.log(`The top three elves carry a total of ${max[0] + max[1] + max[2]} calories.`)
}

main()
