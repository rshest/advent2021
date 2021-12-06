def sum_spawned(nums, total_days):
    spawned = {}

    def get_n_spawned(n, days):
        if days <= n:
            return 0
        if (n, days) in spawned:
            return spawned[(n, days)]
        res = (days - n - 1) // 7 + 1

        for i in range(res):
            res += get_n_spawned(8, days - n - i * 7 - 1)
        spawned[(n, days)] = res
        return res

    return sum(get_n_spawned(x, total_days) for x in nums) + len(nums)

def solution():
    nums = [int(x) for x in open('../data/06.txt').read().split(',')]
    print(f"Answer 1: {sum_spawned(nums, 80)}\nAnswer 2: {sum_spawned(nums, 256)}")
