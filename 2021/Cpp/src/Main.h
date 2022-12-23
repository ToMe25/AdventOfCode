/*
 * Main.h
 *
 *  Created on: 02.12.2021
 *      Author: ToMe25
 */

#ifndef MAIN_H_
#define MAIN_H_

#include <fstream>
#include <string>
#include <utility>

/**
 * The main method that gets called when the program is executed.
 *
 * @param argc	The number of arguments given to the program.
 * @param argv	The arguments given to the programe.
 * @return	The programe exit code.
 */
int main(int argc, char *argv[]);

/*
 * Gets a file input stream for the input file for the given day.
 * Ends the program if the file can't be found.
 *
 * @param day	The day for which to get the input stream.
 * @return	A ifstream for the input file for the given day.
 */
std::ifstream getInputFileStream(const uint8_t day);

/**
 * Prints the help text for this program to the system output.
 *
 * @param filename	The current execution path of this process.
 */
void printUsage(const char *filename);

class AoCRunner {
public:
	virtual ~AoCRunner() {
	}

	/*
	 * Solves the task for the given day.
	 * Runs both part 1 and 2 and prints the result to the system output stream.
	 */
	virtual void solve() = 0;
};

template<uint8_t Day>
class DayRunner: public AoCRunner {
public:
	virtual ~DayRunner() {
	}

	/*
	 * Returns the day this implementation is the solution for.
	 */
	uint8_t getDay() {
		return Day;
	}

	void solve() override {
		solve(getInputFileStream(Day));
	}

	/*
	 * Solves the task for the given day.
	 * Runs both part 1 and 2 and prints the result to the system output stream.
	 *
	 * @param input	An ifstream to the input file for the current day.
	 */
	void solve(std::ifstream input);
};

/*
 * Gets a pointer to the DayRunner for the given day.
 *
 * @tparam Days	An integer_sequence containing all the currently implemented days.
 * @param day	The day for which to get the runner.
 * @return	A pointer to the DayRunner for the given day.
 */
template<uint8_t ... Days>
AoCRunner* getRunner(const uint8_t day,
		const std::integer_sequence<uint8_t, Days...>);

#endif /* MAIN_H_ */
