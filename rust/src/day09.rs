use crate::common;

const OFFS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn find_minima(board: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let bh = board.len() as i32;
    for (j, row) in board.iter().enumerate() {
        for (i, h) in row.iter().enumerate() {
            let bw = row.len() as i32;
            let is_min = OFFS.iter().all(|(dx, dy)| {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                x < 0 || x >= bw || y < 0 || y >= bh || board[y as usize][x as usize] > *h
            });
            if is_min {
                res.push((i, j));
            }
        }
    }
    res
}

fn fill_cell(board: &Vec<Vec<u8>>, bidx: &mut Vec<Vec<i32>>, pos: (usize, usize), basin_idx: i32) {
    let (x, y) = pos;
    if x >= board[0].len() || y >= board.len() {
        return;
    }
    let b = board[y][x];
    if b == 9 {
        bidx[y][x] = 0;
        return;
    }
    if bidx[y][x] >= 0 {
        return;
    }
    bidx[y][x] = basin_idx;
    for (dx, dy) in OFFS {
        let (x1, y1) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
        fill_cell(&board, bidx, (x1, y1), basin_idx);
    }
}

fn find_basins(board: &Vec<Vec<u8>>) -> Vec<Vec<i32>> {
    let w = board[0].len();
    let h = board.len();
    let mut bidx = vec![vec![-1; w]; h];
    let mut nbasins = 0;
    for j in 0..h {
        for i in 0..w {
            if bidx[j][i] == -1 {
                fill_cell(&board, &mut bidx, (i, j), nbasins + 1);
                nbasins += 1;
            }
        }
    }
    bidx
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(9)).unwrap();
    let board: Vec<Vec<u8>> = lines
        .iter()
        .map(|x| x.as_bytes().iter().map(|c| c - '0' as u8).collect())
        .collect();
    let mins = find_minima(&board);
    let res1: usize = mins.iter().map(|(x, y)| board[*y][*x] as usize + 1).sum();

    let basins = find_basins(&board);
    let max_basin = basins
        .iter()
        .map(|x| x.iter().max().unwrap())
        .max()
        .unwrap();

    let mut basin_sizes = vec![];
    let w = board[0].len();
    let h = board.len();
    for b in 1..max_basin + 1 {
        let mut size = 0;
        for j in 0..h {
            for i in 0..w {
                if basins[j][i] == b {
                    size += 1;
                }
            }
        }
        basin_sizes.push(size);
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    let res2: usize = basin_sizes[0..3].iter().product();
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
