#!/bin/bash
# Created on: 02.12.2021
#     Author: ToMe25

# Gets the input file for the current day.
function getInputFile() {
	local input="../../input"

	input=$(realpath $input)

	if [ ! -e $input ]; then
		echo "Directory $INPUT doesn't exist."
		exit 1
	fi

	if [ ! -d $input ]; then
		echo "File $INPUT is not a directory."
		exit 1
	fi

	input+="/Day$1.txt"

	if [ ! -f $input ]; then
		echo "File $INPUT doesn't exist or isn't a file."
		exit 1
	fi

	echo $input
}
