#!/bin/bash
# Created on: 16.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 16)
	local line=$(cat $input)
	for i in $(seq 0 $((${#line} - 1))); do
		local char=${line:$i:1}
		if [ $char == 0 ]; then
			local binary+="0000"
		elif [ $char == 1 ]; then
			local binary+="0001"
		elif [ $char == 2 ]; then
			local binary+="0010"
		elif [ $char == 3 ]; then
			local binary+="0011"
		elif [ $char == 4 ]; then
			local binary+="0100"
		elif [ $char == 5 ]; then
			local binary+="0101"
		elif [ $char == 6 ]; then
			local binary+="0110"
		elif [ $char == 7 ]; then
			local binary+="0111"
		elif [ $char == 8 ]; then
			local binary+="1000"
		elif [ $char == 9 ]; then
			local binary+="1001"
		elif [ $char == A ]; then
			local binary+="1010"
		elif [ $char == B ]; then
			local binary+="1011"
		elif [ $char == C ]; then
			local binary+="1100"
		elif [ $char == D ]; then
			local binary+="1101"
		elif [ $char == E ]; then
			local binary+="1110"
		elif [ $char == F ]; then
			local binary+="1111"
		fi
	done

	local version_sum
	local -a packets
	local -A packet_subpackets
	local -A packet_types
	local -A packet_values
	local i=0
	while [[ $i == 0 || ${#packets[@]} -gt 0 ]]; do
		if [ ${#packets[@]} -gt 0 ]; then
			local packet=${packets[-1]}
			if [[ (${packet_subpackets[$packet]::1} == 1 && ${packet_subpackets[$packet]:1} == 0)
					|| (${packet_subpackets[$packet]::1} != 1 && $(($packet + ${packet_subpackets[$packet]:1})) -le $i) ]]
			then
				local value=${packet_values[$packet]}
				unset packets[-1]

				if [ ${#packets[@]} == 0 ]; then
					local result=$value
				else
					local packet=${packets[-1]}
					if [ ${packet_types[$packet]} == 0 ]; then
						if [ ${packet_values[$packet]} == -1 ]; then
							local packet_values[$packet]=0
						fi
						((packet_values[$packet] += $value))
					elif [ ${packet_types[$packet]} == 1 ]; then
						if [ ${packet_values[$packet]} == -1 ]; then
							local packet_values[$packet]=1
						fi
						((packet_values[$packet] *= $value))
					elif [ ${packet_types[$packet]} == 2 ]; then
						if [[ ${packet_values[$packet]} == -1 || $value -lt ${packet_values[$packet]} ]]; then
							local packet_values[$packet]=$value
						fi
					elif [ ${packet_types[$packet]} == 3 ]; then
						if [ $value -gt ${packet_values[$packet]} ]; then
							local packet_values[$packet]=$value
						fi
					elif [ ${packet_types[$packet]} == 5 ]; then
						if [ ${packet_values[$packet]} == -1 ]; then
							local packet_values[$packet]=$value
						else
							local packet_values[$packet]=$((${packet_values[$packet]} > $value))
						fi
					elif [ ${packet_types[$packet]} == 6 ]; then
						if [ ${packet_values[$packet]} == -1 ]; then
							local packet_values[$packet]=$value
						else
							local packet_values[$packet]=$((${packet_values[$packet]} < $value))
						fi
					elif [ ${packet_types[$packet]} == 7 ]; then
						if [ ${packet_values[$packet]} == -1 ]; then
							local packet_values[$packet]=$value
						else
							local packet_values[$packet]=$((${packet_values[$packet]} == $value))
						fi
					fi

					if [ ${packet_subpackets[$packet]::1} == 1 ]; then
						local packet_subpackets[$packet]=${packet_subpackets[$packet]::1}$((${packet_subpackets[$packet]:1} - 1))
					fi
				fi

				continue
			fi
		fi

		local version=$((2#${binary:$i:3}))
		((i += 3))
		((version_sum += $version))
		local type=$((2#${binary:$i:3}))
		((i += 3))

		if [ $type == 4 ]; then
			local reading=1
			local value=0
			while [ $reading == 1 ]; do
				local reading=${binary:$i:1}
				((i++))
				local value=$(($value << 4))
				((value += 2#${binary:$i:4}))
				((i += 4))
			done

			local packets+=($i)
			local packet_subpackets[$i]=10
			local packet_types[$i]=$type
			local packet_values[$i]=$value
		else
			local subpackets_number=${binary:$i:1}
			((i++))
			if [ $subpackets_number == 1 ]; then
				local subpackets=$((2#${binary:$i:11}))
				((i += 11))
			else
				local subpackets=$((2#${binary:$i:15}))
				((i += 15))
			fi

			local packets+=($i)
			local packet_subpackets[$i]=$subpackets_number$subpackets
			local packet_types[$i]=$type
			local packet_values[$i]=-1
		fi
	done

	echo "The sum of all version headers is $version_sum."

	echo "The result of the outer most packet is $result."
}

main
