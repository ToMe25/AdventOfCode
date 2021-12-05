#!/bin/bash
# Created on: 03.12.2021
#     Author: ToMe25

source Util.sh

# Counts the number of 1 and 0 bits in the given position, and returns the difference.
# Positive if there are more ones.
function count_bits() {
	local bit=$1

	local count=0
	for line in ${numbers[@]}; do
		if [ $(($line >> (11 - $bit) & 1)) == 1 ]; then
			((count++))
		else
			((count--))
		fi
	done
	echo $count
}

# Finds the number that matches the given criteria out of the given list.
function find_number() {
	numbers=($@)
	local lcb_pos=$((${#numbers[@]} - 1))
	local lcb=${numbers[lcb_pos]}
	unset numbers[lcb_pos]

	local i=0
	while [[ ${#numbers[@]} -gt 1 && $i -lt 12 ]]; do
		for nr in ${numbers[@]}; do
			local one=$(($nr >> (11 - $i) & 1 == 1))
			local count=$(count_bits $i)
			if [ $count == 0 ]; then
				if [ $one != $lcb ]; then
					local valid_numbers+=($nr)
				fi
			elif [[ $lcb == 1 && $one == $(($count < 0)) ]]; then
				local valid_numbers+=($nr)
			elif [[ $lcb == 0 && $one == $(($count > 0)) ]]; then
				local valid_numbers+=($nr)
			fi
		done

		((i++))
		local numbers=(${valid_numbers[@]})
		unset valid_numbers
	done
	echo ${numbers[0]}
}

function main() {
	local input=$(getInputFile 3)

	for line in $(cat $input); do
		numbers+=($((2#$line)))
	done

	local gamma=0
	local epsilon=0
	for i in $(seq 0 11); do
		if [ $(count_bits $i) -gt 0 ]; then
			((gamma |= 1 << (11 - $i)))
		else
			((epsilon |= 1 << (11 - $i)))
		fi
	done

	echo "The power consumption of the submarine is $(($gamma * $epsilon))."

	local oxygen=$(find_number ${numbers[@]} 0)
	local co2=$(find_number ${numbers[@]} 1)

	echo "The submarine life support rating is $((oxygen * co2))."
}

main
