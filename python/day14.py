from collections import defaultdict

def step(hist, rules):
    res = defaultdict(int)
    for s, cnt in hist.items():
        if s in rules:
            c = rules[s]
            res[s[0] + c] += cnt
            res[c + s[1]] += cnt
        else:
            res[s] = cnt
    return res

def get_pairs_hist(pattern):
    res = defaultdict(int)
    for i in range(1, len(pattern)):
        res[pattern[i - 1: i + 1]] += 1
    return res

def eval_pattern(start_pattern, rules, steps):
    hist = get_pairs_hist(start_pattern)
    for i in range(steps):
        hist = step(hist, rules)

    char_hist = defaultdict(int)
    for s, cnt in hist.items():
        char_hist[s[0]] += cnt
    char_hist[start_pattern[-1]] += 1

    counts = sorted((char_hist[c], c) for c in char_hist.keys())
    return counts[-1][0] - counts[0][0]

def solution():
    lines = open('../data/14.txt').readlines()
    start_pattern = lines[0].strip()
    rules = {x[0]: x[1] for x in map(lambda s: s.strip().split(" -> "), lines[2:])}

    res1 = eval_pattern(start_pattern, rules, 10)
    res2 = eval_pattern(start_pattern, rules, 40)
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
