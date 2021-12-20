use crate::common;

type Board = Vec<Vec<u8>>;

const OFFS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn flash(board: &mut Board, x: i32, y: i32) -> usize {
    let (w, h) = (board[0].len() as i32, board.len() as i32);
    let mut res = 1;
    board[y as usize][x as usize] = 11;
    for (dx, dy) in OFFS {
        let (cx, cy) = (x + dx, y + dy);
        if cx >= 0 && cx < w && cy >= 0 && cy < h {
            board[cy as usize][cx as usize] += 1;
            if board[cy as usize][cx as usize] == 10 {
                res += flash(board, cx, cy);
            }
        }
    }
    res
}

fn step(board: &mut Board) -> usize {
    let mut num_flashes = 0;
    let (w, h) = (board[0].len(), board.len());
    for j in 0..h {
        for i in 0..w {
            board[j][i] += 1;
            if board[j][i] == 10 {
                num_flashes += flash(board, i as i32, j as i32);
            }
        }
    }
    for j in 0..h {
        for i in 0..w {
            if board[j][i] >= 10 {
                board[j][i] = 0;
            }
        }
    }
    num_flashes
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(11)).unwrap();
    let board: Board = lines
        .iter()
        .map(|x| x.as_bytes().iter().map(|c| c - '0' as u8).collect())
        .collect();
    let (w, h) = (board[0].len(), board.len());

    let mut res1 = 0;
    let mut board1 = board.clone();
    for _ in 0..100 {
        res1 += step(&mut board1);
    }

    let mut res2 = 0;
    let mut board2 = board.clone();
    loop {
        let num_flashes = step(&mut board2);
        res2 += 1;
        if num_flashes == w * h {
            break;
        }
    }
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
