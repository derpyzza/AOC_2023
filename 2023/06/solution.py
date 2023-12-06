file_path = './input.txt'

with open(file_path, 'r') as file:
    file_content = file.read()

lines = file_content.splitlines()

def simulate_race(time, target_distance):
        correct_dists = []
        for hold_time in range(0, time):
            # print(hold_time)
            travel_dist = (time - hold_time) * hold_time
            travel_dist > target_distance and correct_dists.append(hold_time)
        return len(correct_dists)

def solve_problem_one() -> int:
    chrs = []

    for line in lines:
        chr = list(map(int, line.split()[1:]))
        chrs.append(chr)

    list_len = len(chrs[0])
 
    nums = []
    for x in range(0, list_len):
        nums.append(simulate_race(chrs[0][x], chrs[1][x]))

    total = 1
    for num in nums:
        total *= num
    return total

def solve_problem_two() -> int:
    chrs = []

    for line in lines:
        chr = line.split()[1:]
        x = ''
        for c in chr:
            x += c
        chrs.append(int(x))

    x = simulate_race(chrs[0], chrs[1])
    return x

print(solve_problem_one())
print(solve_problem_two())

