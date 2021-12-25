import heapq

HALL_ID = 4
HALL_LEN = 11
COSTS = [1, 10, 100, 1000]


def get_accessible_positions(pos):
    room_depth = (len(pos) - HALL_LEN) // 4
    room_pos_len = 4 * room_depth
    num_positions = room_pos_len + HALL_LEN

    def get_room_id(idx):
        nonlocal room_pos_len, num_positions, room_depth
        if idx >= room_pos_len and idx < num_positions:
            return HALL_ID
        elif idx < room_pos_len:
            return idx // room_depth
        else:
            return -1

    def get_unit_idx(c):
        if c in "abcd":
            return ord(c) - ord("a")
        elif c in "ABCD":
            return 4 + ord(c) - ord("A")
        else:
            return -1

    def get_hallway_exit_idx(room_id):
        nonlocal room_pos_len
        return room_pos_len + room_id * 2 + 2

    def is_hallway_exit(idx):
        nonlocal room_pos_len
        if idx < room_pos_len + 2 or idx > room_pos_len + 8:
            return False
        return (idx - room_pos_len - 2) % 2 == 0

    for i in range(num_positions):
        c = pos[i]
        if c == ".":
            continue
        room_id = get_room_id(i)
        unit_idx = get_unit_idx(c)
        if room_id != HALL_ID and not c.islower():
            blocked = False
            for j in range(i % room_depth):
                if pos[i - j - 1] != ".":
                    blocked = True
                    break
            if blocked:
                # blocked at the bottom of the room
                continue

            hallway_exit_idx = get_hallway_exit_idx(room_id)

            # move into hallway, to the left
            k = hallway_exit_idx - 1
            j = 1
            while k >= room_pos_len and pos[k] == ".":
                if not is_hallway_exit(k):
                    cost = COSTS[unit_idx % 4] * (i % room_depth + 1 + j)
                    pos1 = list(pos)
                    pos1[i] = "."
                    pos1[k] = c.lower()
                    yield cost, "".join(pos1)
                k -= 1
                j += 1

            # move into hallway, to the right
            k = hallway_exit_idx + 1
            j = 1
            while k < num_positions and pos[k] == ".":
                if not is_hallway_exit(k):
                    cost = COSTS[unit_idx % 4] * (i % room_depth + 1 + j)
                    pos1 = list(pos)
                    pos1[i] = "."
                    pos1[k] = c.lower()
                    yield cost, "".join(pos1)
                k += 1
                j += 1
        elif room_id == HALL_ID:
            # try move into our room
            room_id = unit_idx % 4
            offs_in_room = -1
            for j in range(room_depth):
                if pos[room_id * room_depth + j] == ".":
                    offs_in_room += 1
                else:
                    break
            if offs_in_room == -1:
                continue

            blocked = False
            for j in range(offs_in_room + 1, room_depth):
                rc = pos[room_id * room_depth + j]
                if rc != "." and get_unit_idx(rc) % 4 != room_id:
                    blocked = True
                    break
            if blocked:
                # room still contains a unit of different type
                continue

            hallway_exit_idx = get_hallway_exit_idx(room_id)
            path_len = abs(i - hallway_exit_idx)
            if (
                hallway_exit_idx > i
                and pos[i + 1 : hallway_exit_idx + 1].count(".") != path_len
            ):
                # path is blocked (going from left)
                continue
            if hallway_exit_idx < i and pos[hallway_exit_idx:i].count(".") != path_len:
                # path is blocked (going from right)
                continue
            cost = COSTS[unit_idx % 4] * (path_len + 1 + offs_in_room)
            pos1 = list(pos)
            pos1[i] = "."
            pos1[room_id * room_depth + offs_in_room] = c
            yield cost, "".join(pos1)


def get_least_energy(pos):
    room_depth = (len(pos) - HALL_LEN) // 4
    goal = "".join(c * room_depth for c in "ABCD")

    def is_goal(pos):
        nonlocal goal
        return goal == pos[: len(goal)].upper()

    to_explore = [(0, pos)]
    scores = {pos: 0}

    while len(to_explore) > 0:
        score, pos = heapq.heappop(to_explore)
        if is_goal(pos):
            return scores[pos]
        for cost, cpos in get_accessible_positions(pos):
            cscore = score + cost
            if cpos not in scores or scores[cpos] > cscore:
                scores[cpos] = cscore
                heapq.heappush(to_explore, (cscore, cpos))


def insert_extra(lines):
    return lines[:3] + ["  #D#C#B#A#", "  #D#B#A#C#"] + lines[3:]


def parse_pos(lines):
    room_depth = len(lines) - 3
    res = ""
    for j in range(4):
        for i in range(room_depth):
            res += lines[i + 2][3 + j * 2]
    res += "..........."
    return res


def solution():
    lines = open("../data/23.txt").readlines()
    start_pos1 = parse_pos(lines)
    res1 = get_least_energy(start_pos1)
    start_pos2 = parse_pos(insert_extra(lines))
    res2 = get_least_energy(start_pos2)
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
