from collections import defaultdict

def traverse1(graph, pos, visited):
    res = 0
    for next_pos in graph[pos]:
        if next_pos == "end":
            res += 1
            continue
        if next_pos == "start":
            continue
        if next_pos.islower() and visited[next_pos] == 1:
            continue
        visited[next_pos] += 1
        res += traverse1(graph, next_pos, visited)
        visited[next_pos] -= 1
    return res

def traverse2(graph, pos, visited, visited_small=None):
    res = 0
    for next_pos in graph[pos]:
        if next_pos == "end":
            res += 1
            continue
        if next_pos == "start":
            continue
        if next_pos.islower():
            if visited[next_pos] == 1:
                if visited_small == None or visited_small == next_pos:
                    visited[next_pos] += 1
                    res += traverse2(graph, next_pos, visited, next_pos)
                    visited[next_pos] -= 1
                continue
            if visited[next_pos] == 2:
                continue
        visited[next_pos] += 1
        res += traverse2(graph, next_pos, visited, visited_small)
        visited[next_pos] -= 1
    return res

def solution():
    edges = [line.strip().split("-") for line in open("../data/12.txt").readlines()]

    verts = set()
    for a, b in edges:
        verts.add(a)
        verts.add(b)
    verts = list(verts)

    graph = defaultdict(list)
    for a, b in edges:
        graph[a].append(b)
        graph[b].append(a)

    visited = {v: 0 for v in graph}
    res1 = traverse1(graph, "start", visited)
    res2 = traverse2(graph, "start", visited)

    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
