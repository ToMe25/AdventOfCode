#!/bin/bash
# Created on: 21.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 21)

	while read -r line; do
		if [ ${#line} == 0 ]; then
			continue;
		fi

		if [ ! -v player_1_start_pos ]; then
			for token in $line; do
				local player_1_start_pos=$token
			done
		else
			for token in $line; do
				local player_2_start_pos=$token
			done
		fi
	done < $input

	local dice=1
	local player_1_points
	local player_2_points
	local player_1_pos=$player_1_start_pos
	local player_2_pos=$player_2_start_pos
	local rolls
	while [[ $player_1_points -lt 1000 && $player_2_points -lt 1000 ]]; do
		for i in $(seq 0 2); do
			((player_1_pos += $dice))
			dice=$(($dice % 100 + 1))
		done

		player_1_pos=$((($player_1_pos - 1) % 10 + 1))
		((player_1_points += $player_1_pos))

		if [ $player_1_points -ge 1000 ]; then
			((rolls += 3))
			break;
		fi

		for i in $(seq 0 2); do
			((player_2_pos += $dice))
			dice=$(($dice % 100 + 1))
		done

		player_2_pos=$((($player_2_pos - 1) % 10 + 1))
		((player_2_points += $player_2_pos))
		((rolls += 6))
	done

	if [ $player_1_points -lt $player_2_points ]; then
		local result=$(($rolls * $player_1_points))
	else
		local result=$(($rolls * $player_2_points))
	fi

	echo "The number of die rolls multiplied by the losing players score is $result."

	local -a dirac_values
	for i in $(seq 0 2); do
		for j in $(seq 0 2); do
			for k in $(seq 0 2); do
				((dirac_values[$i+$j+$k]++))
			done
		done
	done

	local -a universes
	local -A cache
	universes+=("$player_1_start_pos:0:$player_2_start_pos:0")
	while [ ${#universes[@]} -gt 0 ]; do
		local -a current=(${universes[-1]//:/ })
		local found=1
		local player_1_wins=0
		local player_2_wins=0

		for i in $(seq 2 8); do
			player_1_pos=${current[0]}
			player_1_pos=$((($player_1_pos + $i) % 10 + 1))
			player_1_points=$((${current[1]} + $player_1_pos))

			if [ $player_1_points -ge 21 ]; then
				((player_1_wins += ${dirac_values[$i-2]}))
			else
				for j in $(seq 2 8); do
					player_2_pos=${current[2]}
					player_2_pos=$((($player_2_pos + $j) % 10 + 1))
					player_2_points=$((${current[3]} + $player_2_pos))
					local next="$player_1_pos:$player_1_points:$player_2_pos:$player_2_points"

					if [ $player_2_points -ge 21 ]; then
						((player_2_wins += ${dirac_values[$i-2]} * ${dirac_values[$j-2]}))
					elif [ -v cache[$next] ]; then
						local -a cached=(${cache[$next]//:/ })
						((player_1_wins += ${cached[0]} * ${dirac_values[$i-2]} * ${dirac_values[$j-2]}))
						((player_2_wins += ${cached[1]} * ${dirac_values[$i-2]} * ${dirac_values[$j-2]}))
					else
						universes+=($next)
						found=0
					fi
				done
			fi
		done

		if [ $found == 1 ]; then
			cache[${universes[-1]}]="$player_1_wins:$player_2_wins"
			unset universes[-1]
		fi
	done

	if [ $player_1_wins -gt $player_2_wins ]; then
		result=$player_1_wins
	else
		result=$player_2_wins
	fi

	echo "The player that won in more universes won in $result universes."
}

main
