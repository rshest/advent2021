use crate::common;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    score: usize,
    pos: (i32, i32),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const OFFS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get_risk(board: &Vec<Vec<u8>>, x: i32, y: i32) -> usize {
    let (w, h) = (board[0].len() as i32, board.len() as i32);
    let mut res = board[(y % h) as usize][(x % w) as usize] as usize;
    res += (x / w + y / h) as usize;
    if res > 9 {
        res = res % 10 + 1;
    }
    res
}

fn get_min_risk(board: &Vec<Vec<u8>>, pos: (i32, i32), goal: (i32, i32)) -> usize {
    let mut to_explore = BinaryHeap::new();
    to_explore.push(Node { score: 0, pos: pos });

    let mut scores = HashMap::new();
    scores.insert(pos, 0usize);

    while let Some(Node { score, pos }) = to_explore.pop() {
        let (x, y) = pos;
        if pos == goal {
            return scores[&pos];
        }
        for (dx, dy) in OFFS {
            let (cx, cy) = (x + dx, y + dy);
            if cx < 0 || cy < 0 || cx > goal.0 || cy > goal.1 {
                continue;
            }
            let cpos = (cx, cy);
            let cscore = score + get_risk(&board, cx, cy);
            if !scores.contains_key(&cpos) || scores[&cpos] > cscore {
                scores.insert(cpos, cscore);
                to_explore.push(Node {
                    score: cscore,
                    pos: cpos,
                });
            }
        }
    }
    0
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(15)).unwrap();
    let board: Vec<Vec<u8>> = lines
        .iter()
        .map(|x| x.as_bytes().iter().map(|c| c - '0' as u8).collect())
        .collect();
    let (w, h) = (board[0].len() as i32, board.len() as i32);
    let res1 = get_min_risk(&board, (0, 0), (w - 1, h - 1));
    let res2 = get_min_risk(&board, (0, 0), (w * 5 - 1, h * 5 - 1));
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
