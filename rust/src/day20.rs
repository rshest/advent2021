use crate::common;
use std::collections::HashSet;

type Pixels = HashSet<(i32, i32)>;
type Lookup = Vec<u8>;

fn is_need_invert(lookup: &Lookup) -> bool {
    lookup[0] == 1 && *lookup.last().unwrap() == 0
}

fn get_ext(pixels: &Pixels) -> ((i32, i32), (i32, i32)) {
    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;
    for (x, y) in pixels {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }
    ((minx, maxx), (miny, maxy))
}

fn step(pixels: &Pixels, lookup: &Lookup, is_lit_set: bool) -> Pixels {
    let need_invert = is_need_invert(&lookup);
    let lit_val = if need_invert ^ is_lit_set { 1 } else { 0 };
    let ((minx, maxx), (miny, maxy)) = get_ext(pixels);
    let mut res = HashSet::new();
    for j in miny - 1..maxy + 2 {
        for i in minx - 1..maxx + 2 {
            let mut mask = 0;
            for dj in -1..=1 {
                for di in -1..=1 {
                    let pos = (i + di, j + dj);
                    let mut bit = pixels.contains(&pos) as usize;
                    if !is_lit_set {
                        bit = 1 - bit;
                    }
                    mask = mask * 2 + bit;
                }
            }
            let val = lookup[mask];
            if val == lit_val {
                res.insert((i, j));
            }
        }
    }
    res
}

fn eval_pixels(start_pixels: &Pixels, lookup: &Lookup, steps: usize) -> usize {
    let mut pixels = start_pixels.clone();
    let need_invert = is_need_invert(&lookup);
    for i in 0..steps {
        let is_lit_set = !need_invert || (i % 2 == 0);
        pixels = step(&pixels, lookup, is_lit_set);
    }
    pixels.len()
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(20)).unwrap();
    let lookup: Lookup = lines[0]
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let mut pixels: Pixels = HashSet::new();
    for (j, line) in lines[2..].iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                pixels.insert((i as i32, j as i32));
            }
        }
    }
    let res1 = eval_pixels(&pixels, &lookup, 2);
    let res2 = eval_pixels(&pixels, &lookup, 50);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
