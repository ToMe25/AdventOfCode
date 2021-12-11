#!/bin/bash
# Created on: 11.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 11)

	local map_size=10
	for line in $(cat $input); do
		for i in $(seq 0 $(($map_size - 1))); do
			local -a map+=(${line:$i:1})
		done
	done

	local finished=0
	local run=0
	while [ $finished != 1 ]; do
		for i in $(seq ${#map[@]}); do
			((map[$i - 1]++))
		done

		for i in $(seq 0 $((${#map[@]} - 1))); do
			if [ ${map[$i]} -gt 9 ]; then
				local -a stack+=($i)
				while [ ${#stack[@]} -gt 0 ]; do
					local pos=${stack[-1]}
					unset stack[-1]
					if [ $run -lt 100 ]; then
						((flashes++))
					fi

					local -a map[$pos]=0
					local x=$(($pos % $map_size))
					local y=$(($pos / $map_size))
					for j in $(seq $(($y > 0 ? $y - 1 : $y)) $(($y < $map_size - 1 ? $y + 1 : $y))); do
						for k in $(seq $(($x > 0 ? $x - 1 : $x)) $(($x < $map_size - 1 ? $x + 1 : $x))); do
							local pos1=$(($j * $map_size + $k))
							if [ ${map[$pos1]} -gt 0 ]; then
								((map[$pos1]++))
								if [[ ${map[$pos1]} -gt 9 && ! " ${stack[@]} " =~ " $pos1 " ]]; then
									local -a stack+=($pos1)
								fi
							fi
						done
					done
				done
			fi
		done

		local finished=1
		for i in $(seq ${#map[@]}); do
			if [ ${map[$i - 1]} != 0 ]; then
				local finished=0
				break;
			fi
		done
		((run++))
	done

	echo "After 100 steps there were $flashes flashes."
	echo "It took $run steps for all octopuses to flash simultaneously."
}

main
