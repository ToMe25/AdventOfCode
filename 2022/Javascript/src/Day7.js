function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(7).split(os.EOL)

	var root = { name: '/', parent: undefined, size: 0, children: {}, directory: true }
	var current = root
	for (let i in lines) {
		const line = lines[i]
		if (line.length < 2) {
			continue
		} else if (line.startsWith('$')) {
			const cmd = line.substring(2, 4)
			if (cmd == "cd") {
				const dir = line.substring(5)
				if (dir == '/') {
					current = root
				} else if (dir == "..") {
					current = current.parent
				} else {
					current = current.children[dir]
				}
			}
		} else if (line.startsWith("dir")) {
			current.children[line.substring(4)] = { name: line.substring(4), parent: current, size: 0, children: {}, directory: true }
		} else {
			const split = line.split(' ')
			const size = Number(split[0])
			current.children[split[1]] = { name: split[1], parent: current, size: size, children: {}, directory: false }
			let parent = current
			while (parent) {
				parent.size += size
				parent = parent.parent
			}
		}
	}

	var part1sum = 0
	var stack = [root]
	while (stack.length > 0) {
		const current = stack.pop()
		if (current.size <= 100000) {
			part1sum += current.size
		}

		for (let i in current.children) {
			if (current.children[i].directory) {
				stack.push(current.children[i])
			}
		}
	}

	console.log(`The sum of the sizes of all directories with a size of at most 100000 is ${part1sum}.`)

	const missingSpace = 30000000 - (70000000 - root.size)
	var part2size = root.size
	var stack = [root]
	while (stack.length > 0) {
		const current = stack.pop()
		if (current.size >= missingSpace && current.size < part2size) {
			part2size = current.size
		}

		for (let i in current.children) {
			if (current.children[i].directory) {
				stack.push(current.children[i])
			}
		}
	}

	console.log(`The smallest directory large enough to clear the missing space has a size of ${part2size}.`)
}

main()
