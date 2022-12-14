function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(14).split(os.EOL)

	var rock_lines = []
	var x_offset = Number.POSITIVE_INFINITY
	var x_max = 0
	var y_max = 0
	for (let i in lines) {
		if (lines[i].length == 0) {
			continue
		}

		let line = lines[i]
		let comma_idx = line.indexOf(',')
		let space_idx = line.indexOf(' ')
		let rock_line = []
		rock_line[2] = Number(line.substring(0, comma_idx))
		rock_line[3] = Number(line.substring(comma_idx + 1, space_idx))
		line = line.substring(line.indexOf(' ', space_idx + 1) + 1)
		comma_idx = line.indexOf(',')
		space_idx = line.indexOf(' ')

		if (rock_line[2] < x_offset) {
			x_offset = rock_line[2]
		}
		if (rock_line[2] > x_max) {
			x_max = rock_line[2]
		}
		if (rock_line[3] > y_max) {
			y_max = rock_line[3]
		}

		while (comma_idx != -1) {
			rock_line = [rock_line[2], rock_line[3], 0, 0]
			rock_line[2] = Number(line.substring(0, comma_idx))
			if (space_idx != -1) {
				rock_line[3] = Number(line.substring(comma_idx + 1, space_idx))
				line = line.substring(line.indexOf(' ', space_idx + 1) + 1)
			} else {
				rock_line[3] = Number(line.substring(comma_idx + 1))
				line = ""
			}

			rock_lines.push(rock_line)
			comma_idx = line.indexOf(',')
			space_idx = line.indexOf(' ')

			if (rock_line[2] < x_offset) {
				x_offset = rock_line[2]
			}
			if (rock_line[2] > x_max) {
				x_max = rock_line[2]
			}
			if (rock_line[3] > y_max) {
				y_max = rock_line[3]
			}
		}
	}

	var map = Array.from(new Array(y_max + 1), () => new Array(x_max - x_offset + 1).fill(false))
	for (let i in rock_lines) {
		for (let y = Math.min(rock_lines[i][1], rock_lines[i][3]); y <= Math.max(rock_lines[i][1], rock_lines[i][3]); y++) {
			for (let x = Math.min(rock_lines[i][0], rock_lines[i][2]); x <= Math.max(rock_lines[i][0], rock_lines[i][2]); x++) {
				map[y][x - x_offset] = true
			}
		}
	}

	var vanished = false
	var count = 0
	while (!vanished) {
		let pos = [500 - x_offset, 0]
		count++
		while (true) {
			if (pos[1] == y_max) {
				vanished = true
				break
			} else if (!map[pos[1] + 1][pos[0]]) {
				pos[1]++
			} else if (pos[0] == 0) {
				vanished = true
				break
			} else if (!map[pos[1] + 1][pos[0] - 1]) {
				pos[1]++
				pos[0]--
			} else if (pos[0] == x_max - x_offset) {
				vanished = true
				break
			} else if (!map[pos[1] + 1][pos[0] + 1]) {
				pos[1]++
				pos[0]++
			} else {
				map[pos[1]][pos[0]] = true
				break
			}
		}
	}

	console.log(`${count - 1} units of sand can lie on the platform.`)

	// Adjust for part 2 infinite floor.
	y_max += 2
	x_offset -= y_max
	x_max += y_max
	rock_lines.push([x_offset, y_max, x_max, y_max])
	map = Array.from(new Array(y_max + 1), () => new Array(x_max - x_offset + 1).fill(false))
	for (let i in rock_lines) {
		for (let y = Math.min(rock_lines[i][1], rock_lines[i][3]); y <= Math.max(rock_lines[i][1], rock_lines[i][3]); y++) {
			for (let x = Math.min(rock_lines[i][0], rock_lines[i][2]); x <= Math.max(rock_lines[i][0], rock_lines[i][2]); x++) {
				map[y][x - x_offset] = true
			}
		}
	}

	var blocked = false
	count = 0
	while (!blocked) {
		let pos = [500 - x_offset, 0]
		count++
		while (true) {
			if (!map[pos[1] + 1][pos[0]]) {
				pos[1]++
			} else if (!map[pos[1] + 1][pos[0] - 1]) {
				pos[1]++
				pos[0]--
			} else if (!map[pos[1] + 1][pos[0] + 1]) {
				pos[1]++
				pos[0]++
			} else {
				map[pos[1]][pos[0]] = true
				if (pos[1] == 0) {
					blocked = true
				}
				break
			}
		}
	}

	console.log(`${count} units of sand can lie on the floor.`)
}

main()
