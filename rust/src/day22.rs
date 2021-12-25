use crate::common;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Cuboid {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

#[derive(Debug)]
enum OperationType {
    On,
    Off,
}

#[derive(Debug)]
struct Operation {
    op_type: OperationType,
    extents: Cuboid,
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" ");
        let op = it.next().unwrap();
        let extents = Cuboid::from_str(it.next().unwrap()).unwrap();
        match op {
            "on" => Ok(Operation {
                op_type: OperationType::On,
                extents,
            }),
            "off" => Ok(Operation {
                op_type: OperationType::Off,
                extents,
            }),
            _ => Err("Unexpected operation".to_string()),
        }
    }
}

impl Cuboid {
    fn intersects(&self, other: &Cuboid) -> bool {
        self.x1 < other.x2
            && self.x2 > other.x1
            && self.y1 < other.y2
            && self.y2 > other.y1
            && self.z1 < other.z2
            && self.z2 > other.z1
    }

    fn get_coord(&self, idx: usize) -> i32 {
        match idx {
            0 => self.x1,
            1 => self.x2,
            2 => self.y1,
            3 => self.y2,
            4 => self.z1,
            5 => self.z2,
            _ => panic!("Invalid coordinate index"),
        }
    }
}

impl FromStr for Cuboid {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let p = line
            .split(",")
            .map(|s| {
                s[2..]
                    .split("..")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(Cuboid {
            x1: p[0][0],
            x2: p[0][1],
            y1: p[1][0],
            y2: p[1][1],
            z1: p[2][0],
            z2: p[2][1],
        })
    }
}

fn get_lit_volume(ops: &Vec<Operation>, bound_region: Option<&Cuboid>) -> u64 {
    let mut sections: [Vec<i32>; 3] = [vec![], vec![], vec![]];
    for i in 0..3 {
        let c1 = ops.iter().map(|op| op.extents.get_coord(i * 2));
        let c2 = ops.iter().map(|op| op.extents.get_coord(i * 2 + 1) + 1);
        let mut coord: Vec<_> = c1.chain(c2).collect();
        coord.sort();
        coord.dedup();
        sections[i] = coord;
    }

    let mut grid: HashSet<(u16, u16, u16)> = HashSet::new();
    let mut vol: u64 = 0;
    for (
        idx,
        Operation {
            op_type,
            extents: ext,
        },
    ) in ops.iter().enumerate()
    {
        match bound_region {
            Some(b) => {
                if !b.intersects(&ext) {
                    continue;
                }
            }
            _ => (),
        }
        let i1 = sections[0].binary_search(&ext.x1).unwrap();
        let j1 = sections[1].binary_search(&ext.y1).unwrap();
        let k1 = sections[2].binary_search(&ext.z1).unwrap();

        let mut k = k1;
        while sections[2][k] <= ext.z2 {
            let mut j = j1;
            while sections[1][j] <= ext.y2 {
                let mut i = i1;
                while sections[0][i] <= ext.x2 {
                    let pos = (i as u16, j as u16, k as u16);
                    let is_lit = grid.contains(&pos);
                    if matches!(op_type, OperationType::On) ^ is_lit {
                        let dvol = (sections[0][i + 1] - sections[0][i]) as u64
                            * (sections[1][j + 1] - sections[1][j]) as u64
                            * (sections[2][k + 1] - sections[2][k]) as u64;
                        if matches!(op_type, OperationType::On) {
                            grid.insert(pos);
                            vol += dvol;
                        } else {
                            grid.remove(&pos);
                            vol -= dvol;
                        }
                    }
                    i += 1;
                }
                j += 1;
            }
            k += 1;
        }
    }
    vol
}

const REGION0: Cuboid = Cuboid {
    x1: -50,
    x2: 50,
    y1: -50,
    y2: 50,
    z1: -50,
    z2: 50,
};

pub(crate) fn solution() {
    let ops: Vec<Operation> = common::read_lines(&common::data_file(22))
        .unwrap()
        .iter()
        .map(|s| Operation::from_str(s).unwrap())
        .collect();
    let res1 = get_lit_volume(&ops, Some(&REGION0));
    let res2 = get_lit_volume(&ops, None);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
