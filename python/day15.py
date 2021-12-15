import heapq

def get_risk(board, x, y):
    w, h = len(board[0]), len(board)
    res = board[y % h][x % w]
    res += x // w + y // h
    if res > 9:
        res = res % 10 + 1
    return res

def get_min_risk(board, pos, goal):
    w, h = len(board[0]), len(board)
    to_explore = [(0, pos)]
    scores = {pos: 0}

    while len(to_explore) > 0:
        score, pos = heapq.heappop(to_explore)
        x, y = pos
        if x == goal[0] and y == goal[1]:
            return scores[pos]

        for dx, dy in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
            cx, cy = x + dx, y + dy
            if cx < 0 or cy < 0 or cx > goal[0] or cy > goal[1]:
                continue
            cpos = (cx, cy)
            cscore = score + get_risk(board, cx, cy)
            if cpos not in scores or scores[cpos] > cscore:
                scores[cpos] = cscore
                heapq.heappush(to_explore, (cscore, cpos))

def solution():
    board = [list(map(int, list(s))) for s in open("../data/15.txt").read().split()]
    w, h = len(board[0]), len(board)
    res1 = get_min_risk(board, (0, 0), (w - 1, h - 1))
    res2 = get_min_risk(board, (0, 0), (w * 5 - 1, h * 5 - 1))
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
