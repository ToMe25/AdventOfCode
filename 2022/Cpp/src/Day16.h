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
	std::string_view name;

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
	size_t *connections;

	/**
	 * The number of connected valves.
	 */
	size_t num_connections;

public:
	/**
	 * Creates a new empty valve object with the index 0.
	 */
	valve() noexcept;

	/**
	 * Creates a new valve object representing the given valve state.
	 *
	 * @param index				The index of this valve.
	 * @param name				The name given to this valve.
	 * @param flow_rate			The pressure reduction this valve allows when it is open.
	 * @param open				Whether or nor the valve is open.
	 * @param opened			The time at which this valve was opened. Irrelevant if the valve is closed.
	 * @param connections		A pointer to an array of the indices of the connected valves. May be NULL. Will be copied.
	 * @param num_connectsions	The number of connected valves, aka the size of the connections array.
	 */
	valve(const size_t index, const std::string_view name,
			const uint8_t flow_rate, const bool open = false,
			const uint64_t opened = 0, const size_t *connections = NULL,
			const size_t num_connections = 0);

	/**
	 * A copy constructor.
	 *
	 * @param valve	The valve to copy the values from.
	 */
	valve(const valve &valve);

	/**
	 * A move constructor.
	 *
	 * @param valve	The valve to move the values from.
	 */
	valve(valve &&valve) noexcept;

	/**
	 * A destructor, deleting the connections array.
	 */
	virtual ~valve();

	/**
	 * An assignment operator.
	 *
	 * @param valve	The valve to set this valve to.
	 * @return	The modified valve.
	 */
	valve& operator=(valve valve) noexcept;

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
	const std::string_view& get_name() const;

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
	 * @return	A vector containing the valves this valve is connected to.
	 */
	const std::vector<size_t> get_connections() const;

	/**
	 * Returns a pointer to the underlying connections array.
	 *
	 * @return	A pointer to the underlying array of connections.
	 */
	const size_t* get_connections_ptr() const;

	/**
	 * Get the number of valves this valve is connected to.
	 *
	 * @return	The number of valves this valve is connected to.
	 */
	size_t get_connections_count() const;

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

	/**
	 * Swaps the given two valve instances.
	 *
	 * @param first		The first of the two valves to swap.
	 * @param second	The second of the two values to swap.
	 */
	friend void swap(valve &first, valve &second) noexcept;
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
 * Swaps the given two valve instances.
 *
 * @param first		The first of the two valves to swap.
 * @param second	The second of the two values to swap.
 */
void swap(valve &first, valve &second) noexcept;

/**
 * An object representing the current state of a set of valves and connections.
 * Not internally const to allow assignment, but basically immutable.
 */
class state {
private:
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
	 * All the valves to be tracked.
	 */
	valve *valves;

	/**
	 * The number of valves to be tracked.
	 */
	size_t num_valves;

	/**
	 * The positions of the agents changing the valve states.
	 */
	size_t *positions;

	/**
	 * The target positions of the agents changing the valve states.
	 */
	size_t *targets;

	/**
	 * The number of agents capable of changing the valve states.
	 * The size of positions and targets.
	 */
	size_t num_agents;

public:
	/**
	 * Creates a new state object with the given properties
	 *
	 * @param flow_rate		The rate at which pressure is currently released.
	 * @param time			The current point in time represented by this state.
	 * @param released		The amount of pressure released so far.
	 * @param valves		An array of valves to track. May be NULL. Will be copied.
	 * @param num_valves	The number of valves to track.
	 * @param positions		An array of agent positions. May be NULL. Will be copied.
	 * @param targets		An array of agent target positions. May be NULL. Will be copied.
	 * @param num_agents	The size of both positions and targets.
	 */
	state(const uint64_t flow_rate = 0, const uint64_t time = 0,
			const uint64_t released = 0, const valve *valves = NULL,
			const size_t num_valves = 0, const size_t *positions = NULL,
			const size_t *targets = NULL, const size_t num_agents = 0);

	/**
	 * A copy constructor.
	 *
	 * @param state	The state to copy.
	 */
	state(const state &state);

	/**
	 * A move constructor.
	 *
	 * @param state	The state to move.
	 */
	state(state &&state) noexcept;

	/**
	 * A destructor deleting the underlying valves array.
	 */
	virtual ~state();

	/**
	 * An assignment operator.
	 *
	 * @param state	The state to set this state to.
	 * @return the modified state.
	 */
	state& operator=(state state) noexcept;

	/**
	 * Creates a new state that represents this state when one unit of time passed.
	 * Updates released pressure.
	 *
	 * @return	The updated state.
	 */
	state update() const;

	/**
	 * Creates a new state that represents this state after a given amount of time passed.
	 * Updates released pressure.
	 *
	 * @return	The updated state.
	 */
	state add_time(const uint64_t time) const;

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
	 * @param agent	The index of the agent whose position to get.
	 * @return	The current position of the person controlling the valves.
	 * @throws std::out_of_range	If agent is above the limit of the positions array.
	 */
	size_t get_position(const size_t agent) const;

