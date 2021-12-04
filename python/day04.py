def get_boards(data):
    cells = []
    for line in data:
        if line == '':
            yield cells
            cells = []
        else:
            cells.append([int(x) for x in line.split()])
    yield cells

def is_full(board, pos, is_vert):
    for i in range(len(board)):
        n = board[i][pos] if is_vert else board[pos][i]
        if n != -1:
            return False
    return True

def has_winner(board):
    return any(map(lambda i:
                       is_full(board, i, True) or is_full(board, i, False),
                   range(len(board))))

def apply_num(board, num):
    for row in board:
        for i, el in enumerate(row):
            if el == num:
                row[i] = -1

def run_simulation(nums, boards):
    winners, scores = [], []
    for n in nums:
        for i, board in enumerate(boards):
            if i in winners:
                continue
            apply_num(board, n)
            if has_winner(board):
                s = n * sum(x for row in board for x in row if x != -1)
                winners.append(i)
                scores.append(s)
    return scores

def solution():
    lines = [x.strip() for x in open('../data/04.txt').readlines()]
    nums = [int(x) for x in lines[0].split(",")]
    boards = list(get_boards(lines[2:]))
    scores = run_simulation(nums, boards)

    print(f"Answer 1: {scores[0]}\nAnswer 2: {scores[-1]}")
