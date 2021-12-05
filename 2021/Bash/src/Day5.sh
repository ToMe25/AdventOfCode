#!/bin/bash
# Created on: 05.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 5)

	local -A lines
	for line in $(cat $input); do
		if [[ $line =~ "," ]]; then
			local line=${line//,/ }
			for value in $line; do
				if [ ! -v x1 ]; then
					local x1=$value
				elif [ ! -v y1 ]; then
					local y1=$value
				elif [ ! -v x2 ]; then
					local x2=$value
				else
					local y2=$value
					if [[ $x1 == $x2 || $y1 == $y2 ]]; then
						for x in $(seq $x1 $(($x1 > $x2 ? -1 : 1)) $x2); do
							for y in $(seq $y1 $(($y1 > $y2 ? -1 : 1)) $y2); do
								((lines["$x:$y"]++))
							done
						done
					fi
					unset x1 x2 y1 y2
				fi
			done
		fi
	done

	for value in ${lines[@]}; do
		if [ $value -gt 1 ]; then
			((dangers++))
		fi
	done

	echo "Found $dangers part 1 dangerous spots."

	for line in $(cat $input); do
		if [[ $line =~ "," ]]; then
			local line=${line//,/ }
			for value in $line; do
				if [ ! -v x1 ]; then
					local x1=$value
				elif [ ! -v y1 ]; then
					local y1=$value
				elif [ ! -v x2 ]; then
					local x2=$value
				else
					local y2=$value
					local y=$y1
					if [[ $x1 != $x2 && $y1 != $y2 ]]; then
						for x in $(seq $x1 $(($x1 > $x2 ? -1 : 1)) $x2); do
							((lines["$x:$y"]++))
							(($y1 > $y2 ? y-- : y++))
						done
					fi
					unset x1 x2 y1 y2
				fi
			done
		fi
	done

	local dangers=0
	for value in ${lines[@]}; do
		if [ $value -gt 1 ]; then
			((dangers++))
		fi
	done

	echo "Found $dangers part 2 dangerous spots."
}

main
