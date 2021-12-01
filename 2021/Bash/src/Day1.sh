#!/bin/bash
# Created on: 01.12.2021
#     Author: ToMe25

INPUT="../../input"

INPUT=$(realpath $INPUT)

if [ ! -e $INPUT ]; then
	echo "Directory $INPUT doesn't exist."
	exit 1
fi

if [ ! -d $INPUT ]; then
	echo "File $INPUT is not a directory."
	exit 1
fi

INPUT+="/Day1.txt"

if [ ! -f $INPUT ]; then
	echo "File $INPUT doesn't exist or isn't a file."
	exit 1
fi

for DEPTH in $(cat $INPUT); do
	if [ ! -z $LAST_DEPTH ]; then
		if [ $DEPTH -gt $LAST_DEPTH ]; then
			INCREASES=$((INCREASES+1))
		fi

		if [ ! -z $LAST_LAST_DEPTH ]; then
			SUM=$((DEPTH+LAST_DEPTH+LAST_LAST_DEPTH))
			if [ ! -z $LAST_SUM ]; then
				if [ $SUM -gt $LAST_SUM ]; then
					SUM_INCREASES=$((SUM_INCREASES+1))
				fi
			fi
			LAST_SUM=$SUM
		fi
		LAST_LAST_DEPTH=$LAST_DEPTH
	fi
	LAST_DEPTH=$DEPTH
done

echo "The depth increased $INCREASES times."
echo "The depth sum increased $SUM_INCREASES times."
