def step(board, east):
    w, h = len(board[0]), len(board)
    j = 0
    res = [[" "] * w for _ in range(h)]
    moved = False
    for j in range(h):
        for i in range(w):
            c = board[j][i]
            if res[j][i] == " ":
                res[j][i] = c
            if east and c == ">":
                i1 = (i + 1) % w
                if board[j][i1] == ".":
                    res[j][i1] = ">"
                    res[j][i] = "."
                    moved = True
            elif not east and c == "v":
                j1 = (j + 1) % h
                if board[j1][i] == ".":
                    res[j1][i] = "v"
                    res[j][i] = "."
                    moved = True
    return res, moved


def solution():
    board = [list(x.strip()) for x in open("../data/25.txt").readlines()]
    steps = 0
    while True:
        board, moved1 = step(board, True)
        board, moved2 = step(board, False)
        steps += 1
        if not moved1 and not moved2:
            break

    print(f"Answer 1: {steps}")
