
import re

file_path = './testCase.txt'

with open(file_path, 'r') as file:
    file_content = file.read()

lines = file_content.splitlines()
chr = []

for line in lines:
    l = [*line]
    chr.append(l)


def printCharAtPos(grid: list[list], x: int, y: int):
    print(grid[x][y])

def extractNums(grid: list[list[str]]):
    nums = []
    for rowId, row in enumerate(grid):
        total = ''
        isFirst=False
        firstId=0
        lastId=0
        for id, val in enumerate(row):
            print(rowId, id, val)
            next = id+1
            if val.isnumeric():
                if not isFirst:
                    isFirst = True
                    firstId = id
                if next == len(row):
                    print("true")
                    if row[id].isnumeric():
                        lastId = id
                        total += val
                        nums.append({'num': int(total), 'pos_x': firstId, 'pos_y': rowId, 'lastId': lastId})
                        firstId=0
                        isFirst=False
                        lastId = 0
                        total = ''
                    else:
                        total += val
                else:
                    print("false", next, len(row))
                    if not row[next].isnumeric():
                        lastId = id    
                        total += val
                        nums.append({'num': int(total), 'pos_x': firstId, 'pos_y': rowId, 'lastId': lastId})
                        firstId=0
                        isFirst=False
                        lastId = 0
                        total = ''
                    else:
                        total += val
    return nums

def extractGears(grid: list[list[str]]):
    gears = []
    for rowId, row in enumerate(grid):
        for id, val in enumerate(row):
            if val == '*':
                gears.append({'pos_x': id, 'pos_y': rowId})
    return gears

# def iterateCols(grid: list[list[str]], top, bottom, left, right, pos_y):
#     nums = []
#     for row in range(top, bottom):
#             if left == len(grid[pos_y]):
#                 for cell in range(right, left):
#                     val = grid[row][cell]
#                     num = ''
#                     if val.isnumeric():
#                         # if * is in last column
#                         # scan only to the left <--
#                         last = cell - 1
#                         num += val
#                         # print("number: ", val)
#                         while grid[row][last].isnumeric():
#                             last_val = grid[row][last]
#                             # print("last num: ", grid[row][last])
#                             # print("last: ", last)
#                             num += last_val
#                             last = last - 1
#                         last = cell - 1
#                         # reverse string, because it scans backwards
#                         print(num[::-1])
#                         nums.append(num[::-1])
#             else:
#                 for cell in range(right, left+1):
#                     val = grid[row][cell]
#                     num = ''
#                     if val.isnumeric():
#                         # if * is in first column
#                         if right == 0:
#                             # print("right")
#                             # scan only to the right -->
#                             next = cell + 1
#                             num += val
#                             # print("number: ", val)
#                             while grid[row][next].isnumeric():
#                                 next_val = grid[row][next]
#                                 # print("next num: ", grid[row][next])
#                                 # print("next: ", next)
#                                 num += next_val
#                                 next = next + 1
#                             next = cell + 1
#                             print(num)
#                             nums.append(num)
#                         else:
#                             # if * is in the middle somewhere
#                             print("==MIDDLE==")
#                             last = cell - 1
#                             next = cell + 1
#                             num += val
#                             print("number: ", val)
#                             while grid[row][last].isnumeric():
#                                 # print("yes")
#                                 last_val = grid[row][last]
#                                 # print("last num: ", grid[row][last])
#                                 # print("last: ", last)
#                                 num += last_val
#                                 last = last - 1
#                             last = cell - 1
#                             print("rev: ", num[::-1])
#                             nums.append(num[::-1])
#                             num = ''
#                             # print("number: ", val) 
#                             # print("next: ", next, " len: ", len(grid[row]))
#                             while next < len(grid[row]) and grid[row][next].isnumeric():
#                                 print("next")
#                                 next_val = grid[row][next]
#                                 # print("next num: ", grid[row][next])
#                                 # print("next: ", next)
#                                 num += next_val
#                                 print("num: ", num)
#                                 next = next + 1
#                             next = cell + 1
#                             print("prop: ", num)
#                             nums.append(num) 
#     return nums


