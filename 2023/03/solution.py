
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

def iterateCols(grid: list[list[str]], top, bottom, left, right, pos_y):
    num = ''
    success = False
    for row in range(top, bottom):
            if left == len(grid[pos_y]):
                for cell in range(right, left):
                    val = grid[row][cell]
                    if val.isnumeric():
                        num += val
                    if not val.isnumeric() and not val == '.':
                        success = True
            else:
                for cell in range(right, left+1):
                    val = grid[row][cell]
                    if val.isnumeric():
                        num += val
                    if not val.isnumeric() and not val == '.':
                        success = True

    if success:
        # print(num)
        return [int(num), True]
    else:
        return [None, False]

def drawBounds (grid: list[list[str]], pos_x: int, pos_y: int, lastId: int):
    if not grid[pos_y][pos_x].isnumeric():
        print("not valid")
        return
    #[
    #   Start at the position of the first digit (firstID)
    #   Check if there is a row above and below the first character, and check if there is a column before the first character and after the final character ( the final character is either the final digit or the '.' after it)
    #   
    #   Scan forwards till you hit the first '.' character, this position is the width of the rect
    #   Height of the rect is y+1 ( if not the final line )
    #    
    # ]

    top = 0
    bottom = 0 
    right = 0
    left = 0

    if pos_x > 0:
        right = pos_x - 1
        # print("right", right)
    else: 
        right = pos_x
        # print("right", right)

    if pos_y > 0:
        top = pos_y - 1
        # print("top", top)
    else:
        top = pos_y
        # print("top", top)

    if lastId + 1 < len(grid[pos_y]):
        left = lastId + 1
        # print("left", left)
    else:
        left = lastId
        # print("left", left)

    if pos_y + 1 < len(grid):
        bottom = pos_y + 1
        # print("bottom", bottom)
    else:
        bottom = pos_y
        # print("bottom", bottom)

    # Bounds aquired.    
    # now loop from top to bottom, right to left, and add everything in a list
    if bottom == len(grid):
        [num, success] = iterateCols(grid, top, bottom, left, right, pos_y)
    else:
        [num, success] = iterateCols(grid, top, bottom+1, left, right, pos_y)
    
    if success:
        return num


printCharAtPos(chr, 0, 0)
nums = extractNums(chr)
print(nums)
goodNums = []
for num in nums:
    result = drawBounds(chr, num['pos_x'], num['pos_y'], num['lastId'])
    if result != None:
        goodNums.append(result)
    print("num: ", num['num'], " result: ", result)

total = 0
for num in goodNums:
    total += num

print("total: ", total)

# print("good! ", drawBounds(chr, 0, 4, 2))
# print("bad!  ", drawBounds(chr, 5, 0, 7))