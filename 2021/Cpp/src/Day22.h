/*
 * Day22.h
 *
 *  Created on: 22.12.2021
 *      Author: ToMe25
 */

#ifndef DAY22_H_
#define DAY22_H_

#include "Main.h"
#include <vector>

struct Region {
	int32_t min_x, max_x, min_y, max_y, min_z, max_z;
	uint64_t size;

	/**
	 * Creates a new region of 0 size at 0, 0, 0.
	 */
	Region() :
			min_x(0), max_x(0), min_y(0), max_y(0), min_z(0), max_z(0), size(0) {
	}

	/**
	 * Creates a new region with the given min and max positions.
	 *
	 * @param min_x	The min x coordinate for this region.
	 * @param max_x	The max x coordinate for this region.
	 * @param min_y	The min y coordinate for this region.
	 * @param max_y	The max y coordinate for this region.
	 * @param min_z	The min z coordinate for this region.
	 * @param max_z	The max z coordinate for this region.
	 */
	Region(const uint32_t min_x, const uint32_t max_x, const uint32_t min_y,
			const uint32_t max_y, const uint32_t min_z, const uint32_t max_z) :
			min_x(min_x), max_x(max_x), min_y(min_y), max_y(max_y), min_z(
					min_z), max_z(max_z), size(calculate_size()) {
	}

	/**
	 * Creates a new region with the given min and max positions.
	 *
	 * @param coords	The min and max coords for this region in the following order:
	 * 					min_x, max_x, min_y, max_y, min_z, max_z.
	 */
	Region(const int32_t coords[6]) :
			min_x(coords[0]), max_x(coords[1]), min_y(coords[2]), max_y(
					coords[3]), min_z(coords[4]), max_z(coords[5]), size(
					calculate_size()) {
	}

	/**
	 * Destroys this region.
	 */
	virtual ~Region() {
	}

	/**
	 * Checks whether this region is empty.
	 *
	 * @return	True if this region is empty.
	 */
	bool empty() const {
		return size == 0;
	}

	/**
	 * Checks whether the two regions intersect.
	 *
	 * @param other	The other region with which to check for intersections.
	 * @return	True if the two regions intersect.
	 */
	bool intersects(const Region &other) const;

	/**
	 * Gets a region representing the intersection between this region and another region.
	 *
	 * @param other	The other region with which to calculate the intersection.
	 * @return	A new region representing the intersection between two regions.
	 */
	Region intersection(const Region &other) const;

	/**
	 * Calculates a vector of regions that represents the remains of this region
	 * after cutting of the intersection between the two regions.
	 *
	 * @param other	The region to cut off.
	 * @return	A vector containing the remaining regions.
	 */
	std::vector<Region> outside_intersection(const Region &other) const;

	/**
	 * Checks whether the two regions are equal.
	 *
	 * @param other	The other region to compare this one with.
	 * @return	True if the two regions are equal.
	 */
	bool operator ==(const Region &other) const;

	/**
	 * Checks whether the two regions are not equal.
	 *
	 * @param other	The other region to compare this one with.
	 * @return	True if the two regions are not equal.
	 */
	bool operator !=(const Region &other) const {
		return !operator ==(other);
	}

private:
	/**
	 * Calculates the size(volume) of this region.
	 *
	 * @return	This regions volume.
	 */
	uint64_t calculate_size() const;
};

/**
 * Writes a string representation of the given region to the given output stream.
 *
 * @param stream	The ostream to write to.
 * @param region	The region to write to the output stream.
 * @return	The given ostream.
 */
std::ostream& operator <<(std::ostream &stream, const Region &region);

#endif /* DAY22_H_ */
