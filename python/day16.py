LITERAL_TYPE = 4

def to_bin(data):
    s = bin(int(data, 16))[2:]
    return '0' * (len(data) * 4 - len(s)) + s

def decode(packet, pos=0):
    v = int(packet[pos:pos + 3], 2)
    t = int(packet[pos + 3:pos + 6], 2)
    pos += 6
    if t == LITERAL_TYPE:
        i = 0
        res = ''
        while True:
            res += packet[pos + i + 1 : pos + i + 5]
            if packet[pos + i] == '0':
                break
            i += 5
        return pos + i + 5, (v, t, int(res, 2))
    else:
        length_id = packet[pos]
        pos += 1
        subpackets = []
        if length_id == '0':
            length_bits = int(packet[pos : pos + 15], 2)
            pos += 15
            start_pos = pos
            while pos - start_pos < length_bits:
                pos, subpacket = decode(packet, pos)
                subpackets.append(subpacket)
        else:
            num_sub_packets = int(packet[pos : pos + 11], 2)
            pos += 11
            for i in range(num_sub_packets):
                pos, subpacket = decode(packet, pos)
                subpackets.append(subpacket)
        return pos, (v, t, subpackets)

def get_version_sum(decoded_packet):
    v, _, sub = decoded_packet
    res = v
    if isinstance(sub, list):
        for s in sub:
            res += get_version_sum(s)
    return res

def eval_packet(decoded_packet):
    v, t, sub = decoded_packet
    if t == 0:
        return sum(eval_packet(p) for p in sub)
    elif t == 1:
        res = 1
        for p in sub:
            res *= eval_packet(p)
        return res
    elif t == 2:
        return min(eval_packet(p) for p in sub)
    elif t == 3:
        return max(eval_packet(p) for p in sub)
    elif t == 5:
        return int(eval_packet(sub[0]) > eval_packet(sub[1]))
    elif t == 6:
        return int(eval_packet(sub[0]) < eval_packet(sub[1]))
    elif t == 7:
        return int(eval_packet(sub[0]) == eval_packet(sub[1]))
    else:
        return sub

def solution():
    data = open("../data/16.txt").read().strip()
    packet = to_bin(data)
    _, dp = decode(packet)
    res1 = get_version_sum(dp)
    res2 = eval_packet(dp)
    print(f"Answer 1: {res1}\nAnswer 2: {res2}")
