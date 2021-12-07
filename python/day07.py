import statistics

def part1(nums):
    c = statistics.median(nums)
    return sum(abs(x - c) for x in nums)

# simple gradient descent
def fmin(p0, f, num_it=1000):
    EPS = 0.001
    p = p0
    for i in range(num_it):
        pr = f(p + EPS)
        pl = f(p - EPS)
        dp = (pr - pl)/(2 * EPS)
        p = p - dp * EPS
    return p

def part2(nums):
    def f(x):
        res = 0
        for n in nums:
            d = abs(x - n)
            res += d * (d + 1)/2
        return res
    minx = int(round(fmin(statistics.median(nums), f)))
    return f(minx)

def solution():
    nums = [int(x) for x in open('../data/07.txt').read().split(',')]
    print(f"Answer 1: {int(part1(nums))}\nAnswer 2: {int(part2(nums))}")
