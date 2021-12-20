def is_need_invert(lookup):
    return lookup[0] == 1 and lookup[-1] == 0

def get_ext(pixels):
    minx, miny = 0, 0
    maxx, maxy = 0, 0
    for x, y in pixels:
        minx = min(minx, x)
        maxx = max(maxx, x)
        miny = min(miny, y)
        maxy = max(maxy, y)
    return ((minx, maxx), (miny, maxy))

def step(pixels, lookup, is_lit_set=True):
    need_invert = is_need_invert(lookup)
    lit_val = 1 if need_invert ^ is_lit_set else 0
    (minx, maxx), (miny, maxy) = get_ext(pixels)
    res = set()
    for j in range(miny - 1, maxy + 2):
        for i in range(minx - 1, maxx + 2):
            mask = 0
            for dj in [-1, 0, 1]:
                for di in [-1, 0, 1]:
                    x, y = i + di, j + dj
                    bit = int((x, y) in pixels)
                    if not is_lit_set:
                        bit = 1 - bit
                    mask = mask * 2 + bit
            val = lookup[mask]
            if val == lit_val:
                res.add((i, j))
    return res

def eval_image(pixels, lookup, steps):
    need_invert = is_need_invert(lookup)
    for i in range(steps):
        is_lit_set = not need_invert or (i % 2 == 0)
        pixels = step(pixels, lookup, is_lit_set)
    return len(pixels)

def solution():
    lines = [x.strip() for x in open('../data/20.txt').readlines()]
    lookup = [int(c == '#') for c in list(lines[0])]
    pixels = set((i, j)
                for j, line in enumerate(lines[2:])
                for i, c in enumerate(line)
                if c == '#')

    res1 = eval_image(pixels, lookup, 2)
    res2 = eval_image(pixels, lookup, 50)

    print(f"Answer 1: {res1}\nAnswer 2: {int(res2)}")
