import math

def parse_ext(data):
    parts = (data[len("target area: "):]).split(", ")
    for p in parts:
        v1, v2 = p[2:].split("..")
        yield (int(v1), int(v2))

def in_ext(ext, x, y):
    (x1, x2), (y1, y2) = ext
    return x >= x1 and x <= x2 and y >= y1 and y <= y2

def simulate(ext, vx, vy):
    x, y = 0, 0
    maxh = 0
    while True:
        maxh = max(y, maxh)
        if in_ext(ext, x, y):
            return maxh
        elif x > ext[0][1] or y < ext[1][0]:
            return None
        x += vx
        y += vy
        vx = max(0, vx - 1)
        vy -= 1

def solution():
    line = open("../data/17.txt").read().strip()
    ext = list(parse_ext(line))

    maxh = 0
    numv = 0

    # solve (n + 1) * n / 2 = ext.minx to find minimum vx
    min_vx = (int(math.sqrt(8 * ext[0][0])) - 1) // 2 + 1
    max_vx = ext[0][1]
    min_vy = -5 * min_vx
    max_vy = abs(ext[1][0])
    for vx in range(min_vx, max_vx + 1):
        for vy in range(min_vy, max_vy + 1):
            h = simulate(ext, vx, vy)
            if h != None:
                maxh = max(maxh, h)
                numv += 1
    print(f"Answer 1: {maxh}\nAnswer 2: {numv}")