def iterateCols(grid: list[list[str]], top, bottom, left, right, pos_y):
    nums = []
    for row in range(top, bottom):
            # * is in leftmost column
            if left == len(grid[pos_y]):
                # print("leftmost")
                for cell in range(right, left):
                    val = grid[row][cell]
                    num = ''
                    last = cell - 1
                    if val.isnumeric():
                        print(val)
                        # get first digit
                        while grid[row][last].isnumeric():
                            last_cell = grid[row][last]
                            # print("last cell: ", last_cell)
                            last = last - 1

                        while grid[row][last+1].isnumeric():
                            num += grid[row][last+1]
                            last = last + 1
                        print("num: ", num)
                        nums.append(num)
                        last = cell - 1
                        pass
            else:
                # * is somewhere in between col 0 and col len - 1
                process=True
                for cell in range(right, left+1):
                    val = grid[row][cell]
                    num = ''
                    if process:
                        if val.isnumeric():
                            # if * is in rightmost column
                            if right == 0:
                                num += val
                                next = cell + 1
                                while grid[row][next].isnumeric():
                                    num += grid[row][next]
                                    next = next + 1 
                                next = cell + 1

                                # If next item is a digit, do not process it to avoid duplicates
                                if grid[row][next].isnumeric():
                                    process=False

                                print ("right!")
                                print("num: ", num)
                                nums.append(num)
                            elif right > 0:
                                last = cell - 1
                                 # get first digit
                                while grid[row][last].isnumeric():
                                    last_cell = grid[row][last]
                                    # print("last cell: ", last_cell)
                                    last = last - 1

                                while last + 1 < len(grid[row]) and grid[row][last+1].isnumeric():
                                    num += grid[row][last+1]
                                    last = last + 1
                                print("num: ", num)
                                nums.append(num)
                                last = cell - 1

                                if cell + 1 < len(grid[row]) and grid[row][cell+1].isnumeric():
                                    process=False

                            print(val)
    return nums

def drawBounds (grid: list[list[str]], pos_x: int, pos_y: int):
    if not grid[pos_y][pos_x] == "*":
        print("not valid")
        return
    top = 0
    bottom = 0 
    right = 0
    left = 0

    if pos_x > 0:
        right = pos_x - 1
    else: 
        right = pos_x

    if pos_x + 1 <= len(grid[pos_y]):
        left = pos_x + 1

    if pos_y > 0:
        top = pos_y - 1
    else:
        top = pos_y

    if pos_y + 1 < len(grid):
        bottom = pos_y + 1
    else:
        bottom = pos_y

    # Bounds aquired.    
    # now loop from top to bottom, right to left, and add everything in a list
    nums = []
    if bottom == len(grid):
        nums = iterateCols(grid, top, bottom, left, right, pos_y)
    else:
        nums = iterateCols(grid, top, bottom+1, left, right, pos_y) 

    return nums


# printCharAtPos(chr, 0, 0)
# nums = extractNums(chr)
# print(nums)
# goodNums = []
# for num in nums:
#     result = drawBounds(chr, num['pos_x'], num['pos_y'], num['lastId'])
#     if result != None:
#         goodNums.append(result)
#     print("num: ", num['num'], " result: ", result)

# total = 0
# for num in goodNums:
#     total += num

# print("total: ", total)

gears = extractGears(chr)
ratios = []
for gear in gears:
    result = drawBounds(chr, gear['pos_x'], gear['pos_y'])
    print("===result===")
    print(result, len(result))
    print("===end===")
    if len(result) == 2:
        fin = 1
        for res in result:
            fin *= int(res)
        print("Ratio: ", fin)
        ratios.append(fin)

total = 0
for ratio in ratios:
    total += ratio
print(total)




# print("good! ", drawBounds(chr, 0, 4, 2))
# print("bad!  ", drawBounds(chr, 5, 0, 7))