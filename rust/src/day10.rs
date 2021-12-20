use crate::common;

#[derive(Debug)]
enum LineResult {
    Corrupted(char),
    Incomplete(String),
}

fn get_closing(c: char) -> char {
    match c {
        '[' => ']',
        '{' => '}',
        '<' => '>',
        '(' => ')',
        _ => panic!("get_closing: unexpected character '{}'", c),
    }
}

fn get_scores(c: char) -> (u64, u64) {
    match c {
        ')' => (3, 1),
        ']' => (57, 2),
        '}' => (1197, 3),
        '>' => (25137, 4),
        _ => panic!("get_scores: unexpected character '{}'", c),
    }
}

fn find_match(line: &str) -> LineResult {
    let mut chars = vec![];
    for c in line.chars() {
        if "[({<".contains(c) {
            chars.push(c);
        } else {
            let current = chars.pop().unwrap();
            if c != get_closing(current) {
                return LineResult::Corrupted(c);
            }
        }
    }
    chars.reverse();
    LineResult::Incomplete(chars.iter().cloned().map(get_closing).collect::<String>())
}

fn get_score2(line: &str) -> u64 {
    let mut res = 0;
    for c in line.chars() {
        res = res * 5 + get_scores(c).1;
    }
    res
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(10)).unwrap();
    let matches = lines.iter().map(|s| find_match(&s)).collect::<Vec<_>>();
    let mut res1 = 0;
    let mut scores2 = vec![];
    for m in &matches {
        match m {
            LineResult::Corrupted(c) => res1 += get_scores(*c).0,
            LineResult::Incomplete(s) => scores2.push(get_score2(&s)),
        }
    }
    scores2.sort();
    let res2 = &scores2[scores2.len() / 2];
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
