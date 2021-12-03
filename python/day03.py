def get_most_least_common(nums, pos):
    n0 = 0
    n1 = 0
    for num in nums:
        n0 += num[pos] == '0'
        n1 += num[pos] == '1'
    return ('1', '0') if n1 >= n0 else ('0', '1')

def part1(nums):
    lc, mc = "", ""
    for i in range(0, len(nums[0])):
        (m, l) = get_most_least_common(nums, i)
        mc += m
        lc += l
    return int(mc, 2) * int(lc, 2)

def part2(nums):
    mnums = nums[:]
    lnums = nums[:]
    for i in range(0, len(nums[0])):
        if len(mnums) > 1:
            (m, l) = get_most_least_common(mnums, i)
            mnums = [x for x in mnums if x[i] == m]

        if len(lnums) > 1:
            (m, l) = get_most_least_common(lnums, i)
            lnums = [x for x in lnums if x[i] == l]

    return int(mnums[0], 2) * int(lnums[0], 2)

def solution():
    nums = [x.strip() for x in open('../data/03.txt').readlines()]
    print(f'Answer 1: {part1(nums)}\nAnswer 2: {part2(nums)}')
