function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(6).split(os.EOL)

	var line = ""
	for (let i in lines) {
		if (lines[i].length > 2) {
			line = lines[i]
			break
		}
	}

	var p1buf = []
	var p1found = false
	var p2buf = []
	for (let i in line) {
		p1buf[i % 4] = line[i]
		p2buf[i % 14] = line[i]
		if (i < 4) {
			continue
		}

		if (!p1found) {
			let failed = false
			for (let j = 0; j < 4; j++) {
				for (let k = 0; k < 4; k++) {
					if (k != j && p1buf[k] == p1buf[j]) {
						failed = true
						break
					}
				}
				if (failed) {
					break
				}
			}

			if (!failed) {
				console.log(`The first packet start was found after ${Number(i) + 1} characters.`)
				p1found = true
			}
		} else if (i >= 14) {
			let failed = false
			for (let j = 0; j < 14; j++) {
				for (let k = 0; k < 14; k++) {
					if (k != j && p2buf[k] == p2buf[j]) {
						failed = true
						break
					}
				}
				if (failed) {
					break
				}
			}

			if (!failed) {
				console.log(`The first message start was found after ${Number(i) + 1} characters.`)
				return
			}
		}
	}
}

main()
