def eval_prog(prog, inp):
    pos = 0
    reg = [0] * 4  # wxyz

    def get_val(v):
        nonlocal reg
        if v in "wxyz":
            return reg[ord(v) - ord("w")]
        return int(v)

    for instr in prog:
        op = instr[0]
        if op == "inp":
            if pos == len(inp):
                return None
            reg[ord(instr[1]) - ord("w")] = int(inp[pos])
            pos += 1
        elif op == "add":
            p1 = ord(instr[1]) - ord("w")
            reg[p1] += get_val(instr[2])
        elif op == "mul":
            p1 = ord(instr[1]) - ord("w")
            reg[p1] *= get_val(instr[2])
        elif op == "div":
            p1 = ord(instr[1]) - ord("w")
            v2 = get_val(instr[2])
            if v2 == 0:
                return None
            reg[p1] //= v2
        elif op == "mod":
            p1 = ord(instr[1]) - ord("w")
            v1 = reg[p1]
            v2 = get_val(instr[2])
            if v1 < 0 or v2 <= 0:
                return None
            reg[p1] = v1 % v2
        elif op == "eql":
            p1 = ord(instr[1]) - ord("w")
            v1 = reg[p1]
            v2 = get_val(instr[2])
            reg[p1] = int(v1 == v2)
        else:
            return None
    return reg


MODEL_NUM_LEN = 14

"""
Extra constraints extracted by meticulousluy reading the input code:
n2 = 9
n3 = 1
n5 = n4 - 2
n6 = n1 - 3
n10 = n9 + 5
n11 = n8 - 5
n12 = n7 + 4
n13 = n0 - 1
"""
ALLOWED_RANGES = [
    range(2, 10),
    range(4, 10),
    range(9, 10),
    range(1, 2),
    range(3, 10),
    (4, -2),
    (1, -3),
    range(1, 6),
    range(6, 10),
    range(1, 5),
    (9, 5),
    (8, -5),
    (7, 4),
    (0, -1),
]


def get_min_value(prog):
    res = ""
    for r in ALLOWED_RANGES:
        if isinstance(r, tuple):
            (pos, d) = r
            c = chr(ord(res[pos]) + d)
        else:
            c = chr(ord("0") + min(x for x in r))
        res += c
    w, x, y, z = eval_prog(prog, res)
    assert z == 0, "Incorrect minimum value"
    return res


def get_max_value(prog):
    res = ""
    for r in ALLOWED_RANGES:
        if isinstance(r, tuple):
            (pos, d) = r
            c = chr(ord(res[pos]) + d)
        else:
            c = chr(ord("0") + max(x for x in r))
        res += c
    w, x, y, z = eval_prog(prog, res)
    assert z == 0, "Incorrect maximum value"
    return res


def solution():
    prog = [x.strip().split(" ") for x in open("../data/24.txt").readlines()]
    res1 = get_max_value(prog)
    res2 = get_min_value(prog)

    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
