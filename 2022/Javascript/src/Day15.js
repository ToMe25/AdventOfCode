/**
 * Adds add x positions covered by the given sensor on the given Y position to the given set.
 * Except for the one with the beacon, if that is on the given Y position.
 *
 * @param positions The set to add the positions to.
 * @param sensor    Which sensor to add checked positions for.
 * @param pos_y     The Y position on which to check.
 */
function add_covered_x(positions, sensor, pos_y) {
	const dist_y = Math.abs(sensor.pos_y - pos_y)
	if (dist_y > sensor.distance) {
		return
	}

	for (let x = sensor.pos_x - sensor.distance + dist_y; x <= sensor.pos_x + sensor.distance - dist_y; x++) {
		if (sensor.beacon_y != pos_y || sensor.beacon_x != x) {
			positions.add(x);
		}
	}
}

function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(15).split(os.EOL)

	var sensors = []
	for (let i in lines) {
		if (lines[i].length == 0) {
			continue
		}

		let sensor = {}
		let start_pos = 12
		let end_pos = lines[i].indexOf(',')
		sensor.pos_x = Number(lines[i].substring(start_pos, end_pos))
		start_pos = end_pos + 4
		end_pos = lines[i].indexOf(':', start_pos)
		sensor.pos_y = Number(lines[i].substring(start_pos, end_pos))

		start_pos = end_pos + 25
		end_pos = lines[i].indexOf(',', start_pos)
		sensor.beacon_x = Number(lines[i].substring(start_pos, end_pos))
		start_pos = end_pos + 4
		end_pos = lines[i].length
		sensor.beacon_y = Number(lines[i].substring(start_pos, end_pos))
		sensor.distance = Math.abs(sensor.pos_x - sensor.beacon_x) + Math.abs(sensor.pos_y - sensor.beacon_y)
		sensors.push(sensor)
	}

	const checked_line = 2000000
	var checked_positions = new Set()
	for (let i in sensors) {
		add_covered_x(checked_positions, sensors[i], checked_line)
	}

	console.log(`Line ${checked_line} contains ${checked_positions.size} checked positions.`)

	const max_pos = 4000000
	var x = 0
	var y = 0
	var covered = true
	for (; y <= max_pos; y++) {
		while (x <= max_pos) {
			covered = false
			for (let i in sensors) {
				const dist = Math.abs(sensors[i].pos_x - x) + Math.abs(sensors[i].pos_y - y)
				if (dist <= sensors[i].distance) {
					x += sensors[i].distance - dist + 1
					if (x == sensors[i].pos_x) {
						x += sensors[i].distance + 1
					}
					covered = true
					break
				}
			}
			if (!covered) {
				break
			}
		}
		if (!covered) {
			break;
		} else {
			x = 0
		}
	}

	console.log(`The distress signal beacon tuning frequency is ${x * 4000000 + y}.`)
}

main()
