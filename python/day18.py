import json
import copy

OP_NONE = 0
OP_SPLIT = 1
OP_EXPLODE = 2
OP_SPLIT_EXPLODE = 3
MAX_DEPTH = 5

def parse_num(line):
   return json.loads(line)

def reduce(num, can_split=True):
    prev_regular = None
    next_regular_inc = 0
    current_op = OP_NONE

    def rec(num, depth=0):
        nonlocal prev_regular, next_regular_inc, current_op
        for i, n in enumerate(num):
            if current_op != OP_NONE:
                if not isinstance(n, list):
                    num[i] += next_regular_inc
                    next_regular_inc = 0
                    return
                else:
                    rec(n, depth + 1)
            elif isinstance(n, list):
                if depth >= 3:
                    if prev_regular != None:
                        prev_regular[0][prev_regular[1]] += n[0]
                    next_regular_inc = n[1]
                    num[i] = 0
                    current_op = OP_EXPLODE
                else:
                    rec(n, depth + 1)
            else:
                if can_split and n >= 10:
                    if depth >= 3:
                        if prev_regular != None:
                            prev_regular[0][prev_regular[1]] += n // 2
                        next_regular_inc = (n + 1) // 2
                        num[i] = 0
                        current_op = OP_SPLIT_EXPLODE
                        continue
                    else:
                        num[i] = [n // 2, (n + 1) // 2]
                        current_op = OP_SPLIT
                        return
                prev_regular = (num, i)

    rec(num, 0)
    return current_op

def get_magnitude(num):
    if not isinstance(num, list):
        return num
    return 3 * get_magnitude(num[0]) + 2 * get_magnitude(num[1])

def eval_nums(nums):
    res = nums[0]
    for i in range(1, len(nums)):
        res = [res, nums[i]]
        while True:
            op = reduce(res, False)
            if op == OP_EXPLODE:
                continue
            op = reduce(res, True)
            if op == OP_NONE:
                break
    return res

def solution():
    lines = open('../data/18.txt').readlines()
    nums = [parse_num(line) for line in lines]

    nums1 = copy.deepcopy(nums)
    num1 = eval_nums(nums1)
    res1 = get_magnitude(num1)

    res2 = 0
    maxnum = None
    for i in range(len(nums)):
        for j in range(0, len(nums)):
            if i == j:
                continue
            num = eval_nums([copy.deepcopy(nums[i]), copy.deepcopy(nums[j])])
            mag = get_magnitude(num)
            if res2 < mag:
                res2 = mag
                maxnum = (i, j)

    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
