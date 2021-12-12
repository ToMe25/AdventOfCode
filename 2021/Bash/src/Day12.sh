#!/bin/bash
# Created on: 12.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 12)

	for line in $(cat $input); do
		local line=${line//-/ }
		for str in $line; do
			if [ ! -v first ]; then
				local first=$str
			else
				local second=$str
			fi
		done

		local var="connections_$first"
		local var1="$var[@]"
		if [[ ! " ${!var1} " =~ " $second " ]]; then
			eval "local -a $var+=($second)"
		fi

		local var2="connections_$second"
		local var3="$var2[@]"
		if [[ ! " ${!var3} " =~ " $first " ]]; then
			eval "local -a $var2+=($first)"
		fi

		unset first
	done

	local -a to_check=("start")
	while [ ${#to_check[@]} -gt 0 ]; do
		local current=${to_check[-1]}
		local -a current_list=(${current//,// })
		unset to_check[-1]
		local var="connections_${current_list[-1]}[@]"
		for neighbor in ${!var}; do
			if [ $neighbor == "end" ]; then
				local -a routes+=($current,$neighbor)
			elif [[ $neighbor =~ [[:upper:]] || ! $current =~ $neighbor ]]; then
				local -a to_check+=($current,$neighbor)
			fi
		done
	done

	echo "There are a total of ${#routes[@]} routes without visiting a single small cave twice."

	unset routes
	local -a to_check=("start")
	local -a to_check_2=(0)
	while [ ${#to_check[@]} -gt 0 ]; do
		local current=${to_check[-1]}
		local -a current_list=(${current//,// })
		local double_small=${to_check_2[-1]}
		unset to_check[-1]
		unset to_check_2[-1]
		local var="connections_${current_list[-1]}[@]"
		for neighbor in ${!var}; do
			if [ $neighbor == "end" ]; then
				local -a routes+=($current,$neighbor)
			elif [[ $neighbor =~ [[:upper:]] || ! $current =~ $neighbor ]]; then
				local -a to_check+=($current,$neighbor)
				local -a to_check_2+=($double_small)
			elif [[ $neighbor != "start" && $double_small == 0 ]]; then
				local -a to_check+=($current,$neighbor)
				local -a to_check_2+=(1)
			fi
		done
	done

	echo "There are a total of ${#routes[@]} routes when visiting a single small cave twice."
}

main
