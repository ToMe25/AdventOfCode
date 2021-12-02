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
	for i in $(seq 0 $((${#lines[@]}/2-1))); do
		if [ ${lines[$((i*2))]} == "forward" ]; then
			local horizontal=$(($horizontal+${lines[$((i*2+1))]}))
			local depth=$(($depth+${lines[$((i*2+1))]}*$aim))
		elif [ ${lines[$((i*2))]} == "down" ]; then
			local aim=$(($aim+${lines[$((i*2+1))]}))
		elif [ ${lines[$((i*2))]} == "up" ]; then
			local aim=$(($aim-${lines[$((i*2+1))]}))
		fi
	done

	echo "The horizontal multiplied by the aim(part 1 depth) is $(($horizontal*$aim))."
	echo "The horizontal multiplied by the depth is $(($horizontal*$depth))."
}

main
