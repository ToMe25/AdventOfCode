#!/bin/bash
# Created on 14.11.2022
#     Author: ToMe25

source Util.sh

burrow_count=4

# Array used to get the burrow index for a type of amphipod.
declare -A burrow_index=([A]=0 [B]=1 [C]=2 [D]=3)

# Calculates the new token and cost after moving a amphipod according to the given values.
function move() {
	local token=$1
	local cost=$2
	local burrow=$3
	local burrow_len=$4
	local pos=$5
	local depth=$6

	local prefix=${token%%:*}
	local burrows_start=${#prefix}
	local burrow_pos=$((burrows_start + burrow * (burrow_len + 1) + depth + 1))

	local new_token=$token
	local burrow_char=${token:burrow_pos:1}
	new_token="${new_token::burrow_pos}${token:pos:1}${new_token:$((burrow_pos + 1))}"
	new_token="${new_token::pos}$burrow_char${new_token:$((pos + 1))}"
	if [ $burrow_char == "." ]; then
		local type=${new_token:burrow_pos:1}
	else
		local type=$burrow_char
	fi
	type=${burrow_index[$type]}

	local multi=$((10**type))
	if [ $multi -lt 1 ]; then
		multi=1
	fi

	local new_cost=$(((burrow + 1) * 2 - pos))
	new_cost=$((cost + (${new_cost#-} + depth + 1) * multi))

	echo "$new_token $new_cost"
}

# Calculates the minimum cost for ordering the amphipods from the given token.
# Token structure is "SPACES:BURROWS:" where "BURROWS" is a colon separated list of the spaces in each burrow.
function get_min_cost() {
	local token=$1
	local -a valid_temp_spots=(0 1 3 5 7 9 10)
	local prefix=${token%%:*}
	local burrows_start=${#prefix}
	
	# Assume all burrows share the same length.
	prefix=${token:$((burrows_start + 1))}
	prefix=${prefix%%:*}
	local burrow_len=${#prefix}
	local -a burrows

	local -A known
	known[$token]=0
	local -a checking=($token)

	local last=0
	while [ ${#checking[@]} -gt 0 ]; do
		local ks=${#known[@]}
		if [ ! $((ks / 1000)) -eq $last ]; then
			last=$((ks / 1000))
			echo $ks 1>&2
			if [ $ks -gt 5000 ]; then
				return
			fi
		fi

		token=${checking[0]}
		unset checking[0]
		checking=(${checking[@]})
		local cost=${known[$token]}

		local start=$((burrows_start + 1))
		for i in $(seq $burrow_count); do
			burrows[$((i - 1))]=${token:start:burrow_len}
			start=$((start + burrow_len + 1))
		done

		local modified=0
		for i in $(seq 0 $((burrows_start - 1))); do
			local c=${token:i:1}
			if [ "$c" != "." ]; then
				local burrow_idx=${burrow_index[$c]}
				local target=$(((burrow_idx + 1) * 2))

				local occupied=0
				for j in $(seq $(((i + 1) < target ? (i + 1) : target)) $((i > (target + 1) ? i : (target + 1)))); do
					((j--))
					if [ ${token:j:1} != "." ]; then
						occupied=1
						break
					fi
				done

				if [ $occupied -eq 1 ]; then
					continue
				fi

				local bottom_free=0
				for j in $(seq 0 $((burrow_len - 1))); do
					local burrow=${burrows[burrow_idx]}
					if [ ${burrow:j:1} == "." ]; then
						bottom_free=$j
					elif [ ${burrow:j:1} != $c ]; then
						occupied=1
						break
					fi
				done

				if [ ! $occupied -eq 1 ]; then
					local new_tk=$(move $token $cost $burrow_idx $burrow_len $i $bottom_free)
					local new_token=${new_tk%% *}
					local new_cost=${new_tk##* }
					if [[ -z "${known[$new_token]+a}" || ${known[$new_token]} -gt $new_cost ]]; then
						known[$new_token]=$new_cost
						checking[${#checking[@]}]=$new_token
						modified=1
					fi
				fi
			fi
		done

		if [ $modified -eq 1 ]; then
			continue
		fi

		for i in $(seq 0 $((burrow_count - 1))); do
			for j in $(seq 0 $((burrow_len - 1))); do
				local burrow=${burrows[i]}
				if [ ${burrow:j:1} == "." ]; then
					continue
				fi

				local mismatch=0
				for k in $(seq 0 $((burrow_len - 1))); do
					local char=${burrow:k:1}
					if [ "${burrow_index[$char]-a}" != $i ]; then
						mismatch=1
						break
					fi
				done

				if [ ! $mismatch -eq 1 ]; then
					continue
				fi

				for k in $(seq $((i + 1)) -1 0); do
					local pos=${valid_temp_spots[k]}
					if [ "${token:pos:1}" != "." ]; then
						break
					fi

					local new_tk=$(move $token $cost $i $burrow_len $pos $j)
					local new_token=${new_tk%% *}
					local new_cost=${new_tk##* }
					if [[ -z "${known[$new_token]+a}" || ${known[$new_token]} -gt $new_cost ]]; then
						known[$new_token]=$new_cost
						checking[${#checking[@]}]=$new_token
					fi
				done

				for k in $(seq $((i + 2)) $((${#valid_temp_spots[@]} - 1))); do
					local pos=${valid_temp_spots[k]}
					if [ "${token:pos:1}" != "." ]; then
						break
					fi

					local new_tk=$(move $token $cost $i $burrow_len $pos $j)
					local new_token=${new_tk%% *}
					local new_cost=${new_tk##* }
					if [[ -z "${known[$new_token]+a}" || ${known[$new_token]} -gt $new_cost ]]; then
						known[$new_token]=$new_cost
						checking[${#checking[@]}]=$new_token
					fi
				done

				break
			done
		done
	done

	local solved_state=""
	for i in $(seq $burrows_start); do
		solved_state+="."
	done
	solved_state+=':'

	local -a burrow_chars=(A B C D)
	for i in $(seq 0 $((burrow_count -1))); do
		for j in $(seq $burrow_len); do
			solved_state+=${burrow_chars[i]}
		done
		solved_state+=':'
	done

	for key in "${!known[@]}"; do
		echo "$key=${known[$key]}"
	done
	echo $solves_state
	echo ${known[$solved_state]}
}

function main() {
	local input=$(getInputFile 23)

	local -a burrows
	while read -r line; do
		line=${line//[# ]/}

		if [ ${#line} == 0 ]; then
			continue;
		fi

		if [ -z "${start_tk}" ]; then
			local start_tk=$line
		else
			for i in $(seq $burrow_count); do
				burrows[$((i - 1))]+=${line:i-1:1}
			done
		fi
	done < $input

	start_tk+=":"
	for i in $(seq $burrow_count); do
		start_tk+="${burrows[$i-1]}:"
	done

	local cost=$(get_min_cost $start_tk)

	echo "The cheapest way to order all part 1 amphipods has a cost of $cost."
}

main
