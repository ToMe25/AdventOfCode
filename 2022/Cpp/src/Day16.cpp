/*
 * Day16.cpp
 *
 *  Created on: Dec 16, 2022
 *      Author: ToMe25
 */

#include "Day16.h"
#include <map>
#include <unordered_set>

aoc::valve::valve() noexcept :
		index(0), name(NULL), name_len(0), flow_rate(0), state(false), opened(
				0), connections(NULL), num_connections(0) {

}

aoc::valve::valve(const size_t index, const std::string_view name,
		const uint8_t flow_rate, const bool open, const uint64_t opened,
		const size_t *connections, const size_t num_connections) :
		index(index), name(
				name.length() > 0 ? new char[name.length() + 1] : NULL), name_len(
				name.length()), flow_rate(flow_rate), state(open), opened(
				opened), connections(
				num_connections > 0 ? new size_t[num_connections] : NULL), num_connections(
				num_connections) {
	if (num_connections > 0) {
		std::copy(connections, connections + num_connections,
				this->connections);
	}
	if (name.length() > 0) {
		name.copy(this->name, name.length());
		this->name[name_len] = 0;
	}
}

aoc::valve::valve(const valve &valve) :
		index(valve.index), name(
				valve.name_len > 0 ? new char[valve.name_len + 1] : NULL), name_len(
				valve.name_len), flow_rate(valve.flow_rate), state(valve.state), opened(
				valve.opened), connections(
				valve.num_connections > 0 ?
						new size_t[valve.num_connections] : NULL), num_connections(
				valve.num_connections) {
	if (num_connections > 0) {
		std::copy(valve.connections, valve.connections + valve.num_connections,
				connections);
	}
	if (name_len > 0) {
		std::copy(valve.name, valve.name + valve.name_len + 1, name);
	}
}

aoc::valve::valve(valve &&valve) noexcept :
		index(0), name(NULL), name_len(0), flow_rate(0), state(false), opened(
				0), connections(NULL), num_connections(0) {
	using std::swap;

	swap(*this, valve);
}

aoc::valve::~valve() {
	delete[] connections;
	delete[] name;
}

aoc::valve& aoc::valve::operator=(valve valve) noexcept {
	using std::swap;

	swap(*this, valve);
	return *this;
}

bool aoc::valve::operator==(const valve &other) const {
	if (index != other.index || flow_rate != other.flow_rate
			|| state != other.state || opened != other.opened
			|| num_connections != other.num_connections) {
		return false;
	}

	if (get_name() != other.get_name()) {
		return false;
	}

	for (size_t i = 0; i < num_connections; i++) {
		if (connections[i] != other.connections[i]) {
			return false;
		}
	}

	return true;
}

bool aoc::valve::operator!=(const valve &other) const {
	return !this->operator ==(other);
}

size_t aoc::valve::get_index() const {
	return index;
}

const std::string_view aoc::valve::get_name() const {
	return std::string_view(name, name_len);
}

bool aoc::valve::is_open() const {
	return state;
}

uint64_t aoc::valve::get_opened() const {
	return opened;
}

