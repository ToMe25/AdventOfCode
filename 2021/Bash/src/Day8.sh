#!/bin/bash
# Created on: 08.12.2021
#     Author: ToMe25

source Util.sh

function main() {
	local input=$(getInputFile 8)

	local -a number_to_chars_0=('a' 'b' 'c' 'e' 'f' 'g')
	local -a number_to_chars_1=('c' 'f')
	local -a number_to_chars_2=('a' 'c' 'd' 'e' 'g')
	local -a number_to_chars_3=('a' 'c' 'd' 'f' 'g')
	local -a number_to_chars_4=('b' 'c' 'd' 'f')
	local -a number_to_chars_5=('a' 'b' 'd' 'f' 'g')
	local -a number_to_chars_6=('a' 'b' 'd' 'f' 'e' 'g')
	local -a number_to_chars_7=('a' 'c' 'f')
	local -a number_to_chars_8=('a' 'b' 'c' 'd' 'e' 'f' 'g')
	local -a number_to_chars_9=('a' 'b' 'c' 'd' 'f' 'g')

	for i in $(seq 0 9); do
		local var="number_to_chars_$i[@]"
		for c in ${!var}; do
			eval "local -a char_to_numbers_$c+=($i)"
		done
	done

	for c in {a..g}; do
		local occurrences=0
		for i in $(seq 0 9); do
			local var="number_to_chars_$i[@]"
			if [[ " ${!var} " =~ " $c " ]]; then
				((occurrences++))
			fi
		done
		eval "local -a occurrences_to_possible_chars_$occurrences+=($c)"
	done

	while read -r line; do
		local -a tokens=()
		local output=0
		local solved=0
		for token in $line; do
			local token=$(echo $token | grep -o . | sort | tr -d "\n")
			if [ $token == "|" ]; then
				local output=1
				for c in {a..g}; do
					local occurrences=0
					for t in ${tokens[@]}; do
						if [[ $t =~ $c ]]; then
							((occurrences++))
						fi
					done

					local var="occurrences_to_possible_chars_$occurrences[@]"
					eval "local -a char_to_possible_chars_$c=(${!var})"
				done

				while [ $solved != 1 ]; do
					# Remove chars that are no longer possible options from char_to_possible_chars.
					for t in ${tokens[@]}; do
						local var="token_to_possible_numbers_$t[@]"
						local -a var1=(${!var})

						local -a possible_chars=()
						for nr in ${var1[@]}; do
							local var2="number_to_chars_$nr[@]"
							local -a var3=(${!var2})
							for c in ${var3[@]}; do
								if [[ ! " ${possible_chars[@]} " =~ " $c " ]]; then
									possible_chars+=($c)
								fi
							done
						done

						for i in $(seq 0 $((${#t} - 1))); do
							local c=${t:$i:1}
							local var2="char_to_possible_chars_$c[@]"
							local -a var3=(${!var2})
							for c1 in ${var3[@]}; do
								if [[ ! " ${possible_chars[@]} " =~ " $c1 " ]]; then
									local var4="char_to_possible_chars_$c"
									eval "local -a $var4=(${!var2/$c1})"
								fi
							done
						done
					done

					# Remove numbers that don't contain required chars from token_to_possible_numbers.
					for c in {a..g}; do
						local var="char_to_possible_chars_$c[@]"
						local -a var1=(${!var})
						if [ ${#var1[@]} == 1 ]; then
							for t in ${tokens[@]}; do
								if [[ $t =~ $c ]]; then
									local var2="token_to_possible_numbers_$t[@]"
									local -a var3=(${!var2})
									local c1=${var1[0]}
									local var4="char_to_numbers_$c1[@]"
									local -a var5=(${!var4})
									for nr in ${var3[@]}; do
										if [[ ! " ${var5[@]} " =~ " $nr " ]]; then
											local var6="token_to_possible_numbers_$t"
											eval "local -a $var6=(${!var2/$nr})"
										fi
									done
								fi
							done
						fi
					done

					local solved=1
					for t in ${tokens[@]}; do
						local var="token_to_possible_numbers_$t[@]"
						local -a var1=(${!var})
						if [ ${#var1[@]} -gt 1 ]; then
							local solved=0
						fi
					done
				done
			elif [ $output == 1 ]; then
				local var="token_to_possible_numbers_$token[0]"
				local out_nr=${!var}
				if [ $out_nr == 1 ]; then
					((ones++))
				elif [ $out_nr == 4 ]; then
					((fours++))
				elif [ $out_nr == 7 ]; then
					((sevens++))
				elif [ $out_nr == 8 ]; then
					((eights++))
				fi

				local out_str+=$out_nr
				if [ ${#out_str} == 4 ]; then
					while [ ${out_str::1} == 0 ]; do
						local out_str=${out_str:1}
					done
					((out += out_str))
					unset out_str
					local output=0
				fi
			else
				tokens+=($token)
				if [ ${#token} == 2 ]; then
					eval "local -a token_to_possible_numbers_$token=(1)"
				elif [ ${#token} == 3 ]; then
					eval "local -a token_to_possible_numbers_$token=(7)"
				elif [ ${#token} == 4 ]; then
					eval "local -a token_to_possible_numbers_$token=(4)"
				elif [ ${#token} == 5 ]; then
					eval "local -a token_to_possible_numbers_$token=(2 3 5)"
				elif [ ${#token} == 6 ]; then
					eval "local -a token_to_possible_numbers_$token=(0 6 9)"
				elif [ ${#token} == 7 ]; then
					eval "local -a token_to_possible_numbers_$token=(8)"
				fi
			fi
		done
	done < $input

	echo "The part 1 result is $(($ones + $fours + $sevens + $eights))."
	echo "The part 2 result is $out."
}

main
