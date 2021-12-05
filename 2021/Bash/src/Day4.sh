#!/bin/bash
# Created on: 04.12.2021
#     Author: ToMe25

source Util.sh

# Checks whether the given board contains a full row of marked(positive) numbers.
function check_win() {
	local board=($@)
	for x in $(seq 0 4); do
		local full=1
		for y in $(seq 0 4); do
			if [ ${board[$((5 * $x + $y))]} -lt 0 ]; then
				local full=0
			fi
		done

		if [ $full == 1 ]; then
			echo 1
			return
		fi
	done

	for y in $(seq 0 4); do
		local full=1
		for x in $(seq 0 4); do
			if [ ${board[$((5 * $x + $y))]} -lt 0 ]; then
				local full=0
			fi
		done

		if [ $full == 1 ]; then
			echo 1
			return
		fi
	done
	echo 0
}

# Marks the given number in all the given boards by making the number positive.
function set_number() {
	local number=$1
	local _number=$(($number * -1))

	for board in $(seq 0 99); do
		if [[ ! " ${winners[@]} " =~ " $board " ]]; then
			for x in $(seq 0 4); do
				for y in $(seq 0 4); do
					local pos=$(($board * 25 + $x * 5 + $y))
					if [ ${boards[$pos]} == $_number ]; then
						boards[$pos]=$number
					fi
				done
			done

			if [ $(check_win ${boards[@]:$(($board*25)):25}) == 1 ]; then
				winners+=($board)
			fi
		fi
	done
}

function main() {
	local input=$(getInputFile 4)

	for line in $(cat $input); do
		if [ ! -v numbers ]; then
			local numbers=${line//,/ }
		else
			boards+=("-$line")
		fi
	done

	for number in $numbers; do
		set_number $number
		if [[ ${#winners[@]} == 1 && ! -v first_win_number ]]; then
			local first_win_number=$number
		elif [ ${#winners[@]} == 100 ]; then
			break;
		fi
	done

	for x in $(seq 0 4); do
		for y in $(seq 0 4); do
			if [ ${boards[$((${winners[0]} * 25 + x * 5 + y))]} -lt 0 ]; then
				((unmarked += ${boards[$((${winners[0]} * 25 + x * 5 + y))]} * -1))
			fi
		done
	done

	echo "The first board winning score is $(($unmarked * $first_win_number))."

	unmarked=0
	for x in $(seq 0 4); do
		for y in $(seq 0 4); do
			if [ ${boards[$((${winners[99]} * 25 + x * 5 + y))]} -lt 0 ]; then
				((unmarked += ${boards[$((${winners[99]} * 25 + x * 5 + y))]} * -1))
			fi
		done
	done

	echo "The last board winning score is $(($unmarked * $number))."
}

main
