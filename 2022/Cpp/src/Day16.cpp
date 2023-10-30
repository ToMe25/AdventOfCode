/*
 * Day16.cpp
 *
 *  Created on: Dec 16, 2022
 *      Author: ToMe25
 */

#include "Day16.h"
#include <map>

aoc::valve::valve() noexcept :
		index(0), name(), flow_rate(0), state(false), opened(0), connections(
		NULL), num_connections(0) {

}

aoc::valve::valve(const size_t index, const std::string_view name,
		const uint8_t flow_rate, const bool open, const uint64_t opened,
		const size_t *connections, const size_t num_connections) :
		index(index), name(name), flow_rate(flow_rate), state(open), opened(
				opened), connections(
				num_connections > 0 ? new size_t[num_connections] : NULL), num_connections(
				num_connections) {
	if (num_connections > 0) {
		std::copy(connections, connections + num_connections,
				this->connections);
	}
}

aoc::valve::valve(const valve &valve) :
		index(valve.index), name(valve.name), flow_rate(valve.flow_rate), state(
				valve.state), opened(valve.opened), connections(
				valve.num_connections > 0 ?
						new size_t[valve.num_connections] : NULL), num_connections(
				valve.num_connections) {
	if (num_connections > 0) {
		std::copy(valve.connections, valve.connections + valve.num_connections,
				connections);
	}
}

aoc::valve::valve(valve &&valve) noexcept :
		index(0), name(), flow_rate(0), state(false), opened(0), connections(
		NULL), num_connections(0) {
	using std::swap;

	swap(*this, valve);
}

aoc::valve::~valve() {
	delete[] connections;
}

aoc::valve& aoc::valve::operator=(valve valve) noexcept {
	using std::swap;

	swap(*this, valve);
	return *this;
}

size_t aoc::valve::get_index() const {
	return index;
}

const std::string_view& aoc::valve::get_name() const {
	return name;
}

bool aoc::valve::is_open() const {
	return state;
}

uint64_t aoc::valve::get_opened() const {
	return opened;
}

aoc::valve aoc::valve::open(const uint64_t time) const {
	return valve(index, name, flow_rate, true, time, connections,
			num_connections);
}

uint8_t aoc::valve::get_flow_rate() const {
	return flow_rate;
}

const std::vector<size_t> aoc::valve::get_connections() const {
	return std::vector<size_t>(connections, connections + num_connections);
}

const size_t* aoc::valve::get_connections_ptr() const {
	return connections;
}

size_t aoc::valve::get_connections_count() const {
	return num_connections;
}

aoc::valve aoc::valve::add_connection(const size_t index) const {
	if (std::count(connections, connections + num_connections, index) > 0) {
		return valve(*this);
	}

	valve new_valve(this->index, name, flow_rate, state, opened, NULL, 0);
	new_valve.connections = new size_t[num_connections + 1];
	std::copy(connections, connections + num_connections,
			new_valve.connections);
	new_valve.connections[num_connections] = index;
	new_valve.num_connections = num_connections + 1;
	return new_valve;
}

std::ostream& aoc::operator<<(std::ostream &stream, const aoc::valve &valve) {
	stream << "valve[index=" << valve.index;
	stream << ", name=" << valve.name;
	stream << ", flow rate=" << (uint16_t) valve.flow_rate;
	stream << ", state=" << (valve.state ? "open" : "closed");
	stream << ", opening time=" << valve.opened;
	stream << ", connections=[";
	for (size_t i = 0; i < valve.num_connections; i++) {
		stream << valve.connections[i];
		if (i < valve.num_connections - 1) {
			stream << ", ";
		}
	}
	stream << "]]";
	return stream;
}

void aoc::swap(valve &first, valve &second) noexcept {
	using std::swap;

	swap(first.index, second.index);
	swap(first.name, second.name);
	swap(first.flow_rate, second.flow_rate);
	swap(first.state, second.state);
	swap(first.opened, second.opened);
	swap(first.connections, second.connections);
	swap(first.num_connections, second.num_connections);
}

aoc::state::state(const uint64_t flow_rate, const uint64_t time,
		const uint64_t released, const valve *valves, const size_t num_valves,
		const size_t *positions, const size_t *targets, const size_t num_agents) :
		flow_rate(flow_rate), time(time), released(released), valves(
				num_valves > 0 ? new valve[num_valves] : NULL), num_valves(
				num_valves), positions(
				num_agents > 0 ? new size_t[num_agents] : NULL), targets(
				num_agents > 0 ? new size_t[num_agents] : NULL), num_agents(
				num_agents) {
	if (num_valves > 0) {
		std::copy(valves, valves + num_valves, this->valves);
	}

	if (num_agents > 0) {
		std::copy(positions, positions + num_agents, this->positions);
		std::copy(targets, targets + num_agents, this->targets);
	}
}

