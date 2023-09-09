/*
 * Day16.h
 *
 *  Created on: Dec 16, 2022
 *      Author: ToMe25
 */

#ifndef DAY16_H_
#define DAY16_H_

#include "Main.h"

namespace aoc {
/**
 * Represents the state and connections of a single valve.
 * Not internally const to allow assignment, but basically immutable.
 */
class valve {
private:
	/**
	 * The index of this valve.
	 */
	size_t index;

	/**
	 * The name of this valve.
	 */
	std::string name;

	/**
	 * The flow rate this valve allows when opened.
	 */
	uint8_t flow_rate;

	/**
	 * This valves current state. True if the valve is open, false if it is closed.
	 */
	bool state;

	/**
	 * The time at which this valve was opened.
	 * Only applicable if this valve is open.
	 */
	uint64_t opened;

	/**
	 * The indices of connected valves.
	 */
	std::vector<size_t> connections;

public:
	/**
	 * Creates a new valve object representing the given valve state.
	 *
	 * @param index			The index of this valve.
	 * @param name			The name given to this valve.
	 * @param flow_rate		The pressure reduction this valve allows when it is open.
	 * @param open			Whether or nor the valve is open.
	 * @param opened		The time at which this valve was opened. Irrelevant if the valve is closed.
	 * @param connections	A vector containing the indices of the valves this valve is connected to.
	 */
	valve(const size_t index, const std::string name, const uint8_t flow_rate,
			const bool open = false, const uint64_t opened = 0,
			std::vector<size_t> connections = std::vector<size_t>());

	/**
	 * A copy constructor.
	 *
	 * @param valve	The valve to copy the values from.
	 */
	valve(const valve &valve);

	/**
	 * Copy assignment operator.
	 *
	 * @param valve	The valve to set this valve to.
	 * @return	The modified valve.
	 */
	valve& operator=(const valve &valve);

	/**
	 * Gets the index of this valve.
	 *
	 * @return	This valves index.
	 */
	size_t get_index() const;

	/**
	 * Retrieves this valves name.
	 *
	 * @return	The name given to this valve.
	 */
	const std::string get_name() const;

	/**
	 * Checks whether this valve is currently open.
	 *
	 * @return	True if this valve is open, false otherwise.
	 */
	bool is_open() const;

	/**
	 * Gets the time at which the valve was opened.
	 *
	 * @return	The opening time of this valve.
	 */
	uint64_t get_opened() const;

	/**
	 * Creates a copy of this valve, that is open.
	 *
	 * @param time	The current time at which the valve is being opened.
	 * @return	The new valve objects.
	 */
	valve open(const uint64_t time) const;

	/**
	 * Gets the flow rate this valve allows when opened.
	 *
	 * @return	The max flow rate of this valve.
	 */
	uint8_t get_flow_rate() const;

	/**
	 * Get the indices of all the valves this valve is connected to.
	 *
	 * @return	The vector containing the valves this valve is connected to.
	 */
	const std::vector<size_t>& get_connections() const;

	/**
	 * Creates a copy of this valve with an added connection;
	 * Only adds the connection if it does not yet exist.
	 *
	 * @param index	The index of the valve to add a new connection to.
	 * @return	The new valve object.
	 */
	valve add_connection(const size_t index) const;

	/**
	 * Writes the given valve to the given output stream.
	 *
	 * @param stream	The stream to write the string representation of this valve to.
	 * @param valve		The valve to write to the given output stream.
	 * @return	The stream the valve was written to.
	 */
	friend std::ostream& operator<<(std::ostream &stream, const valve &valve);
};

/**
 * Writes the given valve to the given output stream.
 *
 * @param stream	The stream to write the string representation of this valve to.
 * @param valve		The valve to write to the given output stream.
 * @return	The stream the valve was written to.
 */
std::ostream& operator<<(std::ostream &stream, const valve &valve);

/**
 * An object representing the current state of a set of valves and connections.
 */
class state {
private:
	/**
	 * A vector containing all the valves to be tracked.
	 */
	std::vector<valve> valves;

	/**
	 * The current rate at which pressure is released.
	 */
	uint64_t flow_rate;

	/**
	 * The amount of time that passed since the beginning.
	 */
	uint64_t time;

	/**
	 * The amount by which the pressure has reduced since the beginning.
	 */
	uint64_t released;

	/**
	 * The index of the valve at which the controller is right now.
	 */
	size_t position;

public:
	/**
	 * Creates a new state object with the given properties
	 *
	 * @param valves	The vector of valves to use to store the valve states.
	 * @param flow_rate	The rate at which pressure is currently released.
	 * @param time		The current point in time represented by this state.
	 * @param released	The amount of pressure released so far.
	 * @param position	The current position of the person controlling the valves.
	 */
	state(std::vector<valve> valves, const uint64_t flow_rate = 0,
			const uint64_t time = 0, const uint64_t released = 0,
			const size_t position = 0);

