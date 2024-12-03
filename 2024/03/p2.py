import re

with open("./input.txt") as file:
    input = file.read()

r = re.findall(r"(do\(\))|(don\'t\(\))|(mul\(\d+,\d+\))", input)
m = []
for g in r:
    x = next(filter(None, g))
    m.append(x)

skip = False
s = 0
for i in m:
    if i == "don't()":
        skip = True
        continue
    elif i == "do()":
        skip = False
        continue

    if not skip:
        s += [(int(x[0]) * int(x[1])) for x in re.findall(r"mul\((\d+),(\d+)\)", i)][0]
        
print(s)

