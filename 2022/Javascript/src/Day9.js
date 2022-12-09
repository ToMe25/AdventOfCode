function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(9).split(os.EOL)

	var positions = []
	for (let i = 0; i < 10; i++) {
		positions[i] = { x: 0, y: 0 }
	}

	var part1visited = new Set()
	part1visited.add("0:0")
	var part2visited = new Set()
	part2visited.add("0:0")
	for (let i in lines) {
		const line = lines[i]
		if (line.length < 3) {
			continue
		}

		const steps = Number(line.substring(2))
		for (let j = 0; j < steps; j++) {
			switch (line[0]) {
				case 'U':
					positions[0].y++
					break
				case 'D':
					positions[0].y--
					break
				case 'L':
					positions[0].x--
					break
				case 'R':
					positions[0].x++
					break
			}

			for (let k = 1; k < 10; k++) {
				let neighbor = false;
				for (let x = -1; x < 2; x++) {
					for (let y = -1; y < 2; y++) {
						if (positions[k - 1].x == positions[k].x + x
							&& positions[k - 1].y == positions[k].y + y) {
							neighbor = true;
							break;
						}
					}

					if (neighbor) {
						break;
					}
				}

				if (!neighbor) {
					if (positions[k - 1].x > positions[k].x) {
						positions[k].x++;
					} else if (positions[k - 1].x < positions[k].x) {
						positions[k].x--;
					}

					if (positions[k - 1].y > positions[k].y) {
						positions[k].y++;
					} else if (positions[k - 1].y < positions[k].y) {
						positions[k].y--;
					}
				}
			}
			part1visited.add(positions[1].x + ':' + positions[1].y);
			part2visited.add(positions[9].x + ':' + positions[9].y);
		}
	}

	console.log(`The second knot visited ${part1visited.size} positions at least once.`)
	console.log(`The tenth knot visited ${part2visited.size} positions at least once.`)
}

main()
