#!/bin/bash
# Created on: 10.12.2021
#     Author: ToMe25

source Util.sh

# Recursively checks this field and its neighbors for whether they are part of the same basin.
function recursive_check() {
	local x=$1
	local y=$2
	if [[ " ${checked[@]} " =~ " $x:$y " ]]; then
		return
	fi

	if [ ${map[$y * 100 + $x]} == 9 ]; then
		return
	fi

	((found++))
	checked+=($x:$y)

	if [ $x -gt 0 ]; then
		recursive_check $(($x - 1)) $y
	fi
	if [ $x -lt 99 ]; then
		recursive_check $(($x + 1)) $y
	fi

	if [ $y -gt 0 ]; then
		recursive_check $x $(($y - 1))
	fi
	if [ $y -lt 99 ]; then
		recursive_check $x $(($y + 1))
	fi
}

function main() {
	local input=$(getInputFile 9)

	for line in $(cat $input); do
		for i in $(seq 0 $((${#line} - 1))); do
			local -a map+=(${line:$i:1})
		done
	done

	for i in $(seq 0 $((${#map[@]} - 1))); do
		local lowest=1
		local here=${map[$i]}
		if [[ $i -gt 0 && $here -ge ${map[$i - 1]} ]]; then
			local lowest=0
		fi
		if [[ $i -lt 9999 && $here -ge ${map[$i + 1]} ]]; then
			local lowest=0
		fi

		if [[ $i -gt 99 && $here -ge ${map[$i - 100]} ]]; then
			local lowest=0
		fi
		if [[ $i -lt 9900 && $here -ge ${map[$i + 100]} ]]; then
			local lowest=0
		fi

		if [ $lowest == 1 ]; then
			((risk_levels_sum += $here + 1))
		fi
	done

	echo "The sum of all risk levels is $risk_levels_sum."

	let -a checked
	for y in $(seq 0 99); do
		for x in $(seq 0 99); do
			if [[ ${map[$y * 100 + $x]} != 9 && ! " ${checked[@]} " =~ " $x:$y " ]]; then
				found=0
				recursive_check $x $y
				if [ $found -gt 0 ]; then
					local -a basins+=($found)
				fi
			fi
		done
	done

	local -a basins=($(echo ${basins[@]} | tr ' ' '\n' | sort -n))
	local result=$((${basins[-1]} * ${basins[-2]} * ${basins[-3]}));

	echo "The product of the top three basin sizes is $result."
}

main
