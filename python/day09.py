# DAY09
import math

OFFS = [(0, 1), (0, -1), (1, 0), (-1, 0)]

def find_minima(board):
    res = []
    for j, row in enumerate(board):
        for i, h in enumerate(row):
            is_min = True
            for dx, dy in OFFS:
                x, y = i + dx, j + dy
                if x >= 0 and x < len(row) and y >= 0 and y < len(board) and board[y][x] <= h:
                    is_min = False
                    break
            if is_min:
                res.append((i, j))
    return res

def fill_cell(board, bidx, pos, basin_idx):
    x, y = pos
    if x < 0 or x >= len(board[0]) or y < 0 or y >= len(board):
        return
    b = board[y][x]
    if b == 9:
        bidx[y][x] = 0
        return

    if bidx[y][x] >= 0:
        return
    bidx[y][x] = basin_idx
    h = board[y][x]
    for dx, dy in OFFS:
        x1, y1 = x + dx, y + dy
        fill_cell(board, bidx, (x1, y1), basin_idx)

def find_basins(board):
    w, h = len(board[0]), len(board)
    bidx = [[-1]*w for i in range(h)]
    num_basins = 0
    for j in range(h):
        for i in range(w):
            if bidx[j][i] == -1:
                fill_cell(board, bidx, (i, j), num_basins + 1)
                num_basins += 1
    return bidx

def solution():
    board = [list(map(int, list(x.strip()))) for x in open('../data/09.txt').readlines()]

    mins = find_minima(board)
    res1 = sum(map(lambda p: board[p[1]][p[0]] + 1, mins))

    basins = find_basins(board)
    max_basin = max(map(max, basins))

    basin_sizes = []
    for bidx in range(1, max_basin + 1):
        size = 0
        w, h = len(board[0]), len(board)
        for j in range(h):
            for i in range(w):
                if basins[j][i] == bidx:
                    size += 1
        basin_sizes.append(size)

    res2 = 1
    for x in sorted(basin_sizes, reverse=True)[:3]:
        res2 *= x
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
