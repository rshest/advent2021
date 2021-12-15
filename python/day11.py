# DAY11

OFFS = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]

def flash(board, x, y):
    res = 1
    board[y][x] == 11
    for dx, dy in OFFS:
        cx, cy = x + dx, y + dy
        if cx >= 0 and cx < len(board[0]) and cy >= 0 and cy < len(board):
            board[cy][cx] += 1
            if board[cy][cx] == 10:
                res += flash(board, cx, cy)
    return res

def step(board):
    num_flashes = 0
    for j in range(len(board)):
        for i in range(len(board[0])):
            board[j][i] += 1
            if board[j][i] == 10:
                num_flashes += flash(board, i, j)
    for j in range(len(board)):
        for i in range(len(board[0])):
            if board[j][i] >= 10:
                board[j][i] = 0
    return num_flashes

def solution():
    board = [list(map(int, list(s))) for s in open("../data/11.txt").read().split()]

    res1 = 0
    board1 = [row[:] for row in board]
    for i in range(100):
        res1 += step(board1)

    res2 = 0
    while True:
        num_flashes = step(board)
        res2 += 1
        if num_flashes == len(board) * len(board[0]):
            break

    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
