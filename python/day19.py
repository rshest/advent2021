import numpy as np
import math

SCANNER_DIST = 1000
MIN_POINT_MATCHES = 12
MIN_DIR_MATCHES = math.factorial(MIN_POINT_MATCHES) // 2 // math.factorial(MIN_POINT_MATCHES - 2)

def enum_transforms():
    for i in range(3):
        for sign1 in [1, -1]:
            for j in range(3):
                if i == j:
                    continue
                for sign2 in [1, -1]:
                    tm = np.zeros((3, 3))
                    tm[0][i] = sign1
                    tm[1][j] = sign2
                    tm[2] = np.cross(tm[0], tm[1])
                    yield tm

TRANSFORMS = list(enum_transforms())

def get_offset(coords1, coords2, dirs1, dirs2):
    common_dirs = set(dirs1.keys()).intersection(set(dirs2.keys()))
    if len(common_dirs) < MIN_DIR_MATCHES:
        return []

    matching_dir = next(iter(common_dirs))
    c1 = coords1[dirs1[matching_dir][0]]
    pts1 = set(tuple(p) for p in coords1)

    for i in [0, 1]:
        c2 = coords2[dirs2[matching_dir][i]]
        offs = np.subtract(c2, c1)
        pts2 = set(tuple(np.subtract(c, offs)) for c in coords2)
        if len(pts2.intersection(pts1)) >= MIN_POINT_MATCHES:
            return offs
    return []

def get_scanner_positions(data):
    res = [np.zeros(3) for _ in range(len(data))]
    found = set([0])
    to_find = set(range(1, len(data)))
    non_overlapping = set()

    def compute_dirs(coords):
        return {tuple(np.abs(np.subtract(p2, p1))):
                (i, j) for i, p1 in enumerate(coords)
                for j, p2 in enumerate(coords)}

    # precompute both transformed coordinates and directions
    dirs = {}
    coords = {}
    for i in range(len(data)):
        for k, tm in enumerate(TRANSFORMS):
            coords_tm = [np.dot(tm, c) for c in data[i]]
            coords[(i, k)] = coords_tm
            dirs[(i, k)] = compute_dirs(coords_tm)

    def test_pair(i, j):
        nonlocal found, to_find, res, coords, dirs, data
        if (i, j) in non_overlapping:
            return False
        for k, tm in enumerate(TRANSFORMS):
            offs = get_offset(data[j], coords[(i, k)], dirs[(j, 0)], dirs[(i, k)])
            if len(offs) == 0:
                continue
            data[i] = [np.subtract(np.dot(tm, pt), offs) for pt in coords[(i, 0)]]
            dirs[(i, 0)] = compute_dirs(data[i])
            found.add(i)
            to_find.remove(i)
            res[i] = -offs
            return True
        non_overlapping.add((i, j))
        return False

    while len(to_find) > 0:
        pairs = ((i, j) for i in to_find for j in found)
        for (i, j) in pairs:
            if test_pair(i, j):
                break
    return res

def parse(data):
    lines = data.strip().split("\n")
    coords = [list(map(int, s.strip().split(","))) for s in lines[1:]]
    return coords

def manhattan_dist(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1]) + abs(a[2] - b[2])

def solution():
    data = [parse(x) for x in open('../data/19.txt').read().split("\n\n")]

    scanner_pos = [tuple(x) for x in get_scanner_positions(data)]
    res1 = len(set(tuple(x) for y in data for x in y))

    res2 = 0
    for i in range(0, len(scanner_pos)):
        for j in range(i + 1, len(scanner_pos)):
            res2 = max(res2, manhattan_dist(scanner_pos[i], scanner_pos[j]))
    print(f"Answer 1: {res1}\nAnswer 2: {int(res2)}")
