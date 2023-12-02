#!/bin/bash

if [ -z "$1" ]; 
then
	awk -f solution_1.awk testCase.txt
else 
	awk -f solution_$1.awk testCase.txt
fi
