#!/bin/bash
# Created on: 13.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 13)

	local map_width=2000
	local map_height=2000
	for line in $(cat $input); do
		if [[ $line =~ "," ]]; then
			for str in ${line//,/ }; do
				if [ ! -v x ]; then
					local x=$str
				else
					local y=$str
				fi
			done
			local -a map[$(($y * $map_width + $x))]=1
			unset x
		elif [[ $line =~ "=" ]]; then
			local axis=${line::1}
			local pos=${line:2}
			if [ $axis == "x" ]; then
				local new_map_width=$pos
				local new_map_height=$map_height
			else
				local new_map_width=$map_width
				local new_map_height=$pos
			fi
			for y in $(seq 0 $(($map_height - 1))); do
				for x in $(seq 0 $(($map_width - 1))); do
					if [ ${map[$y * $map_width + $x]-0} == 1 ]; then
						if [ $axis == "x" ]; then
							if [ $x -lt $pos ]; then
								local -a new_map[$(($y * $new_map_width + $x))]=${map[$y * $map_width + $x]}
							elif [ $x -lt $(($pos * 2 + 1)) ]; then
								local -a new_map[$(($y * $new_map_width + $pos * 2 - $x))]=${map[$y * $map_width + $x]}
							fi
						else
							if [ $y -lt $pos ]; then
								local -a new_map[$(($y * $new_map_width + $x))]=${map[$y * $map_width + $x]}
							elif [ $y -lt $(($pos * 2 + 1)) ]; then
								local -a new_map[$((($pos * 2 - $y) * $new_map_width + $x))]=${map[$y * $map_width + $x]}
							fi
						fi
					elif [[ $x -lt $new_map_width && $y -lt $new_map_height && -z ${new_map[$y * $new_map_width + $x]} ]]; then
						local -a new_map[$(($y * $new_map_width + $x))]=0
					fi
				done
			done
			local -a map=(${new_map[@]})
			local map_width=$new_map_width
			local map_height=$new_map_height
			unset new_map

			if [ ! -v points ]; then
				for y in $(seq 0 $(($map_height - 1))); do
					for x in $(seq 0 $(($map_width - 1))); do
						if [ ${map[$(($y * $map_width + $x))]} == 1 ]; then
							((points++))
						fi
					done
				done

				echo  "After one fold there are $points points."
			fi
		fi
	done

	echo "The result after all folds is:"
	for y in $(seq 0 $(($map_height - 1))); do
		for x in $(seq 0 $(($map_width - 1))); do
			if [ ${map[$y * $map_width + $x]} == 1 ]; then
				local line+="#"
			else
				local line+="."
			fi
		done
		echo $line
		unset line
	done
}

main
