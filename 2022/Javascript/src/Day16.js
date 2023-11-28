/**
 * An object representing the current state of a set of valves and connections.
 */
class State {
	#flow_rate = 0
	#time = 0
	#released = 0
	#valves = new Map()
	#positions = []
	#targets = []
	#distances
	#traveled

	/**
	 * Creates a new state object with the given properties
	 *
	 * @param valves    An array of valves to track.
	 * @param positions An array of agent positions.
	 * @param targets   An array of agent target positions.
	 * @throws {TypeError}  If positions, or targets isn't an array, or valves isn't a map.
	 * @throws {RangeError} If the length of positions and targets isn't equal.
	 */
	constructor(valves, positions, targets) {
		if (valves != undefined) {
			if (!(valves instanceof Map)) {
				throw new TypeError("valves has to be a map.")
			}
			this.#valves = valves
		}
		if (positions != undefined) {
			if (!(positions instanceof Array)) {
				throw new TypeError("positions has to be an array.")
			}
			this.#positions = positions
		}
		if (targets != undefined) {
			if (!(targets instanceof Array)) {
				throw new TypeError("targets has to be an array.")
			}
			this.#targets = targets
		}

		if (this.#positions.length != this.#targets.length) {
			throw new RangeError("positions and targets have to have the same length.")
		}

		this.#distances = new Array(this.#positions.length)
		for (let i in this.#positions) {
			this.#distances[i] = this.get_distance(this.#positions[i], this.#targets[i])
		}

		this.#traveled = new Array(this.#positions.length).fill(0)
	}

	/**
	 * The current rate at which pressure is released.
	 */
	get flow_rate() {
		return this.#flow_rate
	}

	/**
	 * The amount of time that passed since the beginning.
	 */
	get time() {
		return this.#time
	}

	/**
	 * The amount by which the pressure has reduced since the beginning.
	 */
	get released() {
		return this.#released
	}

	/**
	 * The number of valves tracked by this state.
	 */
	get valve_count() {
		return this.#valves.size
	}

