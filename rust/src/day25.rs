use crate::common;

type Board = Vec<Vec<char>>;

enum StepDir {
    East,
    South,
}

const EAST_CHAR: char = '>';
const SOUTH_CHAR: char = 'v';
const EMPTY_CHAR: char = '.';

fn step(board: &Board, dir: StepDir) -> (bool, Board) {
    let (w, h) = (board[0].len(), board.len());
    let mut res = vec![vec![' '; w]; h];
    let mut moved = false;
    for j in 0..h {
        for i in 0..w {
            let c = board[j][i];
            if res[j][i] == ' ' {
                res[j][i] = c;
            }
            match dir {
                StepDir::East if c == EAST_CHAR => {
                    let i1 = (i + 1) % w;
                    if board[j][i1] == EMPTY_CHAR {
                        res[j][i1] = EAST_CHAR;
                        res[j][i] = EMPTY_CHAR;
                        moved = true;
                    }
                }
                StepDir::South if c == SOUTH_CHAR => {
                    let j1 = (j + 1) % h;
                    if board[j1][i] == EMPTY_CHAR {
                        res[j1][i] = SOUTH_CHAR;
                        res[j][i] = EMPTY_CHAR;
                        moved = true;
                    }
                }
                _ => {}
            }
        }
    }
    (moved, res)
}

fn eval_board(start_board: Board) -> usize {
    let mut steps = 0;
    let mut board = start_board;
    loop {
        let (moved1, board1) = step(&board, StepDir::East);
        let (moved2, board2) = step(&board1, StepDir::South);
        board = board2;
        steps += 1;
        if !moved1 && !moved2 {
            break;
        }
        if steps > 1000 {
            break;
        }
    }
    steps
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(25)).unwrap();
    let board: Board = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let res1 = eval_board(board);
    println!("Answer 1: {:?}", res1);
}
