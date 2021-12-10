#!/bin/bash
# Created on: 10.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 10)

	local closing=0
	for line in $(cat $input); do
		for i in $(seq 0 $((${#line} - 1))); do
			local c=${line:$i:1}
			if [[ $c == '(' || $c == '[' || $c == '{' || $c == '<' ]]; then
				local -a opened+=($c)
			elif [[ $c == ')' && ${opened[-1]} != '(' ]]; then
				((error_score += 3))
				local -a opened=()
				break
			elif [[ $c == ']' && ${opened[-1]} != '[' ]]; then
				((error_score += 57))
				local -a opened=()
				break
			elif [[ $c == '}' && ${opened[-1]} != '{' ]]; then
				((error_score += 1197))
				local -a opened=()
				break
			elif [[ $c == '>' && ${opened[-1]} != '<' ]]; then
				((error_score += 25137))
				local -a opened=()
				break
			else
				unset opened[-1]
			fi
		done

		while [ ${#opened[@]} -gt 0 ]; do
			((closing *= 5))
			if [ ${opened[-1]} == '(' ]; then
				((closing += 1))
			elif [ ${opened[-1]} == '[' ]; then
				((closing += 2))
			elif [ ${opened[-1]} == '{' ]; then
				((closing += 3))
			elif [ ${opened[-1]} == '<' ]; then
				((closing += 4))
			fi
			unset opened[-1]
		done

		if [ $closing -gt 0 ]; then
			local -a closing_strings+=($closing)
			local closing=0
		fi
	done

	echo "The error score is $error_score."

	local -a closing_strings=($(echo ${closing_strings[@]} | tr ' ' '\n' | sort -n))

	echo "The middle closing string is ${closing_strings[${#closing_strings[@]} / 2]}."
}

main
