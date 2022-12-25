/*
 * Day16.cpp
 *
 *  Created on: Dec 16, 2022
 *      Author: ToMe25
 */

#include "Day16.h"
#include <map>

aoc::valve::valve(const size_t index, const std::string name,
		const uint8_t flow_rate, const bool open, const uint64_t opened, std::vector<size_t> connections) :
		index(index), name(name), flow_rate(flow_rate), state(open), opened(opened), connections(
				connections) {
}

aoc::valve::valve(const valve &valve) :
		index(valve.index), name(valve.name), flow_rate(valve.flow_rate), state(
				valve.state), opened(valve.opened), connections(valve.connections) {
}

aoc::valve& aoc::valve::operator=(const aoc::valve &valve) {
	index = valve.index;
	name = valve.name;
	flow_rate = valve.flow_rate;
	state = valve.state;
	opened = valve.opened;
	connections = valve.connections;
	return *this;
}

size_t aoc::valve::get_index() const {
	return index;
}

const std::string aoc::valve::get_name() const {
	return name;
}

bool aoc::valve::is_open() const {
	return state;
}

uint64_t aoc::valve::get_opened() const {
	return opened;
}

aoc::valve aoc::valve::open(const uint64_t time) const {
	return aoc::valve(index, name, flow_rate, true, time, connections);
}

uint8_t aoc::valve::get_flow_rate() const {
	return flow_rate;
}

const std::vector<size_t>& aoc::valve::get_connections() const {
	return connections;
}

aoc::valve aoc::valve::add_connection(const size_t index) const {
	valve new_valve(*this);
	new_valve.connections = std::vector<size_t>(new_valve.connections);
	if (std::count(new_valve.connections.begin(), new_valve.connections.end(), index) == 0) {
		new_valve.connections.push_back(index);
	}
	return new_valve;
}

std::ostream& aoc::operator<<(std::ostream &stream, const aoc::valve &valve) {
	stream << "valve[index=" << valve.index;
	stream << ", name=" << valve.name;
	stream << ", flow rate=" << (uint16_t) valve.flow_rate;
	stream << ", state=" << (valve.state ? "open" : "closed");
	stream << ", opening time=" << valve.opened;
	stream << ", connections=[";
	for (size_t i = 0; i < valve.connections.size(); i++) {
		stream << valve.connections[i];
		if (i < valve.connections.size() - 1) {
			stream << ", ";
		}
	}
	stream << "]]";
	return stream;
}

aoc::state::state(std::vector<valve> valves, const uint64_t flow_rate,
		const uint64_t time, const uint64_t released,
		const size_t position) :
		valves(valves), flow_rate(flow_rate), time(time), released(released), position(position) {
}

aoc::state::state(const aoc::state &state) :
		valves(state.valves), flow_rate(state.flow_rate), time(state.time), released(
				state.released), position(state.position) {
}

aoc::state& aoc::state::operator=(const aoc::state &state) {
	valves = state.valves;
	flow_rate = state.flow_rate;
	time = state.time;
	released = state.released;
	position = state.position;
	return *this;
}

aoc::state aoc::state::update() const {
	return aoc::state(valves, flow_rate, time + 1, released + flow_rate, position);
}

uint64_t aoc::state::get_flow_rate() const {
	return flow_rate;
}

aoc::state aoc::state::set_flow_rate(const uint64_t flow_rate) const {
	return aoc::state(valves, flow_rate, time, released, position);
}

aoc::state aoc::state::add_flow_rate(const uint64_t flow_rate) const {
	return aoc::state(valves, this->flow_rate + flow_rate, time, released, position);
}

uint64_t aoc::state::get_time() const {
	return time;
}

uint64_t aoc::state::get_released() const {
	return released;
}

size_t aoc::state::get_position() const {
	return position;
}

aoc::state aoc::state::set_position(const size_t position) const {
	return aoc::state(valves, flow_rate, time, released, position);
}

size_t aoc::state::get_size() const {
	return valves.size();
}

const std::vector<aoc::valve> aoc::state::get_valves() const {
	return valves;
}

const aoc::valve& aoc::state::get_valve(const size_t index) const {
	return valves[index];
}

aoc::state aoc::state::set_valve(aoc::valve &valve) const {
	state new_state(*this);
	new_state.valves[valve.get_index()] = valve;
	return new_state;
}

aoc::state aoc::state::add_valve(const std::string name,
		const uint8_t flow_rate, const bool open, const uint64_t opened,
		std::vector<size_t> &connections) const {
	state new_state(*this);
	new_state.valves.push_back(valve(new_state.valves.size(), name, flow_rate, open, opened, connections));
	return new_state;
}

bool aoc::state::valve_open(const size_t idx) const {
	return valves[idx].is_open();
}

