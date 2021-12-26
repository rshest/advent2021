use crate::common;

use std::collections::HashSet;

use std::str::FromStr;

type Field = HashSet<(i32, i32)>;

#[derive(Debug)]
enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("fold along ");
        parts.next();
        let s0 = parts.next().unwrap();
        let n = s0[2..].parse::<i32>().unwrap();
        let c = s0.chars().next().unwrap();
        match c {
            'y' => Ok(Fold::Horizontal(n)),
            'x' => Ok(Fold::Vertical(n)),
            _ => Err(format!["Invalid fold direction: {}", c]),
        }
    }
}

fn apply_fold(field: Field, fold: &Fold) -> Field {
    let mut res = HashSet::new();
    match fold {
        Fold::Vertical(pos) => {
            for (x, y) in field {
                if x < *pos {
                    res.insert((x, y));
                } else {
                    res.insert((2 * pos - x, y));
                }
            }
        }
        Fold::Horizontal(pos) => {
            for (x, y) in field {
                if y < *pos {
                    res.insert((x, y));
                } else {
                    res.insert((x, 2 * pos - y));
                }
            }
        }
    }
    res
}

fn to_str(field: &Field) -> String {
    let minx = field.iter().map(|x| x.0).min().unwrap();
    let miny = field.iter().map(|x| x.1).min().unwrap();
    let maxx = field.iter().map(|x| x.0).max().unwrap();
    let maxy = field.iter().map(|x| x.1).max().unwrap();
    let mut res = String::new();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let c = if field.contains(&(x, y)) { "■" } else { "." };
            res.push_str(c);
        }
        res.push_str("\n");
    }
    res
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(13)).unwrap();
    let mut parts = lines.split(|s| s.len() == 0);
    let mut field: Field = parts
        .next()
        .unwrap()
        .iter()
        .map(|s| {
            let mut nums = s.split(",").map(|x| x.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    let folds: Vec<Fold> = parts
        .next()
        .unwrap()
        .iter()
        .map(|s| Fold::from_str(s).unwrap())
        .collect();

    field = apply_fold(field, &folds[0]);
    let res1 = field.len();

    for fold in &folds[1..] {
        field = apply_fold(field, fold);
    }
    println!("Answer 1: {:?}\nAnswer 2:\n{}", res1, to_str(&field));
}
