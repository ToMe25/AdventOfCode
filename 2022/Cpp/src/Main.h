/*
 * Main.h
 *
 *  Created on: 30.11.2022
 *      Author: ToMe25
 */

#ifndef MAIN_H_
#define MAIN_H_

#include <functional>
#include <fstream>
#include <iostream>
#include <string>

/**
 * The main method that gets called when the program is executed.
 *
 * @param argc	The number of arguments given to the program.
 * @param argv	The arguments given to the programe.
 * @return	The programe exit code.
 */
int main(int argc, char *argv[]);

namespace aoc {
/*
 * Gets a file input stream for the input file for the given day.
 * Ends the program if the file can't be found.
 *
 * @param day	The day for which to get the input stream.
 * @return	A ifstream for the input file for the given day.
 */
std::ifstream getInputFileStream(const uint8_t day);

/**
 * Reads all lines from the given input stream and returns a vector containing them all.
 *
 * @param input	The file to read the lines from.
 * @return	A vector containing each non-empty line as separate string.
 */
std::vector<std::string> readLines(std::ifstream &input);

/**
 * Prints the help text for this program to the given output stream.
 *
 * @param out	The output stream to print the usage to.
 * @param file	The current execution path of this process.
 */
void printUsage(std::ostream &out, const char *file);

/**
 * Formats the given time in microseconds to the given output stream.
 *
 * @param out		The output stream to print the time to.
 * @param time_us	The time to print to the stream.
 */
void formatTime(std::ostream &out, const uint64_t time_us);

/**
 * A std::function object representing a function that solves a single part of a single day.
 */
typedef std::function<std::string(std::ifstream input)> part_func;

/**
 * A std::function representing a function that solves both parts of a challenge.
 */
typedef std::function<std::pair<std::string, std::string>(std::ifstream input)> combined_func;

/**
 * Registers the given function as a part 1 solution to the challenge for the given day.
 * Prints an error message and terminates the program if a part 1 function is already registered for the given day.
 *
 * @param day	The day to register the function for.
 * @param func	The function to register.
 * @return	Whether the function was successfully registered. Always true since it terminates the program otherwise.
 */
bool registerPart1(const uint8_t day, const part_func &func);

/**
 * Registers the given function as a part 2 solution to the challenge for the given day.
 * Prints an error message and terminates the program if a part 2 function is already registered for the given day.
 *
 * @param day	The day to register the function for.
 * @param func	The function to register.
 * @return	Whether the function was successfully registered. Always true since it terminates the program otherwise.
 */
bool registerPart2(const uint8_t day, const part_func &func);

/**
 * Registers a combined part 1 and two function for the given day.
 * Prints an error message and terminates the program if a part 1 or 2 function
 * is already registered for the given day.
 *
 * @param day	The day to register the function for.
 * @param func	The function to register.
 * @return	Whether the function was successfully registered. Always true since it terminates the program otherwise.
 */
bool registerCombined(const uint8_t day, const combined_func &func);
}

#endif /* MAIN_H_ */