aoc::state::state(const state &state) :
		flow_rate(state.flow_rate), time(state.time), released(state.released), valves(
				state.num_valves > 0 ? new valve[state.num_valves] : NULL), num_valves(
				state.num_valves), positions(
				state.num_agents > 0 ? new size_t[state.num_agents] : NULL), targets(
				state.num_agents > 0 ? new size_t[state.num_agents] : NULL), num_agents(
				state.num_agents) {
	if (num_valves > 0) {
		std::copy(state.valves, state.valves + state.num_valves, valves);
	}

	if (num_agents > 0) {
		std::copy(state.positions, state.positions + state.num_agents,
				positions);
		std::copy(state.targets, state.targets + state.num_agents, targets);
	}
}

aoc::state::state(state &&state) noexcept :
		flow_rate(0), time(0), released(0), valves(NULL), num_valves(0), positions(
		NULL), targets(NULL), num_agents(0) {
	using std::swap;

	swap(*this, state);
}

aoc::state::~state() {
	delete[] valves;
	delete[] positions;
	delete[] targets;
}

aoc::state& aoc::state::operator=(state state) noexcept {
	using std::swap;

	swap(*this, state);
	return *this;
}

aoc::state aoc::state::update() const {
	return add_time(1);
}

aoc::state aoc::state::add_time(const uint64_t time) const {
	return state(flow_rate, this->time + time, released + flow_rate * time,
			valves, num_valves, positions, targets, num_agents);
}

uint64_t aoc::state::get_flow_rate() const {
	return flow_rate;
}

aoc::state aoc::state::set_flow_rate(const uint64_t flow_rate) const {
	return state(flow_rate, time, released, valves, num_valves, positions,
			targets, num_agents);
}

aoc::state aoc::state::add_flow_rate(const uint64_t flow_rate) const {
	return state(this->flow_rate + flow_rate, time, released, valves,
			num_valves, positions, targets, num_agents);
}

uint64_t aoc::state::get_time() const {
	return time;
}

uint64_t aoc::state::get_released() const {
	return released;
}

size_t aoc::state::get_position(const size_t agent) const {
	if (agent >= num_agents) {
		throw std::out_of_range("Agent index out of range.");
	}
	return positions[agent];
}

aoc::state aoc::state::set_position(const size_t agent,
		const size_t position) const {
	if (agent >= num_agents) {
		throw std::out_of_range("Agent index out of range.");
	}
	if (position >= num_valves) {
		throw std::out_of_range("Position out of range.");
	}

	state new_state(*this);
	new_state.positions[agent] = position;
	return new_state;
}

size_t aoc::state::get_target(const size_t agent) const {
	if (agent >= num_agents) {
		throw std::out_of_range("Agent index out of range.");
	}
	return targets[agent];
}

aoc::state aoc::state::set_target(const size_t agent,
		const size_t position) const {
	if (agent >= num_agents) {
		throw std::out_of_range("Agent index out of range.");
	}
	if (position >= num_valves) {
		throw std::out_of_range("Position out of range.");
	}

	state new_state(*this);
	new_state.targets[agent] = position;
	return new_state;
}

size_t aoc::state::get_size() const {
	return num_valves;
}

const std::vector<aoc::valve> aoc::state::get_valves() const {
	return std::vector<aoc::valve>(valves, valves + num_valves);
}

const aoc::valve& aoc::state::get_valve(const size_t index) const {
	if (index >= num_valves) {
		throw std::out_of_range("Valve index out of range.");
	}
	return valves[index];
}

aoc::state aoc::state::set_valve(valve &valve) const {
	if (valve.get_index() > num_valves) {
		throw std::out_of_range("Valve index out of range.");
	}

	state new_state(flow_rate, time, released, NULL, 0, positions, targets,
			num_agents);
	if (valve.get_index() == num_valves) {
		new_state.valves = new aoc::valve[num_valves + 1];
		new_state.num_valves = num_valves + 1;
	} else {
		new_state.valves = new aoc::valve[num_valves];
		new_state.num_valves = num_valves;
	}

	std::copy(valves, valves + num_valves, new_state.valves);
	new_state.valves[valve.get_index()] = valve;
	return new_state;
}

aoc::state aoc::state::add_valve(const std::string name,
		const uint8_t flow_rate, const bool open, const uint64_t opened,
		std::vector<size_t> &connections) const {
	state new_state(this->flow_rate, time, released, NULL, 0, positions,
			targets, num_agents);
	new_state.valves = new valve[num_valves + 1];
	new_state.num_valves = num_valves + 1;
	std::copy(valves, valves + num_valves, new_state.valves);
	new_state.valves[num_valves] = valve(new_state.num_valves, name, flow_rate,
			open, opened, connections.data(), connections.size());
	return new_state;
}

bool aoc::state::valve_open(const size_t index) const {
	if (index >= num_valves) {
		throw std::out_of_range("Valve index out of range.");
	}

	return valves[index].is_open();
}

aoc::state aoc::state::open_valve(const size_t index) const {
	if (index >= num_valves) {
		throw std::out_of_range("Valve index out of range.");
	}

	state new_state(*this);
	if (!valves[index].is_open()) {
		new_state.valves[index] = new_state.valves[index].open(time);
		new_state.flow_rate += valves[index].get_flow_rate();
	}
	return new_state;
}

