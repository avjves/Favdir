#!/bin/bash

function fav() {
	OUTPUT=$($HOME/.fav/favdir $1 $2)
	if [ ! -z "$OUTPUT" ]
	then
		cd $OUTPUT
	fi
}
