from functools import reduce

def step1(pos, command):
    (x, depth) = pos
    (cmd, offs) = command
    if cmd == 'up':
        return (x, depth - offs)
    elif cmd == 'down':
        return (x, depth + offs)
    elif cmd == 'forward':
        return (x + offs, depth)

def step2(pos, command):
    (x, depth, aim) = pos
    (cmd, offs) = command
    if cmd == 'up':
        return (x, depth, aim - offs)
    elif cmd == 'down':
        return (x, depth, aim + offs)
    elif cmd == 'forward':
        return (x + offs, depth + aim * offs, aim)

def solution():
    commands = [(x.split()[0], int(x.split()[1]))
             for x in open("../data/02.txt").readlines()]
    pos1 = reduce(step1, commands, (0, 0))
    pos2 = reduce(step2, commands, (0, 0, 0))
    print(f'Answer 1: {pos1[0] * pos1[1]}\nAnswer 2: {pos2[0] * pos2[1]}')
