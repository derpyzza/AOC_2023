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

col1.sort()
col2.sort()

# print(col1)
# print(col2)

ans = []

for x in range(nlines): 
    a = int(col1[x]) - int(col2[x])
    if a < 0:
        a *= -1    
    ans.append(a)

# print(ans)
s = 0

for x in range(0, nlines):
    s += ans[x]

print("out: ", s)
