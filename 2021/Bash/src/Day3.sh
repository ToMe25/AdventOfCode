#!/bin/bash
# Created on: 02.12.2021
#     Author: ToMe25

source Util.sh

function count_bits() {
	local numbers=($@)
	local bit_pos=$((${#numbers[@]} - 1))
	local bit=${numbers[bit_pos]}
	unset numbers[bit_pos]

	local count=0
	for line in ${numbers[@]}]; do
		if [ ${line:$bit:1} == "1" ]; then
			((count++))
		else
			((count--))
		fi
	done
	echo $count
}

function find_number() {
	local numbers=($@)
	local lcb_pos=$((${#numbers[@]} - 1))
	local lcb=${numbers[lcb_pos]}
	unset numbers[lcb_pos]

	local i=0
	while [[ ${#numbers[@]} -gt 1 && $i -lt 12 ]]; do
		for nr in ${numbers[@]}; do
			local one=(${nr:$i:1} == "1")
			local count=$(count_bits ${numbers[@]} $i)
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
		local lines+=($line)
	done

	local gamma=0
	local epsilon=0
	for i in $(seq 0 11); do
		if [ $(count_bits ${lines[@]} $i) -gt 0 ]; then
			((gamma |= 1 << (11 - $i)))
		else
			((epsilon |= 1 << (11 - $i)))
		fi
	done

	echo "The power consumption of the submarine is $(($gamma * $epsilon))."

	local oxygen=$((2#$(find_number ${lines[@]} 0)))
	local co2=$((2#$(find_number ${lines[@]} 1)))

	echo "The submarine life support rating is $((oxygen * co2))."
}

main
