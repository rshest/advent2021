use crate::common;

extern crate num;
use num::signum;
use std::collections::HashMap;
use itertools::Itertools;

type Point = (i32, i32);
type Line = (Point, Point);

fn parse_pt(pt: &str) -> Point {
    pt.split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .next()
        .unwrap()
}

fn parse_line(line: &str) -> Line {
    line.split(" -> ")
        .map(parse_pt)
        .tuple_windows()
        .next()
        .unwrap()
}

fn stroke(
    (x1, y1): &Point,
    (x2, y2): &Point,
    ptmap: &mut HashMap<Point, u32>,
    skip_diagonals: bool,
) {
    let dx = signum(x2 - x1);
    let dy = signum(y2 - y1);
    if skip_diagonals && dx != 0 && dy != 0 {
        return;
    }
    let mut cx = *x1;
    let mut cy = *y1;
    loop {
        *ptmap.entry((cx, cy)).or_insert(0) += 1;
        if cx == *x2 && cy == *y2 {
            break;
        }
        cx += dx;
        cy += dy;
    }
}

fn find_num_overlaps(lines: &Vec<Line>, skip_diagonals: bool) -> usize {
    let mut ptmap: HashMap<Point, u32> = HashMap::new();
    for (a, b) in lines {
        stroke(a, b, &mut ptmap, skip_diagonals);
    }
    ptmap.values().filter(|&x| *x > 1).count()
}

pub(crate) fn solution() {
    let lines: Vec<Line> = common::read_lines(&common::data_file(5))
        .unwrap()
        .iter()
        .map(|s| parse_line(&s))
        .collect();
    let res1 = find_num_overlaps(&lines, true);
    let res2 = find_num_overlaps(&lines, false);
    println!("Answer 1: {}\nAnswer 2: {}", res1, res2);
}