	/**
	 * The names of the valves known by this state.
	 */
	get valve_names() {
		return Array.from(this.#valves.keys())
	}

	/**
	 * The number of agents capable of changing the valve states.
	 */
	get agent_count() {
		return this.#positions.length
	}

	/**
	 * Gets the name of the valve at which the person controlling the valves is currently at.
	 *
	 * @param agent	The index of the agent whose position to get.
	 * @return The current position of the person controlling the valves.
	 * @throws {RangeError} If agent is equal to or more than the number of agents.
	 */
	get_position(agent) {
		if (agent >= this.#positions.length) {
			throw new RangeError("agent has to be less than the number of tracked agents.")
		}
		return this.#positions[agent]
	}

	/**
	 * Creates a new state object in which the position of the controller is the given position.
	 * Resets the traveled distance, and recalculates the distance to the target.
	 *
	 * @param agent    The index of the agent whose position to set.
	 * @param position The new position for the agent.
	 * @return	The new state object with the changed position.
	 * @throws {RangeError} If agent is above the limit of the positions array,
	 *                      or position isn't a known valve.
	 */
	set_position(agent, position) {
		if (agent > this.#positions.length) {
			throw new RangeError("agent has to be less than the number of tracked agents.")
		}
		if (!this.#valves.has(position)) {
			throw new RangeError("position has to be a key in the valves map.")
		}

		let newState = this.clone()
		newState.#positions = Array.from(this.#positions)
		newState.#positions[agent] = position
		newState.#distances = Array.from(this.#distances)
		newState.#distances[agent] = this.get_distance(position, newState.#targets[agent])
		newState.#traveled = Array.from(this.#traveled)
		newState.#traveled[agent] = 0
		return newState
	}

	/**
	 * Gets the name of the target valve of the person controlling the valves.
	 *
	 * @param agent	The index of the agent whose position to get.
	 * @return The target position of the person controlling the valves.
	 * @throws {RangeError} If agent is equal to or more than the number of agents.
	 */
	get_target(agent) {
		if (agent >= this.#targets.length) {
			throw new RangeError("agent has to be less than the number of tracked agents.")
		}
		return this.#targets[agent]
	}

	/**
	 * Creates a new state object in which the target position of the given agent is the given position.
	 * Also recalculates the distance to the target.
	 * 
	 * @param agent    The index of the agent whose target position to set.
	 * @param position The new target position for the agent.
	 * @return	The new state object with the changed position.
	 * @throws {RangeError} If agent is above the limit of the targets array,
	 *                      or position isn't a known valve.
	 */
	set_target(agent, position) {
		if (agent > this.#targets.length) {
			throw new RangeError("agent has to be less than the number of tracked agents.")
		}
		if (!this.#valves.has(position)) {
			throw new RangeError("position has to be a key in the valves map.")
		}

		let newState = this.clone()
		newState.#targets = Array.from(this.#targets)
		newState.#targets[agent] = position
		newState.#distances = Array.from(this.#distances)
		newState.#distances[agent] = this.get_distance(newState.#positions[agent], position)
		return newState
	}

	/**
	 * Checks whether the given valve is open.
	 *
	 * @param valve The name of the valve to check.
	 * @return True if the valve is open.
	 * @throws {RangeError} If valve is not a known valve.
	 */
	valve_open(valve) {
		if (!this.#valves.has(valve)) {
			throw new RangeError("valve has to be a key in the valves map.")
		}
		return this.#valves.get(valve).open
	}

	/**
	 * Creates a copy of this state with the given valve opened.
	 * Returns an unmodified copy if the valve is already open.
	 *
	 * @param valve The name of the valve to open.
	 * @return The new modified state.
	 * @throws {RangeError} If valve isn't a known valve.
	 */
	open_valve(valve) {
		if (!this.#valves.has(valve)) {
			throw new RangeError("valve has to be a key in the valves map.")
		}

		let newState = this.clone()
		if (!valve_open(valve)) {
			newState.#valves = new Map(this.#valves)
			newState.#valves.set(valve, JSON.parse(JSON.stringify(newState.#valves.get(valve))))
			newState.#valves.get(valve).open = true
		}
		return newState
	}

	/**
	 * Gets the flow rate of the valve at the given index.
	 *
	 * @param valve The name of the valve to check.
	 * @return The flow rate the given valve allows when it is open.
	 * @throws {RangeError} If valve is not a known valve.
	 */
	get_valve_flow_rate(valve) {
		if (!this.#valves.has(valve)) {
			throw new RangeError("valve has to be a key in the valves map.")
		}
		return this.#valves.get(valve).rate
	}

	/**
	 * Calculates the distance between the two given positions.
	 *
	 * @param from The position from which to start.
	 * @param to   The position to which to find the distance.
	 * @return The distance from "from" to "to".
	 * @throws {RangeError} If from or to isn't a known valve.
	 */
	get_distance(from, to) {
		if (!this.#valves.has(from)) {
			throw new RangeError("from has to be a key in the valves map.")
		}
		if (!this.#valves.has(to)) {
			throw new RangeError("to has to be a key in the valves map.")
		}

		if (from == to) {
			return 0
		}

		let known_distances = new Map()
		known_distances.set(from, 0)
		let stack = [from]
		while (stack.length > 0) {
			const valveName = stack[stack.length - 1]
			stack.pop()
			const distance = known_distances.get(valveName)
			if (known_distances.has(to) && known_distances.get(to) <= distance) {
				continue
			}

			for (let i in this.#valves.get(valveName).connections) {
				const connected = this.#valves.get(valveName).connections[i]
				if (!known_distances.has(connected) || known_distances.get(connected) > distance + 1) {
					known_distances.set(connected, distance + 1)
					stack.push(connected)
				}
			}
		}

		return known_distances.get(to)
	}

	/**
	 * Creates a new state that represents this state after a given amount of time passed.
	 * Updates the released pressure, and moves all agents one step closer to their current target.
	 * Also opens the target valve, if there is time left for that.
	 *
	 * @return The new modified state.
	 */
	add_time(time) {
		let newState = this.clone()
		if (!time) {
			return newState
		}

		newState.#time += time
		newState.#released += newState.#flow_rate * time
		newState.#valves = new Map(newState.#valves)
		newState.#positions = Array.from(newState.#positions)
		newState.#distances = Array.from(newState.#distances)
		newState.#traveled = Array.from(newState.#traveled)
		for (let agent = 0; agent < newState.agent_count; agent++) {
			const remainingTravel = newState.#distances[agent] - newState.#traveled[agent]
			if (time < remainingTravel) {
				newState.#traveled[agent] += time
			} else {
				const target = newState.#targets[agent]
				if (!newState.valve_open(target) && remainingTravel < time) {
					const openTime = newState.#time + remainingTravel - time + 1
					newState.#valves.set(target, JSON.parse(JSON.stringify(newState.#valves.get(target))))
					newState.#valves.get(target).open = true
					newState.#valves.get(target).opened = openTime
					const newFlowRate = newState.#valves.get(target).rate
					newState.#flow_rate += newFlowRate
					newState.#released += newFlowRate * (newState.#time - openTime)
				}
				newState.#positions[agent] = target
				newState.#distances[agent] = newState.#traveled[agent] = 0
			}
		}
		return newState
	}

	/**
	 * Adds the amount of time necessary for the first agent to reach his target and open the valve.
	 * Unless that would be more than max_time, in which case only max_time is added.
	 *
	 * @param max_time The max amount of time to add to the new state.
	 * @return The new modified state.
	 */
	run_to_nex_target(max_time) {
		if (!max_time) {
			return this.clone()
		}

		let time = max_time
		for (let agent = 0; agent < this.agent_count; agent++) {
			if (this.#positions[agent] == this.#targets[agent] && this.valve_open(this.#positions[agent])) {
				return this.clone()
			} else if (this.#distances[agent] - this.#traveled[agent] < time - 1) {
				time = this.#distances[agent] - this.#traveled[agent] + 1
			}
		}
		return this.add_time(time)
	}

	/**
	 * Generates a string representation of this state object.
	 */
	toString() {
		let string = ""
		string += `State {total flow rate=${this.#flow_rate}`
		string += `, time=${this.#time}`
		string += `, released=${this.#released}`
		string += ", agents=["
		for (let i = 0; i < this.agent_count; i++) {
			string += `Agent {index=${i}`
			string += `, position=${this.#positions[i]}`
			string += `, target=${this.#targets[i]}`
			string += `, distance=${this.#distances[i]}`
			string += `, traveled=${this.#traveled[i]}`
			if (i < this.agent_count - 1) {
				string += "}, "
			} else {
				string += '}'
			}
		}

		string += `], valves=[${Array.from(this.#valves.values()).map(valve => {
			return `Valve {name=${valve.name}, flow_rate=${valve.rate}, state=${valve.open ? "open" : "closed"},`
				+ ` opening time=${valve.opened}, connections=[${valve.connections.join(", ")}]}`
		}).join(", ")}]}`
		return string
	}

	/**
	 * Creates a shallow copy of this state object.
	 * This means that the modifying the underlying map and arrays of the copy
	 * will mutate the original.
	 * 
	 * @return The new state object.
	 */
	clone() {
		let newState = new State()
		newState.#flow_rate = this.#flow_rate
		newState.#time = this.#time
		newState.#released = this.#released
		newState.#valves = this.#valves
		newState.#positions = this.#positions
		newState.#targets = this.#targets
		newState.#distances = this.#distances
		newState.#traveled = this.#traveled
		return newState
	}

	/**
	 * Calculates a hash representation of this state object.
	 */
	hash() {
		let result = 1
		result = (result * 31 + this.#flow_rate) | 0
		result = (result * 31 + this.#time) | 0
		result = (result * 31 + this.#released) | 0
		result = Array.from(this.#valves.values()).map(valve => hashValve(valve))
			.reduce((accumulator, valveHash) => (accumulator * 31 + valveHash) | 0, result)
		result = this.#positions.map(position => hashString(position))
			.reduce((accumulator, stringHash) => (accumulator * 31 + stringHash) | 0, result)
		result = this.#targets.map(target => hashString(target))
			.reduce((accumulator, stringHash) => (accumulator * 31 + stringHash) | 0, result)
		result = this.#distances.reduce((accumulator, distance) => (accumulator * 31 + distance) | 0, result)
		result = this.#traveled.reduce((accumulator, traveled) => (accumulator * 31 + traveled) | 0, result)
		return result
	}
}

/**
 * Calculates a simple Java style hash from a string.
 * 
 * @param string The string to hash.
 * @return The hash calculated from the given string.
 * @throws {TypeError} If the type of string isn't "string".
 */
function hashString(string) {
	if (typeof string != 'string') {
		throw new TypeError("string has to be a string.")
	}

	if (string.length == 0) {
		return 0
	}

	let result = 1
	for (let i = 0; i < string.length; i++) {
		const ch = string.charCodeAt(i)
		result = (result * 31 + ch) | 0
	}
	return result
}

/**
 * Calculates a simple Java style hash for a valve object.
 * 
 * @param valve The valve to hash.
 * @return The hash calculated from the given valve.
 */
function hashValve(valve) {
	let result = 1
	result = (result * 31 + hashString(valve.name)) | 0
	result = (result * 31 + valve.rate) | 0
	result = (result * 31 + valve.open) | 0
	result = (result * 31 + valve.opened) | 0
	result = valve.connections.map(connection => hashString(connection))
		.reduce((accumulator, stringHash) => (accumulator * 31 + stringHash) | 0, result)
	return result
}

/**
 * Tests the max amount of pressure that can be released in the given amount of time.
 *
 * @param valves     The valve structure to be used.
 * @param start_pos  The starting position of all the agents.
 * @param time       The time for which to test.
 * @param num_agents The number of agents that can open valves.
 * @return The max amount of pressure that can be released.
 * @throws {TypeError} If valves isn't a map.
 */
function get_max_released(valves, start_pos, time, num_agents) {
	if (!(valves instanceof Map)) {
		throw new TypeError("valves has to be a map.")
	}

	const startPositions = new Array(num_agents).fill(start_pos)
	let stack = [new State(valves, startPositions, startPositions)]
	// Only store hashes to avoid needing way too much RAM.
	let checked = new Set()
	let maxReleased = 0
	while (stack.length > 0) {
		const currentState = stack[stack.length - 1]
		stack.pop()
		checked.add(currentState.hash())
		let closedValves = []
		let closedFlowRates = []
		const valveNames = currentState.valve_names
		for (let i = 0; i < currentState.valve_count; i++) {
			if (!currentState.valve_open(valveNames[i]) && currentState.get_valve_flow_rate(valveNames[i]) > 0) {
				closedValves.push(valveNames[i])
				closedFlowRates.push(currentState.get_valve_flow_rate(valveNames[i]))
			}
		}

		if (closedValves.length == 0) {
			let newState = currentState.clone()
			if (time - currentState.time > 0) {
				newState = newState.add_time(time - currentState.time)
			}

			if (newState.released > maxReleased) {
				maxReleased = newState.released
			}
			continue
		}

		// Ignore states that cannot be better than the current best.
		closedFlowRates = Int32Array.from(closedFlowRates).sort().reverse()
		let mrel = currentState.released + currentState.flow_rate * (time - currentState.time)
		for (let i = 0; i * 2 + 1 < time - currentState.time && i < closedFlowRates.length; i++) {
			mrel += closedFlowRates[i] * (time - currentState.time - i * 2 - 1)
		}
		if (mrel < maxReleased) {
			continue
		}

		let newStates = [currentState]
		for (let agent = 0; agent < currentState.agent_count; agent++) {
			const currentPos = currentState.get_position(agent)
			if (currentPos != currentState.get_target(agent)) {
				continue
			} else if (!currentState.valve_open(currentPos) && currentState.get_valve_flow_rate(currentPos) > 0) {
				continue
			}

			const newStatesCopy = Array.from(newStates)
			for (let i in newStatesCopy) {
				const state = newStatesCopy[i]
				newStates.splice(newStates.indexOf(state), 1)
				for (let j in closedValves) {
					let found = false
					for (let k = 0; k < state.agent_count; k++) {
						if (state.get_target(k) == closedValves[j]) {
							found = true
							break
						}
						if (!found) {
							newStates.push(state.set_target(agent, closedValves[j]))
						}
					}
				}
			}
		}

		for (let i in newStates) {
			let newState = newStates[i]
			if (newState.time < time) {
				newState = newState.run_to_nex_target(time - newState.time)
				if (newState.time == time && newState.released > maxReleased) {
					maxReleased == newState.released
				}
			}

			if (newState.time == time) {
				if (newState.released > maxReleased) {
					maxReleased = newState.released
				}
			} else if (newState.time < time && !stack.includes(newState) && !checked.has(newState.hash())) {
				stack.push(newState)
			}
		}
	}

	return maxReleased
}

function main() {
	const path = require('path')
	const main = require(path.join(path.resolve(''), 'Main.js'))
	const os = require('os')
	const lines = main.getDayInput(16).split(os.EOL)

	let start_valves = new Map()
	for (let i in lines) {
		if (lines[i].length == 0) {
			continue
		}

		const name = lines[i].substring(6, 8)
		const rate = Number(lines[i].substring(23, lines[i].indexOf(';')))
		const conns = lines[i].substring(lines[i].indexOf(';') + 24).split(",").map(conn => conn.trim())
		start_valves.set(name, { name: name, rate: rate, open: false, opened: 0, connections: conns })
	}

	const part1 = get_max_released(start_valves, "AA", 30, 1)
	console.log(`The maximum amount of pressure that one person can release is ${part1}.`)
	const part2 = get_max_released(start_valves, "AA", 26, 2)
	console.log(`The maximum amount of pressure that two persons can release is ${part2}.`)
}

main()
