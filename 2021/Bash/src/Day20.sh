#!/bin/bash
# Created on: 20.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 20)

	local image
	local y=0
	for line in $(cat $input); do
		if [ ! -v enhancement ]; then
			local enhancement=""
			for i in $(seq 0 $((${#line} - 1))); do
				if [ ${line:$i:1} == '#' ]; then
					enhancement+=1
				else
					enhancement+=0
				fi
			done
		else
			if [ ! -v current_width ]; then
				local current_width=${#line}
			fi

			for x in $(seq 0 $((${#line} - 1))); do
				if [ ${line:$x:1} == '#' ]; then
					image+=1
				else
					image+=0
				fi
			done
		fi
	done

	local current_height=$((${#image} / $current_width))
	local empty_lit=0
	local new_image=""
	local new_height=$current_height
	local new_width=$current_width
	local lit=0
	local value;
	for i in $(seq 0 49); do
		((new_height += 2))
		((new_width += 2))
		new_image=""
		for j in $(seq 0 $(($new_height * $new_width - 1))); do
			value=""
			for k in $(seq -2 0); do
				local y=$(($j / $new_width + $k))
				if [[ $y -ge 0 && $y -lt $current_height ]]; then
					if [[ $(($j % $new_width)) -gt 1 && $(($j % $new_width)) -lt $current_width ]]; then
						value+=${image:$(($y * $current_width + $j % $new_width - 2)):3}
					else
						for l in $(seq -2 0); do
							local x=$(($j % $new_width + $l))
							if [[ $x -ge 0 && $x -lt $current_width ]]; then
								value+=${image:$(($y * $current_width + $x)):1}
							else
								value+=$empty_lit
							fi
						done
					fi
				else
					value+=$empty_lit$empty_lit$empty_lit
				fi
			done
			new_image+=${enhancement:$((2#$value)):1}
		done

		current_height=$new_height
		current_width=$new_width
		image=$new_image
		if [ $empty_lit == 1 ]; then
			empty_lit=${enhancement:511:1}
		else
			empty_lit=${enhancement::1}
		fi

		if [ $i == 1 ]; then
			for j in $(seq 0 $(($current_height * $current_width - 1))); do
				if [ ${image:$j:1} == 1 ]; then
					((lit++))
				fi
			done

			echo "The doubly enhanced image has $lit lit pixels."
		fi
	done

	lit=0
	for i in $(seq 0 $(($current_height * $current_width - 1))); do
		if [ ${image:$i:1} == 1 ]; then
			((lit++))
		fi
	done

	echo "After enhancing the image 50 times $lit pixels are lit."
}

main
