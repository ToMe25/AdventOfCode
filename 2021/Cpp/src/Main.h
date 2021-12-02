/*
 * Main.h
 *
 *  Created on: 02.12.2021
 *      Author: ToMe25
 */

#ifndef MAIN_H_
#define MAIN_H_

#include <fstream>

int main(int argc, char* argv[]);

/*
 * Gets a file input stream for the input file for the given day.
 * Ends the program if the file can't be found.
 */
std::ifstream getInputFileStream(uint8_t day);

class AoCRunner {
public:
	AoCRunner() { }
	virtual ~AoCRunner() { }

	/*
	 * Solves the task for the given day.
	 * Runs both part 1 and 2 and prints the result to the system output stream.
	 */
	virtual void solve() { exit(3); }
};

#endif /* MAIN_H_ */
