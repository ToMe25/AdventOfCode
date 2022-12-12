/**
 * Turns an object with a first and second property into a string with the format "first:second".
 *
 * @param pair The object to convert.
 * @return The string representation of the pair.
 */
function pairToStr(pair) {
	return pair.first + ':' + pair.second
}

function check_position(heights, costs, current_height, current_cost,
	end_cost, new_pos) {
	if (heights[new_pos.second][new_pos.first] < current_height - 1) {
		return false
	}

	if (costs.has(pairToStr(new_pos)) && costs.get(pairToStr(new_pos)) <= current_cost + 1) {
		return false
	}

	if (end_cost > -1 && end_cost <= current_cost + 1) {
		return false
	}

	return true
}

function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(12).split(os.EOL)

	var heights = []
	var end_pos = {}
	var start_pos = {}
	for (let i in lines) {
		const line = lines[i]
		if (line.length == 0) {
			continue
		}

		var row = [];
		for (let j in line) {
			const c = line[j]
			if (c == 'S') {
				start_pos.second = heights.length
				start_pos.first = j
				row.push(0)
			} else if (c == 'E') {
				end_pos.second = heights.length
				end_pos.first = j
				row.push(25)
			} else {
				row.push(c.charCodeAt(0) - 'a'.charCodeAt(0))
			}
		}
		heights.push(row)
	}

	var stack = []
	var costs = new Map()
	stack.push(end_pos)
	costs.set(pairToStr(end_pos), 0)
	var min_cost = -1
	while (stack.length > 0) {
		const current_pos = stack.pop()
		const current_cost = costs.get(pairToStr(current_pos))
		const current_height = heights[current_pos.second][current_pos.first]
		if (costs.has(pairToStr(start_pos)) && current_cost >= costs.get(pairToStr(start_pos))) {
			continue
		}

		if (current_pos.second > 0) {
			const new_pos = {
				first: current_pos.first,
				second: current_pos.second - 1
			}

			if (check_position(heights, costs, current_height,
				current_cost, costs.has(pairToStr(start_pos)) ? costs.get(pairToStr(start_pos)) : -1, new_pos)) {
				costs.set(pairToStr(new_pos), current_cost + 1)
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1
					}
				}

				if (new_pos.first == start_pos.first && new_pos.second == start_pos.second) {
					continue
				} else {
					stack.push(new_pos);
				}
			}
		}

		if (current_pos.second < heights.length - 1) {
			const new_pos = {
				first: current_pos.first,
				second: current_pos.second + 1
			}

			if (check_position(heights, costs, current_height,
				current_cost, costs.has(pairToStr(start_pos)) ? costs.get(pairToStr(start_pos)) : -1, new_pos)) {
				costs.set(pairToStr(new_pos), current_cost + 1)
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1
					}
				}

				if (new_pos.first == start_pos.first && new_pos.second == start_pos.second) {
					continue
				} else {
					stack.push(new_pos);
				}
			}
		}

		if (current_pos.first > 0) {
			const new_pos = {
				first: current_pos.first - 1,
				second: current_pos.second
			}

			if (check_position(heights, costs, current_height,
				current_cost, costs.has(pairToStr(start_pos)) ? costs.get(pairToStr(start_pos)) : -1, new_pos)) {
				costs.set(pairToStr(new_pos), current_cost + 1)
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1
					}
				}

				if (new_pos.first == start_pos.first && new_pos.second == start_pos.second) {
					continue
				} else {
					stack.push(new_pos);
				}
			}
		}

		if (current_pos.first < heights[current_pos.second].length - 1) {
			const new_pos = {
				first: current_pos.first + 1,
				second: current_pos.second
			}

			if (check_position(heights, costs, current_height,
				current_cost, costs.has(pairToStr(start_pos)) ? costs.get(pairToStr(start_pos)) : -1, new_pos)) {
				costs.set(pairToStr(new_pos), current_cost + 1)
				if (heights[new_pos.second][new_pos.first] == 0) {
					if (min_cost < 0 || current_cost + 1 < min_cost) {
						min_cost = current_cost + 1
					}
				}

				if (new_pos.first == start_pos.first && new_pos.second == start_pos.second) {
					continue
				} else {
					stack.push(new_pos);
				}
			}
		}
	}

	console.log(`The shortest path from start to end takes ${costs.get(pairToStr(start_pos))} steps.`)
	console.log(`The shortest hiking path takes ${min_cost} steps.`)
}

main()
