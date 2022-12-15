/*
 * Day15.h
 *
 *  Created on: Dec 15, 2022
 *      Author: ToMe25
 */

#ifndef DAY15_H_
#define DAY15_H_

#include "Main.h"

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
	sensor(const std::pair<int64_t, int64_t> sensor_pos, const std::pair<int64_t, int64_t> beacon_pos);

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
	 * @return True if the position is within this sensors range.
	 */
	bool convers_position(const int64_t x, const int64_t y) const;

	/**
	 * Creates a vector containing all the X positions on a given Y position that are covered by this sensor.
	 * The resulting vector is ordered incrementing.
	 *
	 * @param y		The y position to check;
	 * @param empty	If true, and the beacon is on that y position, the beacon pos will not be included.
	 * @return	The newly created vector.
	 */
	std::vector<int64_t> covered_on_y(const int64_t y, const bool empty) const;

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
