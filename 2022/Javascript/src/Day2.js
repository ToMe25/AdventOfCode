function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(2).split(os.EOL)

	var p1score = 0
	var p2score = 0
	const A = 'A'.charCodeAt(0)
	const X = 'X'.charCodeAt(0)
	for (let i in lines) {
		if (lines[i].length < 2) {
			continue
		}

		var enemy = lines[i].charCodeAt(0) - A
		var mine = lines[i].charCodeAt(2) - X
		p1score += mine + 1
		if (enemy == mine - 1) {
			p1score += 6
		} else if (enemy == 2 && mine == 0) {
			p1score += 6
		} else if (enemy == mine) {
			p1score += 3
		}

		p2score += mine * 3
		p2score += (enemy + 3 + mine - 1) % 3 + 1
	}

	console.log(`The part 1 score is ${p1score}.`)
	console.log(`The part 2 score is ${p2score}.`)
}

main()