	/**
	 * Creates a new state object in which the position of the controller is the given position.
	 *
	 * @param agent		The index of the agent whose position to set.
	 * @param position	The new position to use.
	 * @return	The new state object with the changed position.
	 * @throws std::out_of_range	If agent is above the limit of the positions array,
	 * 								or position is above the limit of the valves array.
	 */
	state set_position(const size_t agent, const size_t position) const;

	/**
	 * Gets the target position of the given agent.
	 *
	 * @param agent	The index of the agent whose target position to get.
	 * @return	The target position of the given agent.
	 * @throws std::out_of_range	If agent is above the limit of the targets array.
	 */
	size_t get_target(const size_t agent) const;

	/**
	 * Sets the target position of the given agent.
	 *
	 * @param agent	The index of the agent whose target position to set.
	 * @param position	The target position for the agent.
	 * @return	The new state object with the changed target position.
	 * @throws std::out_of_range	If agent is above the limit of the targets array.
	 * 								or position is above the limit of the valves array.
	 */
	state set_target(const size_t agent, const size_t position) const;

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
	 * @throws std::out_of_range	If index is greater than the biggest existing valve index.
	 */
	const valve& get_valve(const size_t index) const;

	/**
	 * Creates a new state with the valve at the index of the given valve is replaced with the given valve.
	 * If valve is equals to the size of the valves array, it is added to the and instead.
	 *
	 * @param valve	The new valve to use.
	 * @return	The new modified state.
	 * @throws std::out_of_range	If the index of valve is more than the size of the valves array.
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
			const bool open, const uint64_t opened,
			std::vector<size_t> &connections) const;

	/**
	 * Checks whether the valve at the given index is open.
	 *
	 * @param index	The index of the valve to check.
	 * @return	True if the valve is open.
	 * @throws std::out_of_range	If index is outside the valves array.
	 */
	bool valve_open(const size_t index) const;

	/**
	 * Creates a copy of this state with the valve at the given index opened.
	 * Returns an unmodified copy if the valve is already open.
	 *
	 * @param index	The index of the valve to open.
	 * @return	The new modified state.
	 * @throws std::out_of_range	If index is outside the valves array.
	 */
	state open_valve(const size_t index) const;

	/**
	 * Checks the flow rate of the valve at the given index.
	 *
	 * @param index	The index of the valve to check.
	 * @return	The flow rate the given valve allows when it is open.
	 * @throws std::out_of_range	If index is outside the valves array.
	 */
	uint8_t get_valve_flow_rate(const size_t index) const;

	/**
	 * Gets the indices of the valves the valve at the given index is connected to.
	 *
	 * @param index	The index of the valve to check.
	 * @return The connections vector of the given valve.
	 * @throws std::out_of_range	If index is outside the valves array.
	 */
	const std::vector<size_t> get_valve_connections(const size_t index) const;

	/**
	 * Calculates the distance between the two given positions.
	 *
	 * @param from	The position from which to start.
	 * @param to	The position to which to find the distance.
	 * @return	The distance from "from" to "to".
	 * @throws std::out_of_range	If from or to is outside of the valves array.
	 */
	size_t get_distance(const size_t from, const size_t to) const;

	/**
	 * Calculates the distance from the current position of the given agent to its target position.
	 *
	 * @param agent	The index of the agent to calculate the distance for.
	 * @return	The distance from the current position to the given target position.
	 * @throws std::out_of_range	If the agent index is outside the positions array.
	 */
	size_t get_distance_to_target(const size_t agent) const;

	/**
	 * Writes the given state to the given output stream.
	 *
	 * @param stream	The stream to write to state to.
	 * @param state		The state to write to the given output stream.
	 * @return	The modified stream object.
	 */
	friend std::ostream& operator<<(std::ostream &stream,
			const aoc::state &state);

	/**
	 * Swaps the given two state instances.
	 *
	 * @param first		The first of the two states to swap.
	 * @param second	The second of the two states to swap.
	 */
	friend void swap(state &first, state &second) noexcept;
};

/**
 * Writes the given state to the given output stream.
 *
 * @param stream	The stream to write to state to.
 * @param state		The state to write to the given output stream.
 * @return	The modified stream object.
 */
std::ostream& operator<<(std::ostream &stream, const aoc::state &state);

/**
 * Swaps the given two state instances.
 *
 * @param first		The first of the two states to swap.
 * @param second	The second of the two states to swap.
 */
void swap(state &first, state &second) noexcept;

/**
 * Tests the max amount of pressure that can be released in the given amount of time.
 *
 * @param valves		The valve structure to be used.
 * @param start_pos		The starting position of all the agents.
 * @param time			The time for which to test.
 * @param num_agents	The number of agents that can open valves.
 * @return	The max amount of pressure that can be released.
 */
uint64_t get_max_released(std::vector<valve> valves, size_t start_pos,
		size_t time, size_t num_agents);
}

#endif /* DAY16_H_ */
