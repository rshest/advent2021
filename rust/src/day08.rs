use crate::common;

use std::str;
type Mapping = Vec<Vec<u8>>;

const CHARS: &str = "abcdefg";
const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];
const NUM_SEGMENTS: usize = 7;

fn normalize_mapping(mapping: &mut Mapping) {
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..mapping.len() {
            let c = &mapping[i];
            if c.iter().sum::<u8>() == 1 {
                let p = c.iter().position(|&e| e == 1).unwrap();
                for j in 0..NUM_SEGMENTS {
                    if j != i && mapping[j][p] == 1 {
                        mapping[j][p] = 0;
                        changed = true;
                    }
                }
            }
        }
    }
}

fn is_valid_mapping(mapping: &Mapping) -> bool {
    for i in 0..NUM_SEGMENTS {
        if mapping.iter().map(|m| m[i]).sum::<u8>() != 1 {
            return false;
        }
    }
    mapping.iter().map(|m| m.iter().sum::<u8>()).all(|s| s == 1)
}

fn get_mapping(mapping: &mut Mapping, combs: &Vec<&str>, pos: usize) -> Option<Mapping> {
    if pos == combs.len() {
        normalize_mapping(mapping);
        if is_valid_mapping(&mapping) {
            return Some(mapping.clone());
        } else {
            return None;
        }
    }

    let comb = &combs[pos];
    for digit in DIGITS {
        if digit.len() != comb.len() {
            continue;
        }
        let mut mapping1 = mapping.clone();
        for d in CHARS.bytes() {
            if !digit.as_bytes().contains(&d) {
                for &c in comb.as_bytes() {
                    mapping1[c as usize - 'a' as usize][d as usize - 'a' as usize] = 0;
                }
            }
        }
        if let Some(res_mapping) = get_mapping(&mut mapping1, combs, pos + 1) {
            return Some(res_mapping);
        }
    }
    None
}

fn decode_digit(mapping: &Mapping, combs: &Vec<&str>) -> u64 {
    let mut res = 0;
    for comb in combs {
        let mut scomb = comb.as_bytes().to_vec();
        for i in 0..scomb.len() {
            scomb[i] = mapping[scomb[i] as usize - 'a' as usize]
                .iter()
                .position(|&e| e == 1)
                .map(|x| x + 'a' as usize)
                .unwrap() as u8;
        }
        scomb.sort();
        let s = str::from_utf8(&scomb).unwrap();
        let digit = DIGITS.iter().position(|&e| e == s).unwrap();
        res = res * 10 + digit;
    }
    res as u64
}

fn decode_sum(inp: &Vec<Vec<Vec<&str>>>) -> u64 {
    let mut res = 0;
    for line in inp {
        let combs = &line[0];
        let to_decode = &line[1];
        let mut mapping: Mapping = vec![vec![1; NUM_SEGMENTS]; NUM_SEGMENTS];
        let mut combs_sorted = combs.clone();
        combs_sorted.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        if let Some(res_mapping) = get_mapping(&mut mapping, &combs_sorted, 0) {
            res += decode_digit(&res_mapping, &to_decode);
        }
    }
    res as u64
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(8)).unwrap();
    let inp: Vec<Vec<Vec<&str>>> = lines
        .iter()
        .map(|x| {
            x.split("|")
                .map(|s| s.trim().split(" ").collect())
                .collect()
        })
        .collect();

    let res1: usize = inp
        .iter()
        .map(|v| {
            v[1].iter()
                .filter(|c| [2, 3, 4, 7].contains(&c.len()))
                .count()
        })
        .sum();
    let res2 = decode_sum(&inp);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
