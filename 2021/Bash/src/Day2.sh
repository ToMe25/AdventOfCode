#!/bin/bash
# Created on: 02.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 2)

	for line in $(cat $input); do
		local lines+=($line)
	done

	local aim=0
	for i in $(seq 0 $((${#lines[@]} / 2 - 1))); do
		local command=${lines[$i * 2]}
		local value=${lines[$i * 2 + 1]}
		if [ $command == "forward" ]; then
			((horizontal += $value))
			((depth += $value * $aim))
		elif [ $command == "down" ]; then
			((aim += $value))
		elif [ $command == "up" ]; then
			((aim -= $value))
		fi
	done

	echo "The horizontal multiplied by the aim(part 1 depth) is $(($horizontal*$aim))."
	echo "The horizontal multiplied by the depth is $(($horizontal*$depth))."
}

main
