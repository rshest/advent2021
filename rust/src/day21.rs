use crate::common;
use std::collections::HashMap;

fn game1(start_pos: &[u64]) -> u64 {
    let mut scores: Vec<u64> = vec![0, 0];
    let mut cur_pos = start_pos.to_owned();
    let mut cur_player = 0;
    let mut num_rolls = 0;
    let mut die_val = 1;
    while scores[0] < 1000 && scores[1] < 1000 {
        let steps = die_val + (die_val + 1) + (die_val + 2);
        die_val += 3;
        num_rolls += 1;
        let mut pos = cur_pos[cur_player] + steps;
        pos = (pos - 1) % 10 + 1;
        cur_pos[cur_player] = pos;
        scores[cur_player] += pos;
        if scores[cur_player] >= 1000 {
            break;
        }
        cur_player = 1 - cur_player;
    }
    scores.iter().map(|s| s * num_rolls * 3).min().unwrap()
}

type Mem = HashMap<((u64, u64), (u64, u64), usize), (u64, u64)>;

fn game2(start_pos: &[u64]) -> Vec<u64> {
    fn get_num_winning(
        cur_pos: &(u64, u64),
        scores: &(u64, u64),
        cur_player: usize,
        mem: &mut Mem,
    ) -> (u64, u64) {
        let key = (*cur_pos, *scores, cur_player);
        if mem.contains_key(&key) {
            return mem[&key];
        }
        if scores.0 >= 21 || scores.1 >= 21 {
            return if scores.0 > scores.1 { (1, 0) } else { (0, 1) };
        }

        let mut res = vec![0, 0];
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let steps = i + j + k;
                    let mut pos = steps
                        + if cur_player == 1 {
                            cur_pos.1
                        } else {
                            cur_pos.0
                        };
                    pos = (pos - 1) % 10 + 1;
                    let wins = match cur_player {
                        0 => {
                            get_num_winning(&(pos, cur_pos.1), &(scores.0 + pos, scores.1), 1, mem)
                        }
                        1 => {
                            get_num_winning(&(cur_pos.0, pos), &(scores.0, scores.1 + pos), 0, mem)
                        }
                        _ => panic!("Bad player index"),
                    };
                    res[0] += wins.0;
                    res[1] += wins.1;
                }
            }
        }
        let tres = (res[0], res[1]);
        mem.insert(key, tres);
        tres
    }
    let mut mem = HashMap::new();
    let res = get_num_winning(&(start_pos[0], start_pos[1]), &(0, 0), 0, &mut mem);
    vec![res.0, res.1]
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(21)).unwrap();
    let start_pos: Vec<u64> = lines
        .iter()
        .map(|line| line.split(' ').collect::<Vec<&str>>()[4].parse().unwrap())
        .collect();
    let res1 = game1(&start_pos);
    let res2 = *game2(&start_pos).iter().max().unwrap();
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
