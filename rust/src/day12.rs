use crate::common;
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, Vec<String>>;

const START_LABEL: &str = "start";
const END_LABEL: &str = "end";

fn traverse1(graph: &Graph, pos: &str, visited: &mut HashMap<&str, i32>) -> u64 {
    let mut res = 0;
    for next_pos in &graph[pos] {
        if next_pos == END_LABEL {
            res += 1;
            continue;
        }
        if next_pos == START_LABEL {
            continue;
        } /*
          if next_pos.to_lowercase() == *next_pos && visited[next_pos] == 1 {
              continue;
          }
          visited[next_pos] += 1;
          res += traverse1(graph, next_pos, visited);
          visited[&next_pos] -= 1;*/
    }
    res
}

fn traverse2(
    graph: &Graph,
    pos: &str,
    visited: &mut HashMap<&str, i32>,
    nex_small: Option<String>,
) -> u64 {
    /*
    res = 0
        for next_pos in graph[pos]:
            if next_pos == "end":
                res += 1
                continue
            if next_pos == "start":
                continue
            if next_pos.islower():
                if visited[next_pos] == 1:
                    if visited_small == None or visited_small == next_pos:
                        visited[next_pos] += 1
                        res += traverse2(graph, next_pos, visited, next_pos)
                        visited[next_pos] -= 1
                    continue
                if visited[next_pos] == 2:
                    continue
            visited[next_pos] += 1
            res += traverse2(graph, next_pos, visited, visited_small)
            visited[next_pos] -= 1
        return res
    */
    0
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::test_file(12, 1)).unwrap();
    let edges: Vec<(&str, &str)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split("-").collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (a, b) in &edges {
        graph
            .entry(a.to_string())
            .or_insert(vec![])
            .push(b.to_string());
        graph
            .entry(b.to_string())
            .or_insert(vec![])
            .push(a.to_string());
    }
    let mut visited: HashMap<String, i32> = graph.keys().map(|k| (k.to_string(), 0)).collect();
    let res1 = graph; //traverse1(&graph, &"start", &mut visited);
    let res2 = visited; //traverse2(&graph, &"start", &mut visited, None);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
