def fold(coords, dir, pos):
    res = []
    if dir == 'x':
        for x, y in coords:
            if x < pos:
                res.append((x, y))
            else:
                res.append((2*pos - x, y))
    else:
        for x, y in coords:
            if y < pos:
                res.append((x, y))
            else:
                res.append((x, 2*pos - y))
    return res

def solution():
    clines, flines = open('../data/13.txt').read().split("\n\n")
    coords = [list(map(int, x.split(","))) for x in clines.split("\n")]
    fdirs = [x[11:].split("=") for x in flines.strip().split("\n")]

    coords = set(fold(coords, fdirs[0][0], int(fdirs[0][1])))
    res1 = len(set(coords))

    for dir, pos in fdirs[1:]:
        coords = set(fold(coords, dir, int(pos)))

    mx, my = max(coords)
    m = [['.']*(mx + 1) for i in range(my + 1)]
    for x, y in coords:
        m[y][x] = "#"
    res2 = "\n".join("".join(x) for x in m)
    print(f"Answer 1: {res1}\nAnswer 2:\n{res2}")