	/**
	 * A copy constructor.
	 *
	 * @param state	The state to copy.
	 */
	state(const state &state);

	/**
	 * A copy assignment operator.
	 *
	 * @param state	The state to set this state to.
	 * @return the modified state.
	 */
	state& operator=(const state &state);

	/**
	 * Creates a new state that represents this state when one unit of time passed.
	 * Updates released pressure.
	 *
	 * @return	The updated state.
	 */
	state update() const;

	/**
	 * Gets the current rate at which pressure is released.
	 *
	 * @return	The current total flow rate.
	 */
	uint64_t get_flow_rate() const;

	/**
	 * Creates a new state with the given flow rate.
	 *
	 * @param flow_rate	The flow rate to use for the new copy of this state.
	 * @return	The newly created state.
	 */
	state set_flow_rate(const uint64_t flow_rate) const;

	/**
	 * Creates a new state, with this states flow rate plus the given flow rate.
	 *
	 * @param flow_rate	The flow rate to add.
	 * @return	The new state object.
	 */
	state add_flow_rate(const uint64_t flow_rate) const;

	/**
	 * Returns the amount of time that passed since the first state was created.
	 *
	 * @return	The amount of time that passed.
	 */
	uint64_t get_time() const;

	/**
	 * Gets the total amount of released pressure.
	 *
	 * @return	The released pressure.
	 */
	uint64_t get_released() const;

	/**
	 * Gets the index of the valve at which the person controlling the valves is currently at.
	 *
	 * @return	The current position of the person controlling the valves.
	 */
	size_t get_position() const;

	/**
	 * Creates a new state object in which the position of the controller is the given position.
	 *
	 * @param position	The new position to use.
	 * @return	The new state object with the changed position.
	 */
	state set_position(const size_t position) const;

	/**
	 * Gets the number of valves stored in this state.
	 *
	 * @return	The number of tracked valve states.
	 */
	size_t get_size() const;

	/**
	 * Returns the vector containing the valves this state monitors.
	 *
	 * @return	The vector containing the valve objects.
	 */
	const std::vector<valve> get_valves() const;

	/**
	 * Get the valve at the given index.
	 *
	 * @param index	The index of the valve to get.
	 * @return	The valve at the given index.
	 */
	const valve& get_valve(const size_t index) const;

	/**
	 * Creates a new state with the valve at the index of the given valve is replaced with the given valve.
	 *
	 * @param valve	The new valve to use.
	 * @return	The new modified state.
	 */
	state set_valve(valve &valve) const;

	/**
	 * Creates a new state with a new valve with the given properties added.
	 *
	 * @param name			The name of the new state to add.
	 * @param flow_rate		The flow rate the new valve allows when it is open.
	 * @param open			Whether the state to add is open already.
	 * @param opened		The timem at which the given valve was opened.
	 * @param connections	The indices of the valves the new valve is connected to.
	 * @return	The newly created state.
	 */
	state add_valve(const std::string name, const uint8_t flow_rate,
			const bool open, const uint64_t opened, std::vector<size_t> &connections) const;

	/**
	 * Checks whether the valve at the given index is open.
	 *
	 * @param idx	The index of the valve to check.
	 * @return	True if the valve is open.
	 */
	bool valve_open(const size_t idx) const;

	/**
	 * Creates a copy of this state with the valve at the given index opened.
	 *
	 * @param idx	The index of the valve to open.
	 * @return	The new modified state,
	 */
	state open_valve(const size_t idx) const;

	/**
	 * Checks the flow rate of the valve at the given index.
	 *
	 * @param idx	The index of the valve to check.
	 * @return	The flow rate the given valve allows when it is open.
	 */
	uint8_t get_valve_flow_rate(const size_t idx) const;

	/**
	 * Gets the indices of the valves the valve at the given index is connected to.
	 *
	 * @param idx	The index of the valve to check.
	 * @return The connections vector of the given valve.
	 */
	const std::vector<size_t>& get_valve_connections(const size_t idx) const;

	/**
	 * Calculates the distance from the current position to the given target position.
	 *
	 * @param target	The index of the valve to move to.
	 * @return	The distance from the current position to the given target position.
	 */
	size_t get_distance_to(const size_t target) const;

	/**
	 * Writes the given state to the given output stream.
	 *
	 * @param stream	The stream to write to state to.
	 * @param state		The state to write to the given output stream.
	 * @return	The modified stream object.
	 */
	friend std::ostream& operator<<(std::ostream &stream, const aoc::state &state);
};

/**
 * Writes the given state to the given output stream.
 *
 * @param stream	The stream to write to state to.
 * @param state		The state to write to the given output stream.
 * @return	The modified stream object.
 */
std::ostream& operator<<(std::ostream &stream, const aoc::state &state);
}

#endif /* DAY16_H_ */