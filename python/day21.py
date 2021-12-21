def parse(line):
    parts = line.split(" ")
    return int(parts[4])

def game1(start_pos):
    def roll():
        cur = 1
        while True:
            yield cur + (cur + 1) + (cur + 2)
            cur += 3

    scores = [0, 0]
    cur_pos = start_pos[:]
    cur_player = 0
    num_rolls = 0
    roll_gen = roll()
    while scores[0] < 1000 and scores[1] < 1000:
        steps = next(roll_gen)
        num_rolls += 1
        pos = cur_pos[cur_player] + steps
        pos = (pos - 1) % 10 + 1
        cur_pos[cur_player] = pos
        scores[cur_player] += pos
        if scores[cur_player] >= 1000:
            break
        cur_player = 1 - cur_player
    return min(s * num_rolls * 3 for s in scores)

def game2(start_pos):
    mem = {}
    def get_num_winning(cur_pos, scores, cur_player):
        nonlocal mem
        key = (cur_pos, scores, cur_player)
        if key in mem:
            return mem[key]
        if scores[0] >= 21 or scores[1] >= 21:
            if scores[0] > scores[1]:
                return (1, 0)
            else:
                return (0, 1)
        res = [0, 0]
        for i in range(1, 4):
            for j in range (1, 4):
                for k in range (1, 4):
                    steps = i + j + k
                    pos = cur_pos[cur_player] + steps
                    pos = (pos - 1) % 10 + 1
                    wins = get_num_winning((pos, cur_pos[1]),
                                            (scores[0] + pos, scores[1]),
                                            1) if cur_player == 0 else \
                    get_num_winning((cur_pos[0], pos),
                                            (scores[0], scores[1] + pos),
                                            0)
                    res[0] += wins[0]
                    res[1] += wins[1]

        mem[key] = res
        return res
    return get_num_winning(tuple(start_pos), (0, 0), 0)

def solution():
    positions = [parse(x.strip()) for x in open('../data/21.txt').readlines()]
    res1 = game1(positions)
    res2 = max(game2(positions))

    print(f"Answer 1: {res1}\nAnswer 2: {int(res2)}")
