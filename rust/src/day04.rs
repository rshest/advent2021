use crate::common;

type Board = Vec<Vec<i32>>;

fn is_full(board: &Board, pos: usize, is_vert: bool) -> bool {
    for i in 0..board.len() {
        let n = if is_vert {
            board[i][pos]
        } else {
            board[pos][i]
        };
        if n != -1 {
            return false;
        }
    }
    true
}

fn has_winner(board: &Board) -> bool {
    return (0..board.len()).any(|i| is_full(board, i, true) || is_full(board, i, false));
}

fn apply_num(board: &mut Board, num: i32) {
    for row in board {
        for i in 0..row.len() {
            if row[i] == num {
                row[i] = -1;
            }
        }
    }
}

fn run_simulation((nums, mut boards): (Vec<i32>, Vec<Board>)) -> Vec<i32> {
    let mut winners = vec![];
    let mut scores = vec![];
    for n in nums {
        for i in 0..boards.len() {
            if winners.contains(&i) {
                continue;
            }
            apply_num(&mut boards[i], n);
            if has_winner(&boards[i]) {
                let s: i32 = boards[i].iter().flatten().filter(|&x| *x != -1).sum();
                winners.push(i);
                scores.push(s * n);
            }
        }
    }
    scores
}

fn parse_game(lines: Vec<String>) -> (Vec<i32>, Vec<Board>) {
    let nums = lines[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut boards = vec![];
    let mut board = vec![];
    for line in &lines[2..] {
        if line.is_empty() {
            boards.push(board);
            board = vec![];
        } else {
            board.push(
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }
    boards.push(board);
    (nums, boards)
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(4)).unwrap();
    let scores = run_simulation(parse_game(lines));
    println!(
        "Answer 1: {}\nAnswer 2: {}",
        scores[0],
        scores.last().unwrap()
    );
}
