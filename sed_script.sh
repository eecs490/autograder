#! /usr/bin/env bash
sed 's/REPLACE_START \([0-9]*\)/'"$(cat replace_start.txt)"'/g; s/REPLACE_END/'"$(cat replace_end.txt)"'/g' $1 
