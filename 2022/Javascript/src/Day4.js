function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(4).split(os.EOL)

	var useless = 0
	var overlapping = 0
	for (i in lines) {
		if (lines[i].length < 2) {
			continue
		}

		const nums = lines[i].split(/[\-,]/)
		const start1 = Number(nums[0])
		const end1 = Number(nums[1])
		const start2 = Number(nums[2])
		const end2 = Number(nums[3])
		if ((start1 >= start2 && end1 <= end2) || (start1 <= start2 && end1 >= end2)) {
			useless++
		}

		if ((start1 >= start2 && start1 <= end2) || (start2 >= start1 && start2 <= end1)) {
			overlapping++
		}
	}

	console.log(`There are ${useless} completely overlapping assignments.`)
	console.log(`There are ${overlapping} partially overlapping assignments.`)
}

main()
