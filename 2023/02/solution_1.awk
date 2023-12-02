#!/bin/env/awk -f


# Read line
# Check if line matches given constraints
# Add lines that match given constraints to a list or something
# Add up the game number of all lines that match the constraints
# Echo sum;
#
# Variables:
# 	Blue
# 	Green
# 	Red
# 
# separate each game into a variable
#
#


BEGIN{
	FS=":"
	total=0
}
{
	# Get game number
	gameNum=substr($1, 5, 6);
	gamePossible=1;

	# Split line from the : onwards, on the delimiter ";" and store the result in rounds
	split($2, rounds, ";");

	# Loop over all the rounds 
	for(i=1; i<=length(rounds); i++){
		# Extract the colours from the rounds
		split(rounds[i], colours, ",")

		# Loop over all the colours
		for(j=1; j<=length(colours); j++) {
			split(colours[j], colour, " ");
			if (colour[2] == "blue") {
				if (colour[1] > 14) {
					print gameNum " not possible"
					gamePossible=0;
					break;
				}
			} else if (colour[2] == "red") {
				if (colour[1] > 12) {
					print gameNum " not possible"
					gamePossible=0;
					break;
				}
			} else if (colour[2] == "green") {
				if (colour[1] > 13) {
					print gameNum " not possible"
					gamePossible=0;
					break;
				}
			}
		}
	}

	if (gamePossible==1) {
		print "Game " gameNum " possible"
		total += gameNum;
	}
}

END{
	print total
}
