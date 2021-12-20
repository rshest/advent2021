use crate::common;

#[derive(Debug)]
struct Extents((i32, i32), (i32, i32));

impl Extents {
    fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.0 .0 && x <= self.0 .1 && y >= self.1 .0 && y <= self.1 .1
    }
}

fn parse_ext(line: &str) -> Extents {
    let prefix = "target area: ";
    let p: Vec<Vec<_>> = line[prefix.len()..]
        .split(", ")
        .map(|s| {
            s[2..]
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    Extents((p[0][0], p[0][1]), (p[1][0], p[1][1]))
}

fn simulate(ext: &Extents, vx0: i32, vy0: i32) -> Option<i32> {
    let (mut x, mut y) = (0, 0);
    let (mut vx, mut vy) = (vx0, vy0);
    let mut maxh = 0;
    loop {
        maxh = maxh.max(y);
        if ext.contains(x, y) {
            return Some(maxh);
        }
        if x > ext.0 .1 || y < ext.1 .0 {
            return None;
        }
        x += vx;
        y += vy;
        vx = (vx - 1).max(0);
        vy -= 1;
    }
}

fn find_maxh_and_numv(ext: &Extents) -> (i32, usize) {
    // solve (n + 1) * n / 2 = ext.minx to find minimum vx
    let min_vx = ((((8 * ext.0 .0) as f32).sqrt() as i32) - 1) / 2 + 1;
    let max_vx = ext.0 .1;
    let min_vy = -5 * min_vx;
    let max_vy = ext.1 .0.abs();
    let mut maxh = 0;
    let mut numv = 0;
    for vx in min_vx..max_vx + 1 {
        for vy in min_vy..max_vy + 1 {
            match simulate(ext, vx, vy) {
                Some(h) => {
                    maxh = maxh.max(h);
                    numv += 1;
                }
                _ => (),
            }
        }
    }
    (maxh, numv)
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(17)).unwrap();
    let ext = parse_ext(&lines[0]);
    let (maxh, numv) = find_maxh_and_numv(&ext);
    println!("Answer 1: {}\nAnswer 2: {}", maxh, numv);
}
