#!/bin/bash
# Created on: 07.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 7)

	local ORIGINAL_IFS=$IFS
	IFS=,
	for number in $(cat $input); do
		local numbers+=($number)
		((average+=$number))
	done
	IFS=$ORIGINAL_IFS

	local numbers=($(echo ${numbers[@]} | tr ' ' '\n' | sort -n))
	local median=${numbers[$((${#numbers[@]} / 2))]}

	for nr in ${numbers[@]}; do
		local dist=$(($nr - $median))
		((distance += ${dist#-}))
	done

	echo "The total fuel used for part 1 is $distance units."

	((average /= ${#numbers[@]}))

	local distance=0
	for nr in ${numbers[@]}; do
		local dist=$(($nr - $average))
		local dist=${dist#-}
		((distance += ($dist + 1) * $dist / 2))
	done

	echo "The total fuel used for part 2 is $distance units."
}

main
