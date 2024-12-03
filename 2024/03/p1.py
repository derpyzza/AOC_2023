import re

with open("./test.txt") as file:
    input = file.read()

print(sum([(int(x[0]) * int(x[1])) for x in re.findall(r"mul\((\d+),(\d+)\)", input)]))


