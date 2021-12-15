CHARS = 'abcdefg'
DIGITS = ['abcefg', 'cf', 'acdeg', 'acdfg', 'bcdf',
         'abdfg', 'abdefg', 'acf', 'abcdefg', 'abcdfg']
DIGITS_MAP = {x: i for i, x in enumerate(DIGITS)}
NUM_SEGMENTS = 7

def normalize_mapping(mapping):
    changed = True
    while changed:
        changed = False
        for i, c in enumerate(mapping):
            if sum(c) == 1:
                p = c.index(1)
                for j in range(NUM_SEGMENTS):
                    if j != i and mapping[j][p] == 1:
                        mapping[j][p] = 0
                        changed = True

def is_valid_mapping(mapping):
    if not all(sum(c) == 1 for c in mapping):
        return False
    for i in range(NUM_SEGMENTS):
        s = [mapping[j][i] for j in range(NUM_SEGMENTS)]
        if sum(s) != 1:
            return False
    return True

def get_mapping(mapping, combs, pos):
    if pos == len(combs):
        normalize_mapping(mapping)
        return mapping if is_valid_mapping(mapping) else None
    comb = combs[pos]
    for digit in DIGITS:
        if len(digit) != len(comb):
            continue
        mapping1 = [x[:] for x in mapping]
        for d in [x for x in CHARS if x not in digit]:
            for c in comb:
                mapping1[ord(c) - ord('a')][ord(d) - ord('a')] = 0
        res_mapping = get_mapping(mapping1, combs, pos + 1)
        if res_mapping != None:
            return res_mapping
    return None

def decode_digit(m, combs):
    res = 0
    for comb in combs:
        s = "".join(sorted(chr(m[ord(c) - ord('a')].index(1) + ord('a')) for c in comb))
        if s not in DIGITS_MAP:
            return None
        digit = DIGITS_MAP[s]
        res = res * 10 + digit
    return res

def decode_sum(inp):
    res = 0
    for combs, to_decode in inp:
        mapping = [[1] * NUM_SEGMENTS] * NUM_SEGMENTS
        combs.sort(key=len)
        mapping = get_mapping(mapping, combs, 0)
        res += decode_digit(mapping, to_decode)
    return res

def parse(line):
    return line.strip().split(" ")

def solution():
    data = [list(map(parse, x.strip().split("|")))
        for x in open('../data/08.txt').readlines()]

    res1 = sum(len(x) in [2, 3, 4, 7] for a, b in data for x in b)
    res2 = decode_sum(data)
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