aoc::state aoc::state::open_valve(const size_t idx) const {
	state new_state(*this);
	new_state.valves[idx] = new_state.valves[idx].open(time);
	if (!valves[idx].is_open()) {
		new_state.flow_rate += valves[idx].get_flow_rate();
	}
	return new_state;
}

uint8_t aoc::state::get_valve_flow_rate(const size_t idx) const {
	return valves[idx].get_flow_rate();
}

const std::vector<size_t>& aoc::state::get_valve_connections(const size_t idx) const {
	return valves[idx].get_connections();
}

size_t aoc::state::get_distance_to(const size_t target) const {
	if (target == position) {
		return 0;
	}

	std::map<size_t, size_t> known_distances;
	known_distances[position] = 0;
	std::vector<size_t> stack;
	stack.push_back(position);
	while (stack.size() > 0) {
		const size_t pos = stack.back();
		stack.pop_back();
		const size_t distance = known_distances[pos];
		if (known_distances.count(target) == 1 && distance >= known_distances[target]) {
			continue;
		}

		for (size_t connected : valves[pos].get_connections()) {
			if (known_distances.count(connected) == 0
					|| distance + 1 < known_distances[connected]) {
				known_distances[connected] = distance + 1;
				stack.push_back(connected);
			}
		}
	}

	return known_distances[target];
}

std::ostream& aoc::operator<<(std::ostream &stream, const aoc::state &state) {
	stream << "state[total flow rate=" << state.flow_rate;
	stream << ", time=" << state.time;
	stream << ", released=" << state.released;
	stream << ", position=" << state.position;
	stream << ", valves=[";
	for (size_t i = 0; i < state.valves.size(); i++) {
		stream << state.valves[i];
		if (i < state.valves.size() - 1) {
			stream << ", ";
		}
	}
	stream << "]]";
	return stream;
}

std::string day16part1(std::ifstream input) {
	std::vector<aoc::valve> start_valves;
	std::map<std::string, size_t> name_to_idx;
	std::vector<std::vector<std::string>> connections;
	std::string line;
	while (std::getline(input, line)) {
		if (line.length() == 0) {
			continue;
		}

		std::string name = line.substr(6, 2);
		size_t semicolon = line.find(';');
		uint8_t rate = std::stoi(line.substr(23, semicolon - 23));
		start_valves.push_back(aoc::valve(start_valves.size(), name, rate));
		std::string conns = line.substr(semicolon + 24) + ',';
		conns.erase(std::remove(conns.begin(), conns.end(), ' '), conns.end());
		name_to_idx.insert( { name, start_valves.size() - 1 });
		std::vector<std::string> valve_conns;
		while (conns.length() > 0) {
			valve_conns.push_back(conns.substr(0, 2));
			conns = conns.substr(3);
		}
		connections.push_back(valve_conns);
	}

	for (size_t i = 0; i < start_valves.size(); i++) {
		for (std::string connection : connections[i]) {
			start_valves[i] = start_valves[i].add_connection(name_to_idx[connection]);
		}
	}

	aoc::state initial_state(start_valves);
	initial_state = initial_state.set_position(name_to_idx["AA"]);

	std::vector<aoc::state> stack;
	stack.push_back(initial_state);
	aoc::state max_released {std::vector<aoc::valve>() };
	while (stack.size() > 0) {
		const aoc::state current_state = stack.back();
		stack.pop_back();
		std::vector<size_t> closed_valves;
		for (size_t i = 0; i < current_state.get_size(); i++) {
			if (!current_state.valve_open(i) && current_state.get_valve_flow_rate(i) > 0) {
				closed_valves.push_back(i);
			}
		}

		if (closed_valves.size() == 0) {
			aoc::state new_state = current_state;
			for (size_t i = 0; i < 30 - current_state.get_time(); i++) {
				new_state = new_state.update();
			}
			if (max_released.get_released() < new_state.get_released()) {
				max_released = new_state;
			}
			continue;
		}

		for (const size_t val : closed_valves) {
			aoc::state new_state = current_state;
			const size_t dist = current_state.get_distance_to(val);
			for (size_t i = 0; i <= dist; i++) {
				new_state = new_state.update();
				if (new_state.get_time() == 30) {
					if (max_released.get_released() < new_state.get_released()) {
						max_released = new_state;
					}
					break;
				}
			}

			new_state = new_state.open_valve(val);
			new_state = new_state.set_position(val);
			if (new_state.get_time() == 30) {
				if (max_released.get_released() < new_state.get_released()) {
					max_released = new_state;
				}
			} else if (new_state.get_time() < 30) {
				stack.push_back(new_state);
			}
		}
	}

	std::vector<aoc::valve> open_valves;
	for (size_t i = 0; i < max_released.get_size(); i++) {
		if (max_released.valve_open(i)) {
			open_valves.push_back(max_released.get_valve(i));
		}
	}

	return std::to_string(max_released.get_released());
}

bool d16p1 = aoc::registerPart1(16, &day16part1);