uint8_t aoc::state::get_valve_flow_rate(const size_t index) const {
	if (index >= num_valves) {
		throw std::out_of_range("Valve index out of range.");
	}

	return valves[index].get_flow_rate();
}

const std::vector<size_t> aoc::state::get_valve_connections(
		const size_t index) const {
	if (index >= num_valves) {
		throw std::out_of_range("Valve index out of range.");
	}

	return valves[index].get_connections();
}

size_t aoc::state::get_distance(const size_t from, const size_t to) const {
	if (from >= num_valves) {
		throw std::out_of_range("Start index out of range.");
	}
	if (to >= num_valves) {
		throw std::out_of_range("End index out of range.");
	}

	if (from == to) {
		return 0;
	}

	std::map<size_t, size_t> known_distances;
	known_distances[from] = 0;
	std::vector<size_t> stack;
	stack.push_back(from);
	while (stack.size() > 0) {
		const size_t pos = stack.back();
		stack.pop_back();
		const size_t distance = known_distances[pos];
		if (known_distances.count(to) == 1 && distance >= known_distances[to]) {
			continue;
		}

		for (const size_t *connected = valves[pos].get_connections_ptr();
				connected
						< valves[pos].get_connections_ptr()
								+ valves[pos].get_connections_count();
				connected++) {
			if (known_distances.count(*connected) == 0
					|| distance + 1 < known_distances[*connected]) {
				known_distances[*connected] = distance + 1;
				stack.push_back(*connected);
			}
		}
	}

	return known_distances[to];
}

size_t aoc::state::get_distance_to_target(const size_t agent) const {
	if (agent >= num_agents) {
		throw std::out_of_range("Agent index out of range.");
	}

	return get_distance(positions[agent], targets[agent]);
}

std::ostream& aoc::operator<<(std::ostream &stream, const aoc::state &state) {
	stream << "state[total flow rate=" << state.flow_rate;
	stream << ", time=" << state.time;
	stream << ", released=" << state.released;
	stream << ", valves=[";
	for (size_t i = 0; i < state.num_valves; i++) {
		stream << state.valves[i];
		if (i < state.num_valves - 1) {
			stream << ", ";
		}
	}
	stream << "], agents=[";
	for (size_t i = 0; i < state.num_agents; i++) {
		stream << "agent[index=";
		stream << i;
		stream << ", position=";
		stream << state.positions[i];
		stream << ", target=";
		stream << state.targets[i];
		if (i < state.num_agents - 1) {
			stream << "], ";
		} else {
			stream << ']';
		}
	}
	stream << "]]";
	return stream;
}

void aoc::swap(state &first, state &second) noexcept {
	using std::swap;

	swap(first.flow_rate, second.flow_rate);
	swap(first.time, second.time);
	swap(first.released, second.released);
	swap(first.valves, second.valves);
	swap(first.num_valves, second.num_valves);
	swap(first.positions, second.positions);
	swap(first.targets, second.targets);
	swap(first.num_agents, second.num_agents);
}

uint64_t aoc::get_max_released(std::vector<valve> valves, size_t start_pos,
		size_t time, size_t num_agents) {
	size_t *initial_positions = new size_t[num_agents];
	for (size_t i = 0; i < num_agents; i++) {
		initial_positions[i] = start_pos;
	}

	state initial_state(0, 0, 0, valves.data(), valves.size(),
			initial_positions, initial_positions, num_agents);

	std::vector<state> stack;
	stack.push_back(initial_state);
	state max_released;
	while (stack.size() > 0) {
		const state current_state = stack.back();
		stack.pop_back();
		std::vector<size_t> closed_valves;
		for (size_t i = 0; i < current_state.get_size(); i++) {
			if (!current_state.valve_open(i)
					&& current_state.get_valve_flow_rate(i) > 0) {
				closed_valves.push_back(i);
			}
		}

		if (closed_valves.size() == 0) {
			state new_state(current_state);
			if (time - current_state.get_time() > 0) {
				new_state = new_state.add_time(time - current_state.get_time());
			}

			if (max_released.get_released() < new_state.get_released()) {
				max_released = new_state;
			}
			continue;
		}

		for (const size_t val : closed_valves) {
			state new_state = current_state.set_target(0, val);
			const size_t dist = new_state.get_distance_to_target(0);
			if (current_state.get_time() < time) {
				new_state = new_state.add_time(
						std::min(dist + 1, time - current_state.get_time()));
				if (new_state.get_time() == time) {
					if (max_released.get_released()
							< new_state.get_released()) {
						max_released = new_state;
					}
				}
			}

			new_state = new_state.open_valve(val);
			new_state = new_state.set_position(0, val);
			if (new_state.get_time() == time) {
				if (max_released.get_released() < new_state.get_released()) {
					max_released = new_state;
				}
			} else if (new_state.get_time() < time) {
				stack.push_back(std::move(new_state));
			}
		}
	}

	return max_released.get_released();
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
			start_valves[i] = start_valves[i].add_connection(
					name_to_idx[connection]);
		}
	}

	return std::to_string(
			aoc::get_max_released(start_valves, name_to_idx["AA"], 30, 1));
}

bool d16p1 = aoc::registerPart1(16, &day16part1);