aoc::valve aoc::valve::open(const uint64_t time) const {
	return valve(index, get_name(), flow_rate, true, time, connections,
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

	valve new_valve(this->index, get_name(), flow_rate, state, opened, NULL, 0);
	new_valve.connections = new size_t[num_connections + 1];
	std::copy(connections, connections + num_connections,
			new_valve.connections);
	new_valve.connections[num_connections] = index;
	new_valve.num_connections = num_connections + 1;
	return new_valve;
}

std::ostream& aoc::operator<<(std::ostream &stream, const aoc::valve &valve) {
	stream << "valve[index=" << valve.index;
	stream << ", name=" << valve.get_name();
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
	swap(first.name_len, second.name_len);
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
				num_agents), distances(
				num_agents > 0 ? new size_t[num_agents] : NULL), traveled(
				num_agents > 0 ? new size_t[num_agents] { 0 } : NULL) {
	if (num_valves > 0) {
		std::copy(valves, valves + num_valves, this->valves);
	}

	if (num_agents > 0) {
		std::copy(positions, positions + num_agents, this->positions);
		if (targets != NULL) {
			std::copy(targets, targets + num_agents, this->targets);
		} else {
			std::copy(positions, positions + num_agents, this->targets);
		}

		for (size_t i = 0; i < num_agents; i++) {
			distances[i] = get_distance(positions[i], targets[i]);
		}
	}
}

aoc::state::state(const state &state) :
		flow_rate(state.flow_rate), time(state.time), released(state.released), valves(
				state.num_valves > 0 ? new valve[state.num_valves] : NULL), num_valves(
				state.num_valves), positions(
				state.num_agents > 0 ? new size_t[state.num_agents] : NULL), targets(
				state.num_agents > 0 ? new size_t[state.num_agents] : NULL), num_agents(
				state.num_agents), distances(
				state.num_agents > 0 ? new size_t[state.num_agents] : NULL), traveled(
				state.num_agents > 0 ? new size_t[state.num_agents] : NULL) {
	if (num_valves > 0) {
		std::copy(state.valves, state.valves + state.num_valves, valves);
	}

	if (num_agents > 0) {
		std::copy(state.positions, state.positions + state.num_agents,
				positions);
		std::copy(state.targets, state.targets + state.num_agents, targets);
		std::copy(state.distances, state.distances + state.num_agents,
				distances);
		std::copy(state.traveled, state.traveled + state.num_agents, traveled);
	}
}

aoc::state::state(state &&state) noexcept :
		flow_rate(0), time(0), released(0), valves(NULL), num_valves(0), positions(
		NULL), targets(NULL), num_agents(0), distances(NULL), traveled(NULL) {
	using std::swap;

	swap(*this, state);
}

aoc::state::~state() {
	delete[] valves;
	delete[] positions;
	delete[] targets;
	delete[] distances;
	delete[] traveled;
}

aoc::state& aoc::state::operator=(state state) noexcept {
	using std::swap;

	swap(*this, state);
	return *this;
}

bool aoc::state::operator==(const state &other) const {
	if (flow_rate != other.flow_rate || time != other.time
			|| released != other.released || num_valves != other.num_valves
			|| num_agents != other.num_agents) {
		return false;
	}

	for (size_t i = 0; i < num_valves; i++) {
		if (valves[i] != other.valves[i]) {
			return false;
		}
	}

	for (size_t i = 0; i < num_agents; i++) {
		if (positions[i] != other.positions[i] || targets[i] != other.targets[i]
				|| distances[i] != other.distances[i]
				|| traveled[i] != other.traveled[i]) {
			return false;
		}
	}

	return true;
}

bool aoc::state::operator!=(const state &other) const {
	return !this->operator ==(other);
}

aoc::state aoc::state::update() const {
	return add_time(1);
}

aoc::state aoc::state::add_time(const uint64_t time) const {
	state new_state(*this);
	if (time == 0) {
		return new_state;
	}

	new_state.time += time;
	new_state.released += new_state.flow_rate * time;
	for (size_t agent = 0; agent < new_state.num_agents; agent++) {
		const size_t remaining_travel = new_state.distances[agent]
				- new_state.traveled[agent];
		if (time < remaining_travel) {
			new_state.traveled[agent] += time;
		} else {
			const size_t target = new_state.targets[agent];
			if (!new_state.valve_open(target) && remaining_travel < time) {
				const uint64_t open_time = new_state.time + remaining_travel
						- time + 1;
				new_state.valves[target] = new_state.valves[target].open(
						open_time);
				const uint8_t new_valve_flow_rate =
						new_state.valves[target].get_flow_rate();
				new_state.flow_rate += new_valve_flow_rate;
				new_state.released += new_valve_flow_rate
						* (new_state.time - open_time);
			}
			new_state.positions[agent] = target;
			new_state.distances[agent] = new_state.traveled[agent] = 0;
		}
	}
	return new_state;
}

aoc::state aoc::state::run_to_nex_target(const uint64_t max_time) const {
	if (max_time == 0) {
		return state(*this);
	}

	uint64_t time = max_time;
	for (size_t agent = 0; agent < num_agents; agent++) {
		if (positions[agent] == targets[agent]
				&& valves[positions[agent]].is_open()) {
			return state(*this);
		}

		if (distances[agent] - traveled[agent] < time - 1) {
			time = distances[agent] - traveled[agent] + 1;
		}
	}
	return add_time(time);
}

uint64_t aoc::state::get_flow_rate() const {
	return flow_rate;
}

aoc::state aoc::state::set_flow_rate(const uint64_t flow_rate) const {
	state new_state(*this);
	new_state.flow_rate = flow_rate;
	return new_state;
}

aoc::state aoc::state::add_flow_rate(const uint64_t flow_rate) const {
	state new_state(*this);
	new_state.flow_rate += flow_rate;
	return new_state;
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
	new_state.distances[agent] = get_distance(new_state.positions[agent],
			position);
	new_state.traveled[agent] = 0;
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
	new_state.distances[agent] = get_distance(new_state.positions[agent],
			position);
	return new_state;
}

size_t aoc::state::get_agents_count() const {
	return num_agents;
}

size_t aoc::state::get_size() const {
	return get_valves_count();
}

const std::vector<aoc::valve> aoc::state::get_valves() const {
	return std::vector<aoc::valve>(valves, valves + num_valves);
}

size_t aoc::state::get_valves_count() const {
	return num_valves;
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

	state new_state(*this);
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
	state new_state(*this);
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
	if (!new_state.valves[index].is_open()) {
		new_state.valves[index] = new_state.valves[index].open(time);
		new_state.flow_rate += new_state.valves[index].get_flow_rate();
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

	return distances[agent];
}

void aoc::state::print_to(std::ostream &stream, const bool multiline) const {
	if (multiline) {
		stream << "########## STATE ##########" << std::endl;
		stream << "Time: " << time << std::endl;
		stream << "Flow Rate: " << flow_rate << std::endl;
		stream << "Released: " << released << std::endl;
		stream << "AGENTS(" << num_agents << "):" << std::endl;
		for (size_t i = 0; i < num_agents; i++) {
			stream << "\tagent[index=";
			stream << i;
			stream << ", position=";
			stream << valves[positions[i]].get_name() << '(' << positions[i]
					<< ')';
			stream << ", target=";
			stream << valves[targets[i]].get_name() << '(' << targets[i] << ')';
			stream << ", distance=";
			stream << distances[i];
			stream << ", traveled=";
			stream << traveled[i];
			stream << ']' << std::endl;
		}
		stream << "VALVES(" << num_valves << "):" << std::endl;
		for (size_t i = 0; i < num_valves; i++) {
			stream << "\tvalve[index=" << valves[i].get_index();
			stream << ", name=" << valves[i].get_name();
			stream << ", flow rate=" << (uint16_t) valves[i].get_flow_rate();
			stream << ", state=" << (valves[i].is_open() ? "open" : "closed");
			stream << ", opening time=" << valves[i].get_opened();
			stream << ", connections=[";
			for (size_t j = 0; j < valves[i].get_connections_count(); j++) {
				stream << valves[valves[i].get_connections()[j]].get_name()
						<< '(' << valves[i].get_connections()[j] << ')';
				if (j < valves[i].get_connections_count() - 1) {
					stream << ", ";
				}
			}
			stream << "]]" << std::endl;
		}
	} else {
		stream << "state[total flow rate=" << flow_rate;
		stream << ", time=" << time;
		stream << ", released=" << released;
		stream << ", agents=[";
		for (size_t i = 0; i < num_agents; i++) {
			stream << "agent[index=";
			stream << i;
			stream << ", position=";
			stream << positions[i];
			stream << ", target=";
			stream << targets[i];
			stream << ", distance=";
			stream << distances[i];
			stream << ", traveled=";
			stream << traveled[i];
			if (i < num_agents - 1) {
				stream << "], ";
			} else {
				stream << ']';
			}
		}
		stream << "], valves=[";
		for (size_t i = 0; i < num_valves; i++) {
			stream << valves[i];
			if (i < num_valves - 1) {
				stream << ", ";
			}
		}
		stream << "]]";
	}
}

std::ostream& aoc::operator<<(std::ostream &stream, const aoc::state &state) {
	state.print_to(stream);
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
	swap(first.distances, second.distances);
	swap(first.traveled, second.traveled);
}

uint64_t aoc::get_max_released(std::vector<valve> valves, size_t start_pos,
		size_t time, size_t num_agents) {
	using std::hash;

	size_t *initial_positions = new size_t[num_agents];
	for (size_t i = 0; i < num_agents; i++) {
		initial_positions[i] = start_pos;
	}

	std::vector<state> stack;
	stack.push_back(
			state(0, 0, 0, valves.data(), valves.size(), initial_positions,
					initial_positions, num_agents));
	// Only store hashes to avoid needing over 10GB of RAM.
	std::unordered_set<size_t> checked;
	size_t max_released = 0;
	while (stack.size() > 0) {
		const state current_state = stack.back();
		stack.pop_back();
		checked.insert(hash<state>()(current_state));
		std::vector<size_t> closed_valves;
		std::vector<uint64_t> closed_flow_rates;
		for (size_t i = 0; i < current_state.get_size(); i++) {
			if (!current_state.valve_open(i)
					&& current_state.get_valve_flow_rate(i) > 0) {
				closed_valves.push_back(i);
				closed_flow_rates.push_back(
						current_state.get_valve_flow_rate(i));
			}
		}

		if (closed_valves.size() == 0) {
			state new_state(current_state);
			if (time - current_state.get_time() > 0) {
				new_state = new_state.add_time(time - current_state.get_time());
			}

			if (max_released < new_state.get_released()) {
				max_released = new_state.get_released();
			}
			continue;
		}

		// Ignore states that cannot be better than the current best.
		std::sort(closed_flow_rates.rbegin(), closed_flow_rates.rend());
		size_t mrel = current_state.get_released()
				+ current_state.get_flow_rate()
						* (time - current_state.get_time());
		for (size_t i = 0;
				i * 2 + 1 < time - current_state.get_time()
						&& i < closed_flow_rates.size(); i++) {
			mrel += closed_flow_rates[i]
					* (time - current_state.get_time() - i * 2 - 1);
		}
		if (mrel < max_released) {
			continue;
		}

		std::vector<state> new_states;
		new_states.push_back(current_state);
		for (size_t agent = 0; agent < current_state.get_agents_count();
				agent++) {
			const size_t current_pos = current_state.get_position(agent);
			if (current_pos != current_state.get_target(agent)
					|| (!current_state.valve_open(current_pos)
							&& current_state.get_valve_flow_rate(current_pos)
									> 0)) {
				continue;
			}

			for (state st : std::vector<state>(new_states)) {
				new_states.erase(
						std::find(new_states.begin(), new_states.end(), st));
				for (const size_t val : closed_valves) {
					bool found = false;
					for (size_t i = 0; i < st.get_agents_count(); i++) {
						if (st.get_target(i) == val) {
							found = true;
							break;
						}
					}
					if (!found) {
						new_states.push_back(st.set_target(agent, val));
					}
				}
			}
		}

		for (state new_state : new_states) {
			if (new_state.get_time() < time) {
				new_state = new_state.run_to_nex_target(
						time - new_state.get_time());
				if (new_state.get_time() == time
						&& new_state.get_released() > max_released) {
					max_released = new_state.get_released();
				}
			}

			if (new_state.get_time() == time) {
				if (max_released < new_state.get_released()) {
					max_released = new_state.get_released();
				}
			} else if (new_state.get_time() < time
					&& std::count(stack.begin(), stack.end(), new_state) == 0
					&& checked.count(hash<state>()(new_state)) == 0) {
				stack.push_back(std::move(new_state));
			}
		}
	}

	return max_released;
}

size_t std::hash<aoc::valve>::operator()(const aoc::valve &valve) const noexcept {
	size_t result = 1;
	result = result * 31 + hash<size_t>()(valve.index);
	result = result * 31 + hash<string_view>()(valve.get_name());
	result = result * 31 + hash<uint8_t>()(valve.flow_rate);
	result = result * 31 + hash<bool>()(valve.state);
	result = result * 31 + hash<uint64_t>()(valve.opened);
	for (size_t i = 0; i < valve.num_connections; i++) {
		result = result * 31 + hash<size_t>()(valve.connections[i]);
	}
	result = result * 31 + hash<size_t>()(valve.num_connections);
	return result;
}

size_t std::hash<aoc::state>::operator()(const aoc::state &state) const noexcept {
	size_t result = 1;
	result = result * 31 + hash<uint64_t>()(state.flow_rate);
	result = result * 31 + hash<uint64_t>()(state.time);
	result = result * 31 + hash<uint64_t>()(state.released);
	for (size_t i = 0; i < state.num_valves; i++) {
		result = result * 31 + hash<aoc::valve>()(state.valves[i]);
	}
	result = result * 31 + hash<size_t>()(state.num_valves);
	for (size_t i = 0; i < state.num_agents; i++) {
		result = result * 31 + hash<size_t>()(state.positions[i]);
		result = result * 31 + hash<size_t>()(state.targets[i]);
		result = result * 31 + hash<size_t>()(state.distances[i]);
		result = result * 31 + hash<size_t>()(state.traveled[i]);
	}
	result = result * 31 + hash<size_t>()(state.num_agents);
	return result;
}

std::pair<std::string, std::string> day16comb(std::ifstream input) {
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

	uint64_t part1 = aoc::get_max_released(start_valves, name_to_idx["AA"], 30,
			1);
	uint64_t part2 = aoc::get_max_released(start_valves, name_to_idx["AA"], 26,
			2);
	return {std::to_string(part1), std::to_string(part2)};
}

bool d16c = aoc::registerCombined(16, &day16comb);
