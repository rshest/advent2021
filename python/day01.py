def sliding_window(arr, n):
    for i in range(0, len(arr) - n + 1):
        yield arr[i:i+n]

def solution():
    nums = [int(x) for x in open("../data/01.txt").readlines()]

    res1 = sum(x[1] > x[0] for x in sliding_window(nums, 2))
    res2 = sum(x[1] > x[0] for x in sliding_window(
        [sum(y) for y in sliding_window(nums, 3)],
        2))

    print(f'Answer 1: {res1}\nAnswer 2: {res2}')
