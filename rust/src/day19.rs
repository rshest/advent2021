use crate::common;
use std::collections::HashMap;
use std::collections::HashSet;

extern crate itertools;
use itertools::Itertools;

type DirMap = HashMap<Vec3, (usize, usize)>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug)]
struct Mat3 {
    pub row: [Vec3; 3],
}

impl Vec3 {
    fn dot(&self, other: &Vec3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: -other.y * self.z + self.y * other.z,
            y: other.x * self.z - self.x * other.z,
            z: -other.x * self.y + self.x * other.y,
        }
    }

    fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn transform(&self, m: &Mat3) -> Vec3 {
        Vec3 {
            x: self.dot(&m.row[0]),
            y: self.dot(&m.row[1]),
            z: self.dot(&m.row[2]),
        }
    }

    fn set(&mut self, pos: usize, val: i32) {
        match pos {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.z = val,
            _ => panic!("Invalid Vec3 component index"),
        }
    }

    fn abs(&self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn reverse(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn zero() -> Vec3 {
        Vec3 { x: 0, y: 0, z: 0 }
    }

    fn manhattan_dist(&self, other: &Vec3) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Mat3 {
    fn zero() -> Mat3 {
        Mat3 {
            row: [Vec3::zero(), Vec3::zero(), Vec3::zero()],
        }
    }
}

fn enum_transforms() -> Vec<Mat3> {
    let mut res = vec![];
    for i in 0..3 {
        for sign1 in [1, -1] {
            for j in 0..3 {
                if i == j {
                    continue;
                }
                for sign2 in [1, -1] {
                    let mut tm = Mat3::zero();
                    tm.row[0].set(i, sign1);
                    tm.row[1].set(j, sign2);
                    tm.row[2] = tm.row[0].cross(&tm.row[1]);
                    res.push(tm);
                }
            }
        }
    }
    res
}

fn compute_dirs(coords: &Vec<Vec3>) -> HashMap<Vec3, (usize, usize)> {
    let mut res = HashMap::new();
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            res.insert(coords[j].sub(&coords[i]).abs(), (i, j));
        }
    }
    res
}

fn get_offset(
    coords1: &Vec<Vec3>,
    coords2: &Vec<Vec3>,
    dirs1: &DirMap,
    dirs2: &DirMap,
) -> Option<Vec3> {
    let mut num_matching_dirs = 0;
    let mut candidate_point1 = 0;
    let mut candidate_points2 = vec![];
    for v in dirs1.keys() {
        if dirs2.contains_key(&v) {
            num_matching_dirs += 1;
            if candidate_points2.len() == 0 {
                let (a, b) = dirs2[&v];
                candidate_points2.push(a);
                candidate_points2.push(b);
                candidate_point1 = dirs1[&v].0;
            }
        }
    }
    if num_matching_dirs < 66 {
        return None;
    }

    let coords2_set: HashSet<&Vec3> = coords2.iter().collect();
    for cand_pt in candidate_points2 {
        let c1 = &coords1[candidate_point1];
        let c2 = &coords2[cand_pt];
        let offs = c2.sub(&c1);
        let mut num_matching_pos = 0;
        for v in coords1 {
            if coords2_set.contains(&v.add(&offs)) {
                num_matching_pos += 1;
            }
        }
        if num_matching_pos >= 12 {
            return Some(offs);
        }
    }
    None
}

fn get_scanner_positions(data: &Vec<Vec<Vec3>>) -> Option<(Vec<Vec3>, Vec<Vec<Vec3>>)> {
    struct Context {
        res: Vec<Vec3>,
        found: HashSet<usize>,
        to_find: HashSet<usize>,
        non_overlapping: HashSet<(usize, usize)>,
        dirs: HashMap<(usize, usize), DirMap>,
        coords: HashMap<(usize, usize), Vec<Vec3>>,
        transforms: Vec<Mat3>,
    }

    let mut ctx = Context {
        res: vec![Vec3::zero(); data.len()],
        found: vec![0].into_iter().collect(),
        to_find: (1..data.len()).into_iter().collect(),
        non_overlapping: HashSet::new(),
        dirs: HashMap::new(),
        coords: HashMap::new(),
        transforms: enum_transforms(),
    };

    // precompute both transformed coordinates and directions
    for i in 0..data.len() {
        for (k, tm) in ctx.transforms.iter().enumerate() {
            let coords_tm = data[i].iter().map(|p| p.transform(&tm)).collect();
            ctx.dirs.insert((i, k), compute_dirs(&coords_tm));
            ctx.coords.insert((i, k), coords_tm);
        }
    }

    fn test_pair(i: usize, j: usize, ctx: &mut Context) -> bool {
        if ctx.non_overlapping.contains(&(i, j)) {
            return false;
        }
        for k in 0..ctx.transforms.len() {
            match get_offset(
                &ctx.coords[&(j, 0)],
                &ctx.coords[&(i, k)],
                &ctx.dirs[&(j, 0)],
                &ctx.dirs[&(i, k)],
            ) {
                None => continue,
                Some(offs) => {
                    ctx.coords.insert(
                        (i, 0),
                        ctx.coords[&(i, k)].iter().map(|pt| pt.sub(&offs)).collect(),
                    );
                    ctx.dirs.insert((i, 0), compute_dirs(&ctx.coords[&(i, k)]));
                    ctx.found.insert(i);
                    ctx.to_find.remove(&i);
                    ctx.res[i] = offs.reverse();
                    return true;
                }
            }
        }
        ctx.non_overlapping.insert((i, j));
        false
    }

    while ctx.to_find.len() > 0 {
        let pairs: Vec<(usize, usize)> = ctx
            .to_find
            .iter()
            .cartesian_product(ctx.found.iter())
            .map(|(i, j)| (*i, *j))
            .collect();
        let mut found = false;
        for (i, j) in pairs {
            if test_pair(i, j, &mut ctx) {
                found = true;
                break;
            }
        }
        if !found {
            println!("No solution");
            return None;
        }
    }
    let data_tm = (0..data.len())
        .map(|i| ctx.coords[&(i, 0)].clone())
        .collect();
    let res = ctx.res;
    Some((res, data_tm))
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(19)).unwrap();
    let mut data: Vec<Vec<Vec3>> = lines
        .split(|s| s.len() == 0)
        .map(|lines| {
            lines[1..]
                .iter()
                .map(|line| {
                    let mut parts = line.split(",").map(|n| n.parse::<i32>().unwrap());
                    Vec3 {
                        x: parts.next().unwrap(),
                        y: parts.next().unwrap(),
                        z: parts.next().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    if let Some((scanner_pos, data_tm)) = get_scanner_positions(&mut data) {
        let mut bpos: HashSet<Vec3> = HashSet::new();
        for d in data_tm {
            for p in d {
                bpos.insert(p);
            }
        }
        let res1 = bpos.len();

        let mut res2 = 0;
        for i in 0..scanner_pos.len() {
            for j in i + 1..scanner_pos.len() {
                res2 = res2.max(scanner_pos[i].manhattan_dist(&scanner_pos[j]));
            }
        }
        println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
    }
}
