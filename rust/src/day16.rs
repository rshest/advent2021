use crate::common;

#[derive(Debug)]
enum PacketType {
    Sum = 0,
    Prod,
    Min,
    Max,
    Literal,
    Greater,
    Less,
    Equal,
}

#[derive(Debug)]
enum Payload {
    Literal(u64),
    Subpackets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u32,
    packet_type: PacketType,
    payload: Payload,
}

fn decode(packet: &str, pos: usize) -> (usize, Packet) {
    let version = u32::from_str_radix(&packet[pos..pos + 3], 2).unwrap();
    let packet_type = match u32::from_str_radix(&packet[pos + 3..pos + 6], 2).unwrap() {
        0 => PacketType::Sum,
        1 => PacketType::Prod,
        2 => PacketType::Min,
        3 => PacketType::Max,
        4 => PacketType::Literal,
        5 => PacketType::Greater,
        6 => PacketType::Less,
        7 => PacketType::Equal,
        _ => panic!("Invalid packet type"),
    };
    let mut cpos = pos + 6;
    let payload = match packet_type {
        PacketType::Literal => {
            let mut i: usize = 0;
            let mut bits: String = "".to_owned();
            loop {
                bits.push_str(&packet[cpos + i + 1..cpos + i + 5]);
                if &packet[cpos + i..cpos + i + 1] == "0" {
                    break;
                }
                i += 5;
            }
            cpos += i + 5;
            Payload::Literal(u64::from_str_radix(&bits, 2).unwrap())
        }
        _ => {
            let length_id = &packet[cpos..cpos + 1];
            cpos += 1;
            let mut subpackets = vec![];
            if length_id == "0" {
                let length_bits =
                    u32::from_str_radix(&packet[cpos..cpos + 15], 2).unwrap() as usize;
                cpos += 15;
                let start_pos = cpos;
                while cpos - start_pos < length_bits {
                    let (cpos1, subpacket) = decode(packet, cpos);
                    cpos = cpos1;
                    subpackets.push(subpacket);
                }
            } else {
                let num_sub_packets = u32::from_str_radix(&packet[cpos..cpos + 11], 2).unwrap();
                cpos += 11;
                for _ in 0..num_sub_packets {
                    let (cpos1, subpacket) = decode(packet, cpos);
                    cpos = cpos1;
                    subpackets.push(subpacket);
                }
            }
            Payload::Subpackets(subpackets)
        }
    };
    (
        cpos,
        Packet {
            version,
            packet_type,
            payload,
        },
    )
}

fn to_bin(data: &str) -> String {
    data.chars()
        .map(|c| {
            let dec = u8::from_str_radix(&c.to_string(), 16).unwrap();
            format!("{:04b}", dec)
        })
        .collect::<Vec<String>>()
        .join("")
}

fn get_version_sum(packet: &Packet) -> u32 {
    match &packet.payload {
        Payload::Literal(_) => packet.version,
        Payload::Subpackets(subpackets) => {
            subpackets.iter().map(|s| get_version_sum(&s)).sum::<u32>() + packet.version
        }
    }
}

fn eval_packet(packet: &Packet) -> u64 {
    match &packet.payload {
        Payload::Literal(val) => *val,
        Payload::Subpackets(sub) => match &packet.packet_type {
            PacketType::Sum => sub.iter().map(eval_packet).sum(),
            PacketType::Prod => sub.iter().map(eval_packet).product(),
            PacketType::Min => sub.iter().map(eval_packet).min().unwrap(),
            PacketType::Max => sub.iter().map(eval_packet).max().unwrap(),
            PacketType::Greater => (eval_packet(&sub[0]) > eval_packet(&sub[1])) as u64,
            PacketType::Less => (eval_packet(&sub[0]) < eval_packet(&sub[1])) as u64,
            PacketType::Equal => (eval_packet(&sub[0]) == eval_packet(&sub[1])) as u64,
            _ => panic!("Inconsistent packet type"),
        },
    }
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(16)).unwrap();
    let pdata = to_bin(&lines[0].trim());
    let (_, packet) = decode(&pdata, 0);
    let res1 = get_version_sum(&packet);
    let res2 = eval_packet(&packet);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
