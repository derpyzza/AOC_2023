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
	blue=0;
	red=0;
	green=0;

	power=0;

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
				if (colour[1] > blue) {
					blue = colour[1];
				}
			} else if (colour[2] == "red") {
				if (colour[1] > red) {
					red = colour[1];
				}
			} else if (colour[2] == "green") {
				if (colour[1] > green) {
					green = colour[1];
				}
			}
		}
	}

	power = green * blue * red;
	print "Power: " power " Number: " gameNum;
	total += power;
}

END{
	print "two"
	print total
}
