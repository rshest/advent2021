use crate::common;
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, Vec<String>>;

pub(crate) fn solution() {
    let lines = common::read_lines(&common::test_file(12, 1)).unwrap();
    let edges: Vec<(&str, &str)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split("-").collect();
            (parts[0], parts[1])
        })
        .collect();
    let res1 = edges;
    let res2 = 0;
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
