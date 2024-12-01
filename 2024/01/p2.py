
with open("./input.txt") as file:
    input = file.read()

lines = input.splitlines()
# lines = [
# "3   4",
# "4   3",
# "2   5",
# "1   3",
# "3   9",
# "3   3"
# ]
nlines = len(lines)

col1 = []
col2 = []

for line in lines:
    l = line.split("   ")
    col1.append(l[0])
    col2.append(l[1])

print(col1)
print(col2)

ans = []
times = [ 0 for _ in range(nlines) ]

for i, v in enumerate(col1):
    for y in col2:
        if y == v:
            times[i] += 1
print(times)

for i, x in enumerate(times):
    ans.append(int(col1[i]) * int(times[i]))

print(ans)

s = 0

for x in range(0, nlines):
    s += ans[x]

print(s)
