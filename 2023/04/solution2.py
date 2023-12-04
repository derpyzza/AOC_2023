file_path = './input.txt'

with open(file_path, 'r') as file:
    file_content = file.read()

lines = file_content.splitlines()

print("chugging...")
x = []
for line in lines:
    y = line.split(":")
    x.append(y)
    
f = []
for y in x:
    g = y[1].split("|")
    f.append(g)

win_nums = []
my_nums = []
for g in f:
    win_nums.append(g[0])
    my_nums.append(g[1])

scores = [1 for _ in range(len(lines))]
for id, set in enumerate(win_nums):
    nums_list = set.split(" ")
    my_nums_list = my_nums[id].split(" ")
    score = 1
    for num in nums_list:
        if num == '':
            continue
        for n in my_nums_list:
            if n == '':
                continue
            if num == n:
                score += 1
    for y in range(0, scores[id]):
        for x in range(1, score):
            scores[id+x] += 1
    
print(scores)

total = 0
for score in scores:
    total += score

print(total)