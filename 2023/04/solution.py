file_path = './input.txt'

with open(file_path, 'r') as file:
    file_content = file.read()

lines = file_content.splitlines()

print("chugging...")
x = []
for line in lines:
    # 0 [Game 1], 1 [12 12 32 43 | 1 34 65 23 65 ...]
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

scores = []
for id, set in enumerate(win_nums):
    nums_list = set.split(" ")
    my_nums_list = my_nums[id].split(" ")
    score = 0
    for num in nums_list:
        if num == '':
            continue
        for n in my_nums_list:
            if n == '':
                continue
            if num == n:
                if not score:
                    score = 1
                else:
                    score = score * 2
    if score:
        scores.append(score)

total = 0
for score in scores:
    total += score

print(total)