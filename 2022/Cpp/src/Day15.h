/*
 * Day15.h
 *
 *  Created on: Dec 15, 2022
 *      Author: ToMe25
 */

#ifndef DAY15_H_
#define DAY15_H_

#include "Main.h"
#include <unordered_set>

namespace aoc {
/**
 * An object representing a sensor from the input data.
 */
class sensor {
private:
	/**
	 * The sensors x position.
	 */
	int64_t pos_x;

	/**
	 * The sensors y position.
	 */
	int64_t pos_y;

	/**
	 * The beacon x position.
	 */
	int64_t beacon_x;

	/**
	 * The beacon y position.
	 */
	int64_t beacon_y;

	/**
	 * The distance from the sensor to the beacon.
	 */
	uint64_t distance;

public:
	/**
	 * Creates a new sensor object, representing a sensor at the given position.
	 *
	 * @param sensor_pos	The position of the sensor in 2D space.
	 * @param beacon_pos	The position of the beacon the sensor locked onto.
	 */
	sensor(const std::pair<int64_t, int64_t> sensor_pos,
			const std::pair<int64_t, int64_t> beacon_pos);

	/**
	 * Returns a new pair representing the position of the sensor.
	 *
	 * @return	A pair containing the X and Y position of the sensor.
	 */
	std::pair<int64_t, int64_t> get_sensor_pos() const;

	/**
	 * Creates a new pair representing the position of the beacon this sensor locked onto.
	 *
	 * @return	A pair with the beacons X and Y position.
	 */
	std::pair<int64_t, int64_t> get_beacon_pos() const;

	/**
	 * Gets the manhatten distance from between the sensor and the beacon.
	 * The AoC rules guarantee that there is no beacon in this range other than the one stored in this object.
	 *
	 * @return	The distance to the beacon.
	 */
	uint64_t get_distance() const;

	/**
	 * Checks whether the given position is within the area where there can be no beacon,
	 * other than the one locked onto by the sensor.
	 *
	 * @param x	The X coordinate of the position to check.
	 * @param y The Y coordinate of the position to check.
	 * @return True if the position is within this sensors range.
	 */
	bool covers_position(const int64_t x, const int64_t y) const;

	/**
	 * Adds the x positions this sensor covers on the given y position to the given set.
	 *
	 * @param positions	The set to add the positions to.
	 * @param pos_y		The y position to check;
	 * @param empty		If true, and the beacon is on that y position, the beacon position will not be included.
	 */
	void add_covered_on_y(std::unordered_set<int64_t> &positions, const int64_t pos_y,
			const bool empty) const;

	/**
	 * Calculates the distance between the given position and the sensor.
	 *
	 * @param x	The X position to check against.
	 * @param y The Y position to check against.
	 * @return	The manhatten distance between the sensor and the given position.
	 */
	uint64_t distance_to(const int64_t x, const int64_t y) const;

	/**
	 * Prints the given beacon to the given output stream.
	 *
	 * @param stream	The stream to write to.
	 * @param sensor	The sensor to write to the stream.
	 * @return	The stream that was written to.
	 */
	friend std::ostream& operator<<(std::ostream &stream, const sensor &sensor);
};

/**
 * Prints the given beacon to the given output stream.
 *
 * @param stream	The stream to write to.
 * @param sensor	The sensor to write to the stream.
 * @return	The stream that was written to.
 */
std::ostream& operator<<(std::ostream &stream, const sensor &sensor);
}

#endif /* DAY15_H_ */
