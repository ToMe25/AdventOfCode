#!/bin/bash
# Created on: 14.12.2021
#     Author: ToMe25

source Util.sh

# Expands polymer based on the rules from expansions.
# The number of times this is done is controlled by the first parameter.
# Returns the difference between the count of the most common and the least common char.
function expand_and_count() {
	local times=$1

	local -A polymer_cpy
	for i in $(seq 0 $(($times - 1))); do
		polymer_cpy=()
		for key in ${!polymer[@]}; do
			if [[ " ${!expansions[@]} " =~ " $key " ]]; then
				((polymer_cpy[${key::1}${expansions[$key]}]+=${polymer[$key]}))
				((polymer_cpy[${expansions[$key]}${key:1}]+=${polymer[$key]}))
			else
				((polymer_cpy[$key]+=${polymer[$key]}))
			fi
		done

		polymer=()
		for key in ${!polymer_cpy[@]}; do
			polymer[$key]=${polymer_cpy[$key]}
		done
	done

	local -A counts
	for key in ${!polymer[@]}; do
		((counts[${key:1}]+=${polymer[$key]}))
	done

	((counts[$first]++))

	local min=0
	local max=0
	for i in ${counts[@]}; do
		if [[ $min == 0 || $min -gt $i ]]; then
			local min=$i
		fi

		if [ $max -lt $i ]; then
			local max=$i
		fi
	done

	echo $(($max - $min))
}

function main() {
	local input=$(getInputFile 14)

	declare -A polymer
	declare -A expansions
	while read -r line; do
		if [ -z "${polymer[*]}" ]; then
			first=${line::1}
			for i in $(seq 0 $((${#line} - 2))); do
				((polymer[${line:$i:2}]++))
			done
		elif [ ${#line} -gt 0 ]; then
			expansions[${line::2}]=${line:6}
		fi
	done < $input

	local result=$(expand_and_count 10)

	echo "The part one result is $result."

	local result=$(expand_and_count 40)

	echo "The part two result is $result."
}

main
