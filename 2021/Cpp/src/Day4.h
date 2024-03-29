/*
 * Day4.h
 *
 *  Created on: 04.12.2021
 *      Author: ToMe25
 */

#ifndef DAY4_H_
#define DAY4_H_

#include "Main.h"
#include <vector>

/*
 * Checks if the board contains any horizontal or vertical row of trues.
 *
 * @param board	The board to check.
 * @return	Whether the board contained a full row or column of trues.
 */
bool check_win(const bool board[5][5]);

/*
 * Sets the given number to true on all boards.
 *
 * @param boards			A 3d array about which number is where on the boards.
 * @param values			A 3d array containing which numbers were found already. modified in place.
 * @param number			The number to set to true.
 * @param previous_winners	The boards that already win in the order they won in.
 * @return	The array positions of the boards that won, in the order they won in.
 */
std::vector<uint16_t> set_number(const uint8_t boards[100][5][5],
		bool values[100][5][5], const uint8_t number,
		std::vector<uint16_t> &previous_winners);

#endif /* DAY4_H_ */
