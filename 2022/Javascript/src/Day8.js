function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(8).split(os.EOL)

	const zero = '0'.charCodeAt(0)
	var map = []
	for (let i in lines) {
		const line = lines[i]
		if (line.length == 0) {
			continue
		}

		map[i] = []
		for (let j in line) {
			map[i][j] = line.charCodeAt(j) - zero
		}
	}

	var visible = 0
	var best_scenic = 0
	for (let y in map) {
		for (x in map[y]) {
			const tree = map[y][x]
			let hidden = false
			let vis = false
			let scenic = 1
			for (let x1 = x - 1; x1 >= 0; x1--) {
				if (map[y][x1] >= tree) {
					hidden = true
					scenic *= x - x1
					break
				}
			}

			if (!hidden) {
				vis = true
				scenic *= x
			}

			hidden = false
			for (let x1 = Number(x) + 1; x1 < map[y].length; x1++) {
				if (map[y][x1] >= tree) {
					hidden = true
					scenic *= x1 - x
					break
				}
			}

			if (!hidden) {
				vis = true
				scenic *= map[y].length - x - 1
			}

			hidden = false
			for (let y1 = y - 1; y1 >= 0; y1--) {
				if (map[y1][x] >= tree) {
					hidden = true
					scenic *= y - y1
					break
				}
			}

			if (!hidden) {
				vis = true
				scenic *= y
			}

			hidden = false
			for (let y1 = Number(y) + 1; y1 < map.length; y1++) {
				if (map[y1][x] >= tree) {
					hidden = true
					scenic *= y1 - y
					break
				}
			}

			if (!hidden) {
				vis = true
				scenic *= map.length - y - 1
			}

			if (vis) {
				visible++;
			}

			if (best_scenic < scenic) {
				best_scenic = scenic;
			}
		}
	}

	console.log(`${visible} trees are visible from the edges.`)
	console.log(`The best tree has a scenic score of ${best_scenic}.`)
}

main()
