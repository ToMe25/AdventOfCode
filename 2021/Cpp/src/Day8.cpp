/*
 * Day8.cpp
 *
 *  Created on: 08.12.2021
 *      Author: ToMe25
 */

#include "Main.h"
#include <algorithm>
#include <cmath>
#include <iostream>
#include <map>
#include <sstream>
#include <set>
#include <vector>

template <>
void DayRunner<8>::solve(std::ifstream input) {

	std::set<char> number_to_chars[10];
	number_to_chars[0] = { 'a', 'b', 'c', 'e', 'f', 'g' };
	number_to_chars[1] = { 'c', 'f' };
	number_to_chars[2] = { 'a', 'c', 'd', 'e', 'g' };
	number_to_chars[3] = { 'a', 'c', 'd', 'f', 'g' };
	number_to_chars[4] = { 'b', 'c', 'd', 'f' };
	number_to_chars[5] = { 'a', 'b', 'd', 'f', 'g' };
	number_to_chars[6] = { 'a', 'b', 'd', 'f', 'e', 'g' };
	number_to_chars[7] = { 'a', 'c', 'f' };
	number_to_chars[8] = { 'a', 'b', 'c', 'd', 'e', 'f', 'g' };
	number_to_chars[9] = { 'a', 'b', 'c', 'd', 'f', 'g' };

	std::set<uint8_t> char_to_numbers[7];
	for (uint8_t i = 0; i < 10; i++) {
		for (char c : number_to_chars[i]) {
			char_to_numbers[c - 'a'].insert(i);
		}
	}

	std::map<uint8_t, std::vector<char>> occurrences_to_possible_chars;
	for (uint8_t i = 0; i < 7; i++) {
		uint8_t occurrences = 0;
		for (uint8_t j = 0; j < 10; j++) {
			if (number_to_chars[j].find('a' + i) != number_to_chars[j].end()) {
				occurrences++;
			}
		}

		occurrences_to_possible_chars[occurrences].push_back('a' + i);
	}

	uint8_t ones = 0;
	uint8_t fours = 0;
	uint8_t sevens = 0;
	uint8_t eights = 0;

	std::string line;
	std::istringstream stream;
	std::string token;
	bool output = false;
	std::string out_str;
	uint64_t out = 0;
	while(getline(input, line)) {
		std::map<std::string, std::vector<uint8_t>> token_to_possible_numbers;
		std::vector<char> char_to_possible_chars[7];

		stream.clear();
		stream.str(line);
		while (stream >> token) {
			std::sort(token.begin(), token.end());
			if (token == "|") {
				output = true;
				bool solved = false;
				// Set char_to_possible_chars for each character to those characters with the same number of occurrences.
				for (uint8_t i = 0; i < 7; i++) {
					uint8_t occurrences = 0;
					for (std::pair<std::string, std::vector<uint8_t>> entry : token_to_possible_numbers) {
						if (entry.first.find('a' + i) != std::string::npos) {
							occurrences++;
						}
					}

					char_to_possible_chars[i] = occurrences_to_possible_chars[occurrences];
				}

				while (!solved) {
					// Make sure char_to_possible_chars only contains characters that are used
					// for a possible number for the current token.
					for (std::pair<std::string, std::vector<uint8_t>> entry : token_to_possible_numbers) {
						std::set<char> possible_chars;
						for (uint8_t nr : token_to_possible_numbers[entry.first]) {
							possible_chars.insert(number_to_chars[nr].begin(),
									number_to_chars[nr].end());
						}

						for (char c : entry.first) {
							std::vector<char>::iterator it =
									std::set_intersection(
											char_to_possible_chars[c - 'a'].begin(),
											char_to_possible_chars[c - 'a'].end(),
											possible_chars.begin(),
											possible_chars.end(),
											char_to_possible_chars[c - 'a'].begin());
							char_to_possible_chars[c - 'a'].resize(
									it - char_to_possible_chars[c - 'a'].begin());
						}
					}

					// If a character can only possibly represent one character, remove numbers that don't contain
					// this character from all tokens containing the character.
					for (uint8_t i = 0; i < 7; i++) {
						if (char_to_possible_chars[i].size() == 1) {
							uint8_t c = *char_to_possible_chars[i].begin() - 'a';
							for (std::pair<std::string, std::vector<uint8_t>> entry : token_to_possible_numbers) {
								if (entry.first.find('a' + i) != std::string::npos) {
									std::vector<uint8_t>::iterator it =
											std::set_intersection(
													entry.second.begin(),
													entry.second.end(),
													char_to_numbers[c].begin(),
													char_to_numbers[c].end(),
													token_to_possible_numbers[entry.first].begin());
									token_to_possible_numbers[entry.first].resize(
											it - token_to_possible_numbers[entry.first].begin());
								}
							}
						}
					}

					solved = true;
					for (std::pair<std::string, std::vector<uint8_t>> entry : token_to_possible_numbers) {
						if (entry.second.size() > 1) {
							solved = false;
							break;
						}
					}
				}
			} else if (output) {
				uint16_t out_nr = token_to_possible_numbers[token][0];
				if (out_nr == 1) {
					ones++;
				} else if (out_nr == 4) {
					fours++;
				} else if (out_nr == 7) {
					sevens++;
				} else if (out_nr == 8) {
					eights++;
				}

				out_str += std::to_string(out_nr);
				if (out_str.length() == 4) {
					out += std::stoi(out_str);
					out_str = "";
				}
			} else {
				if (token.length() == 2) {
					token_to_possible_numbers[token] = { 1 };
				} else if (token.length() == 3) {
					token_to_possible_numbers[token] = { 7 };
				} else if (token.length() == 4) {
					token_to_possible_numbers[token] = { 4 };
				} else if (token.length() == 5) {
					token_to_possible_numbers[token] = { 2, 3, 5 };
				} else if (token.length() == 6) {
					token_to_possible_numbers[token] = { 0, 6, 9 };
				} else if (token.length() == 7) {
					token_to_possible_numbers[token] = { 8 };
				}
			}
		}
		output = false;
	}

	std::cout << "The part 1 result is " << (ones + fours + sevens + eights) << '.' << std::endl;
	std::cout << "The part 2 result is " << out << '.' << std::endl;
}
