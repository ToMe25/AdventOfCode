#!/bin/bash
# Created on: 19.12.2021
#     Author: ToMe25

source Util.sh

# Adds the two given snailfish numbers together and returns the reduced result.
function add_snailfish() {
	local first=$1
	local second=$2
	local current="[$first,$second]"
	local modified=1

	while [ $modified == 1 ]; do
		local modified=0
		local last_nr=0
		local last_nr_i=0
		local -a pairs=()
		local i=0

		while [ $i -lt ${#current} ]; do
			local c=${current:$i:1}
			if [ $c == "[" ]; then
				local pairs+=($i)
			elif [ $c == "]" ]; then
				unset pairs[-1]
			elif [[ $c =~ [0-9] ]]; then
				if [ ${#pairs[@]} -gt 4 ]; then
					if [[ ${current:$i+1:1} == "," && ${current:$i+2:1} =~ [0-9] ]]; then
						if [ $last_nr_i -gt 0 ]; then
							if [[ ${current:$i-1:1} =~ [0-9] ]]; then
								local number=${current:$i-1:2}
							else
								local number=$c
							fi

							if [ $last_nr -gt 9 ]; then
								local current=${current::$last_nr_i}$(($number + $last_nr))${current:$last_nr_i+2}
							else
								local current=${current::$last_nr_i}$(($number + $last_nr))${current:$last_nr_i+1}
								if [ $(($last_nr + $number)) -gt 9 ]; then
									((i++))
								fi
							fi
						fi

						((i += 2))

						for j in $(seq $(($i + 1)) $((${#current} - 1))); do
							if [[ ${current:$j:1} =~ [0-9] ]]; then
								if [[ ${current:$i+1:1} =~ [0-9] ]]; then
									if [ $(($i + 1)) == $j ]; then
										continue
									fi
									local number=${current:$i:2}
								else
									local number=${current:$i:1}
								fi

								if [[ ${current:$j+1:1} =~ [0-9] ]]; then
									((number += ${current:$j:2}))
									local current=${current::$j}$number${current:$j+2}
									break
								else
									((number += ${current:$j:1}))
									local current=${current::$j}$number${current:$j+1}
									break
								fi
							fi
						done

						if [[ ${current:$i-3:1} =~ [0-9] && ${current:$i+1:1} =~ [0-9] ]]; then
							local current=${current::$i-4}0${current:$i+3}
							((i -= 4))
						elif [[ ${current:$i-3:1} =~ [0-9] ]]; then
							local current=${current::$i-4}0${current:$i+2}
							((i -= 4))
						elif [[ ${current:$i+1:1} =~ [0-9] ]]; then
							local current=${current::$i-3}0${current:$i+3}
							((i -= 3))
						else
							local current=${current::$i-3}0${current:$i+2}
							((i -= 3))
						fi

						unset pairs[-1]
						local last_nr_i=$i
						local last_nr=0
						local modified=1
					fi
				else
					if [ $(($last_nr_i + 1)) == $i ]; then
						local last_nr=${current:$i-1:2}
					else
						local last_nr=${current:$i:1}
						local last_nr_i=$i
					fi
				fi
			fi
			((i++))
		done

		if [ $modified != 1 ]; then
			for i in $(seq 0 $((${#current} - 1))); do
				local c=${current:$i:1}
				if [ $c == "[" ]; then
					local pairs+=($i)
				elif [ $c == "]" ]; then
					unset pairs[-1]
				elif [[ $c =~ [0-9] && ${current:$i+1:1} =~ [0-9] ]]; then
					local number=${current:$i:2}
					local current="${current::$i}[$(($number / 2)),$((($number + 1) / 2))]${current:$i+2}"
					local modified=1
					break
				fi
			done
		fi
	done

	echo $current
}

# Calculates the magnitude for the given shellfish number.
function get_magnitude() {
	local number=$1
	for i in $(seq 0 $((${#number} - 1))); do
		local c=${number:$i:1}
		if [[ $c =~ [0-9] ]]; then
			local -a numbers+=($c)
		elif [ $c == "]" ]; then
			local nr=$((${numbers[-1]} * 2))
			unset numbers[-1]
			((nr += ${numbers[-1]} * 3))
			unset numbers[-1]
			numbers+=($nr)
		fi
	done

	echo ${numbers[-1]}
}

function main() {
	local input=$(getInputFile 18)

	for line in $(cat $input); do
		if [ ! -v current ]; then
			local current=$line
		else
			local current=$(add_snailfish $current $line)
		fi
		local -a lines+=($line)
	done

	local magnitude=$(get_magnitude $current)

	echo "The resulting magnitute is $magnitude."

	local max_magnitude=0
	for line in ${lines[@]}; do
		for line2 in ${lines[@]}; do
			if [ $line != $line2 ]; then
				local magnitude=$(get_magnitude $(add_snailfish $line $line2))
				if [ $magnitude -gt $max_magnitude ]; then
					local max_magnitude=$magnitude
				fi
			fi
		done
	done

	echo "The max possible magnitude from adding two numbers is $max_magnitude."
}

main
