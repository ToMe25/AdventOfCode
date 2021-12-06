#!/bin/bash
# Created on: 05.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 6)

	local fishes=(0 0 0 0 0 0 0 0 0)
	local ORIGINAL_IFS=$IFS
	IFS=,
	for fish in $(cat $input); do
		((fishes[fish]++))
	done
	IFS=$ORIGINAL_IFS

	for i in $(seq 0 79); do
		local day=$(($i % 7))
		local temp=${fishes[7]}
		local fishes[7]=${fishes[8]}
		local fishes[8]=${fishes[$day]}
		((fishes[$day]+=$temp))
	done

	for i in $(seq 0 8); do
		((total+=${fishes[i]}))
	done

	echo "After 80 days there are $total lanternfishes."

	for i in $(seq 80 255); do
		local day=$(($i % 7))
		local temp=${fishes[7]}
		local fishes[7]=${fishes[8]}
		local fishes[8]=${fishes[$day]}
		((fishes[$day]+=$temp))
	done

	local total=0
	for i in $(seq 0 8); do
		((total+=${fishes[i]}))
	done

	echo "After 256 days there are $total lanternfishes."
}

main
