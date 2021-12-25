import bisect

def parse(line):
    def parse_range(s):
        parts = s[2:].split("..")
        return (int(parts[0]), int(parts[1]))
    parts1 = line.split(" ")
    op = 1 if parts1[0] == "on" else 0
    p = [parse_range(p) for p in parts1[1].split(",")]
    return (op, p[0], p[1], p[2])

def intersects(cube1, cube2):
    return cube1[0][0] < cube2[0][1] and cube1[0][1] > cube2[0][0] and \
        cube1[1][0] < cube2[1][1] and cube1[1][1] > cube2[1][0] and \
        cube1[2][0] < cube2[2][1] and cube1[2][1] > cube2[2][0]

def get_lit_volume(ops, bound_region=None, log_progress=False):
    def get_coords_axis(ops, axis_id):
        c1 = [op[axis_id + 1][0] for op in ops]
        c2 = [op[axis_id + 1][1] + 1 for op in ops]
        return sorted(list(set(c1 + c2)))

    sections = [get_coords_axis(ops, i) for i in range(3)]
    grid = set()

    vol = 0
    for idx, (op, (x1, x2), (y1, y2), (z1, z2)) in enumerate(ops):
        if log_progress:
            print(f"op {idx} out of {len(ops)}, grid size: {len(grid)}")
        if bound_region != None:
            region = ((x1, x2), (y1, y2), (z1, z2))
            if not intersects(bound_region, region):
                continue
        i1 = bisect.bisect_left(sections[0], x1)
        j1 = bisect.bisect_left(sections[1], y1)
        k1 = bisect.bisect_left(sections[2], z1)
        k = k1
        while sections[2][k] <= z2:
            j = j1
            while sections[1][j] <= y2:
                i = i1
                while sections[0][i] <= x2:
                    pos = (i, j, k)
                    is_lit = pos in grid
                    if (op == 1) ^ is_lit:
                        dvol = (sections[0][i + 1] - sections[0][i]) * \
                            (sections[1][j + 1] - sections[1][j]) * \
                            (sections[2][k + 1] - sections[2][k])
                        if op == 1:
                            grid.add(pos)
                            vol += dvol
                        else:
                            grid.remove(pos)
                            vol -= dvol
                    i += 1
                j += 1
            k += 1
    return vol

def solution():
    REGION0 = ((-50, 50), (-50, 50), (-50, 50))
    ops = [parse(x.strip()) for x in open('../data/22.txt').readlines()]
    res1 = get_lit_volume(ops, REGION0)
    res2 = get_lit_volume(ops, None)

    print(f"Answer 1: {res1}\nAnswer 2: {int(res2)}")
