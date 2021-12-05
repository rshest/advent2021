from collections import defaultdict

def sgn(x):
    if x == 0:
        return 0
    return 1 if x > 0 else -1

def stroke(a, b, ptmap, skip_diagonals):
    (x1, y1) = a
    (x2, y2) = b
    dx = sgn(x2 - x1)
    dy = sgn(y2 - y1)

    if skip_diagonals and dx != 0 and dy != 0:
        return

    cx, cy = x1, y1
    while True:
        ptmap[(cx, cy)] += 1
        if cx == x2 and cy == y2:
            break
        cx += dx
        cy += dy

def find_num_overlaps(points, skip_diagonals):
    ptmap = defaultdict(int)
    for a, b in points:
        stroke(a, b, ptmap, skip_diagonals)
    return sum(ptmap[x] >= 2 for x in ptmap)

def parse_pt(pt):
    (x, y) = [int(c) for c in pt.split(',')]
    return (x, y)

def solution():
    points = [list(map(parse_pt, x.strip().split(' -> ')))
            for x in open('../data/05.txt').readlines()]
    res1 = find_num_overlaps(points, True)
    res2 = find_num_overlaps(points, False)
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
