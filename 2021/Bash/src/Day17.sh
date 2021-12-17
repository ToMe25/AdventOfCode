#!/bin/bash
# Created on: 17.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 17)

	for str in $(cat $input); do
		if [ ${str::1} == 'x' ]; then
			local str=${str:2:-1}
			local str=${str//./ }
			for nr in $str; do
				if [ ! -v target_x_min ]; then
					local target_x_min=$nr
				else
					local target_x_max=$nr
				fi
			done
		elif [ ${str::1} == 'y' ]; then
			local str=${str:2}
			local str=${str//./ }
			for nr in $str; do
				if [ ! -v target_y_min ]; then
					local target_y_min=$nr
				else
					local target_y_max=$nr
				fi
			done
		fi
	done

	local best_max_y=0
	local velocities=0
	for start_velocity_y in $(seq $target_y_min $(($target_y_min * -1))); do
		for start_velocity_x in $(seq 0 $target_x_max); do
			local x=0
			local y=0
			local max_y=0
			local vel_x=$start_velocity_x
			local vel_y=$start_velocity_y

			while [[ $vel_y -ge 0 || $y -gt $target_y_min ]]; do
				((x += $vel_x))
				((y += $vel_y))

				if [ $vel_x -gt 0 ]; then
					((vel_x--))
				elif [ $vel_x -lt 0 ]; then
					((vel_x++))
				fi
				((vel_y--))

				if [ $y -gt $max_y ]; then
					local max_y=$y
				fi

				if [[ $x -ge $target_x_min && $x -le $target_x_max && $y -ge $target_y_min && $y -le $target_y_max ]]; then
					if [ $max_y -gt $best_max_y ]; then
						local best_max_y=$max_y
					fi
					((velocities++))
					break
				fi
			done
		done
	done

	echo "The highest possible y position is $best_max_y."

	echo "There are a total of $velocities possible velocities that hit the target."
}

main
