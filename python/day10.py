CMAP = {"[": "]", "{": "}", "<": ">", "(": ")"}
SCORES1 = {")": 3, "]": 57, "}": 1197, ">": 25137}
SCORES2 = {")": 1, "]": 2, "}": 3, ">": 4}
CORRUPTED = 1
INCOMPLETE = 2

def find_match(line):
    chars = []
    for c in line:
        if c in "[({<":
            chars.append(c)
        else:
            current = chars.pop()
            if c != CMAP[current]:
                return (CORRUPTED, c)
    return (INCOMPLETE, "".join(CMAP[x] for x in chars[::-1]))

def score2(line):
    res = 0
    for c in line:
        res = res * 5 + SCORES2[c]
    return res

def solution():
    lines = [x.strip() for x in open('../data/10.txt').readlines()]
    matches = [find_match(line) for line in lines]
    res1 = sum(SCORES1[x] for (t, x) in matches if t == CORRUPTED)
    scores2 = sorted(score2(x) for (t, x) in matches if t == INCOMPLETE)
    res2 = scores2[len(scores2) // 2]
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
