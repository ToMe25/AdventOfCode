#!/bin/bash
# Created on: 01.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 1)
	for depth in $(cat $input); do
		if [ ! -z $last_depth ]; then
			if [ $depth -gt $last_depth ]; then
				((increases++))
			fi

			if [ ! -z $last_last_depth ]; then
				local sum=$((depth + last_depth + last_last_depth))
				if [ ! -z $last_sum ]; then
					if [ $sum -gt $last_sum ]; then
						((sum_increases++))
					fi
				fi
				local last_sum=$sum
			fi
			local last_last_depth=$last_depth
		fi
		local last_depth=$depth
	done

	echo "The depth increased $increases times."
	echo "The depth sum increased $sum_increases times."
}

main
