#!/bin/bash
# Created on 14.11.2022
#     Author: ToMe25

source Util.sh

burrow_count=4

# Array used to get the burrow index for a type of amphipod.
declare -A burrow_index=([A]=0 [B]=1 [C]=2 [D]=3)

declare -a part2_burrows=(DD CB BA AC)

# Calculates the new token and cost after moving a amphipod according to the given values.
function move() {
	local token=$1
	local burrow=$2
	local burrow_len=$3
	local pos=$4
	local depth=$5

	local prefix=${token%%:*}
	local burrows_start=${#prefix}
	local burrow_pos=$((burrows_start + burrow * (burrow_len + 1) + depth + 1))

	local burrow_char=${token:burrow_pos:1}
	local new_token="${token::burrow_pos}${token:pos:1}${token:$((burrow_pos + 1))}"
	new_token="${new_token::pos}$burrow_char${new_token:$((pos + 1))}"
	if [ $burrow_char == "." ]; then
		local type=${new_token:burrow_pos:1}
	else
		local type=$burrow_char
	fi
	type=${burrow_index[$type]}

	local multi=$((10 ** type))
	if [ $multi -lt 1 ]; then
		multi=1
	fi

	local new_cost=$(((burrow + 1) * 2 - pos))
	new_cost=$(((${new_cost#-} + depth + 1) * multi))

	echo "$new_token $new_cost"
}

# Calculates the minimum cost for ordering the amphipods from the given token.
# Token structure is "SPACES:BURROWS:" where "BURROWS" is a colon separated list of the spaces in each burrow.
function get_min_cost() {
	local token=$1
	local -a valid_temp_spots=(0 1 3 5 7 9 10)
	local num_vts=${#valid_temp_spots[@]}
	local prefix=${token%%:*}
	local burrows_start=${#prefix}

	# Assume all burrows share the same length.
	prefix=${token:$((burrows_start + 1))}
	prefix=${prefix%%:*}
	local burrow_len=${#prefix}
	local -a burrows
	local seqbl=$(seq 0 $((burrow_len - 1)))
	local seqbs=$(seq 0 $((burrows_start - 1)))
	local seqbc=$(seq 0 $((burrow_count - 1)))

	local -A known
	known[$token]=0
	local checking="$token "

	local last=0
	while [ ${#checking} -gt 1 ]; do
		local ks=${#known[@]}
		if [ ! $((ks / 1000)) -eq $last ]; then
			last=$((ks / 1000))
			echo "Checked costs for $ks tokens." 1>&2
		fi

		token=${checking%% *}
		checking=${checking#* }
		unset cost

		for i in $seqbc; do
			burrows[$i]=${token:$((burrows_start + 1 + (burrow_len + 1) * i)):burrow_len}
		done

		local modified=0
		for i in $seqbs; do
			local c=${token:i:1}
			if [ "$c" != "." ]; then
				local burrow_idx=${burrow_index[$c]}
				local target=$(((burrow_idx + 1) * 2))
				local burrow=${burrows[$burrow_idx]}

				local free=1
				for ((j = ((i + 1) < target ? (i + 1) : target); j < (i > (target + 1) ? i : (target + 1)); j++)); do
					if [ ${token:j:1} != "." ]; then
						free=0
						break
					fi
				done

				if [ $free -eq 1 ]; then
					local bottom_free=0
					for j in $seqbl; do
						local bc=${burrow:j:1}
						if [ $bc == "." ]; then
							bottom_free=$j
						elif [ $bc != $c ]; then
							free=0
							break
						fi
					done

					if [ $free -eq 1 ]; then
						if [ -z "$cost" ]; then
							local cost=${known[$token]}
						fi

						local new_tk=$(move $token $burrow_idx $burrow_len $i $bottom_free)
						local new_token=${new_tk% *}
						local new_cost=$(($cost + ${new_tk#* }))
						local old_cost=${known[$new_token]}
						if [[ -z "$old_cost" || $old_cost -gt $new_cost ]]; then
							known[$new_token]=$new_cost
							checking+="$new_token "
							modified=1
						fi
					fi
				fi
			fi
		done

		if [ $modified -eq 1 ]; then
			continue
		fi

		for i in $seqbc; do
			local burrow=${burrows[$i]}
			for j in $seqbl; do
				if [ ${burrow:j:1} == "." ]; then
					continue
				fi

				local mismatch=0
				for ((k = j; k < $burrow_len; k++)); do
					local char=${burrow:k:1}
					if [ "${burrow_index[$char]-a}" != $i ]; then
						mismatch=1
						break
					fi
				done

				if [ ! $mismatch -eq 1 ]; then
					continue
				fi

				local burrow_idx=${burrow:j:1}
				burrow_idx=${burrow_index[$burrow_idx]}
				if [ $burrow_idx != $i ]; then
					local start=$(((i + 1) * 2))
					local target=$(((burrow_idx + 1) * 2))
					local tgt_burrow=${burrows[$burrow_idx]}

					local free=1
					for ((k = (start < target ? start : target); k <= (start > target ? start : target); k++)); do
						if [ ${token:k:1} != "." ]; then
							free=0
							break
						fi
					done

					if [ $free -eq 1 ]; then
						local bottom_free=0
						for k in $seqbl; do
							local bc=${tgt_burrow:k:1}
							if [ $bc == "." ]; then
								bottom_free=$k
							elif [ $bc != ${burrow:j:1} ]; then
								free=0
								break
							fi
						done

						if [ $free -eq 1 ]; then
							if [ -z "$cost" ]; then
								local cost=${known[$token]}
							fi

							local new_tk=$(move $token $i $burrow_len $target $j)
							local new_token=${new_tk% *}
							local new_cost=$(($cost + ${new_tk#* }))
							new_tk=$(move $new_token $burrow_idx $burrow_len $target $bottom_free)
							new_token=${new_tk% *}
							((new_cost += ${new_tk#* }))
							local old_cost=${known[$new_token]}
							if [[ -z "$old_cost" || $old_cost -gt $new_cost ]]; then
								known[$new_token]=$new_cost
								checking+="$new_token "
								modified=1
							fi
						fi
					fi

					if [ $modified -eq 1 ]; then
						continue
					fi
				fi

				for ((k = i + 1; k >= 0; k--)); do
					local pos=${valid_temp_spots[$k]}
					if [ ${token:pos:1} != "." ]; then
						break
					fi

					if [ -z "$cost" ]; then
						local cost=${known[$token]}
					fi

					local new_tk=$(move $token $i $burrow_len $pos $j)
					local new_token=${new_tk% *}
					local new_cost=$(($cost + ${new_tk#* }))
					local old_cost=${known[$new_token]}
					if [[ -z "$old_cost" || $old_cost -gt $new_cost ]]; then
						known[$new_token]=$new_cost
						checking+="$new_token "
					fi
				done

				for ((k = i + 2; k < $num_vts; k++)); do
					local pos=${valid_temp_spots[$k]}
					if [ ${token:pos:1} != "." ]; then
						break
					fi

					if [ -z "$cost" ]; then
						local cost=${known[$token]}
					fi

					local new_tk=$(move $token $i $burrow_len $pos $j)
					local new_token=${new_tk% *}
					local new_cost=$(($cost + ${new_tk#* }))
					local old_cost=${known[$new_token]}
					if [[ -z "$old_cost" || $old_cost -gt $new_cost ]]; then
						known[$new_token]=$new_cost
						checking+="$new_token "
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

	local prefix=${start_tk%%:*}
	local burrows_start=${#prefix}
	start_tk=${start_tk:0:burrows_start + 1}

	for i in $(seq 0 $((burrow_count - 1))); do
		burrow=${burrows[$i]}
		start_tk+=${burrow:0:1}${part2_burrows[$i]}${burrow:1:1}:
	done

	cost=$(get_min_cost $start_tk)

	echo "The cheapest way to order all part 2 amphipods has a cost of $cost."
}

main
