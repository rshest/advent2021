use crate::common;

use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;
type VisitedMap = HashMap<String, i32>;

const START_LABEL: &str = "start";
const END_LABEL: &str = "end";

fn traverse1(graph: &Graph, pos: &str, visited: &mut VisitedMap) -> u64 {
    let mut res = 0;
    for next_pos in &graph[pos] {
        if next_pos == END_LABEL {
            res += 1;
            continue;
        }
        if next_pos == START_LABEL {
            continue;
        }
        if next_pos.to_lowercase() == *next_pos && visited[next_pos] == 1 {
            continue;
        }
        *visited.get_mut(next_pos).unwrap() += 1;
        res += traverse1(graph, &next_pos, visited);
        *visited.get_mut(next_pos).unwrap() -= 1;
    }
    res
}

fn traverse2(graph: &Graph, pos: &str, visited: &mut VisitedMap, next_small: Option<&str>) -> u64 {
    let mut res = 0;
    for next_pos in &graph[pos] {
        if next_pos == END_LABEL {
            res += 1;
            continue;
        }
        if next_pos == START_LABEL {
            continue;
        }
        if next_pos.to_lowercase() == *next_pos {
            if visited[next_pos] == 1 {
                if next_small == None || &next_small.unwrap() == next_pos {
                    *visited.get_mut(next_pos).unwrap() += 1;
                    res += traverse2(graph, &next_pos, visited, Some(next_pos));
                    *visited.get_mut(next_pos).unwrap() -= 1;
                }
                continue;
            }
            if visited[next_pos] == 2 {
                continue;
            }
        }
        *visited.get_mut(next_pos).unwrap() += 1;
        res += traverse2(graph, &next_pos, visited, next_small);
        *visited.get_mut(next_pos).unwrap() -= 1;
    }
    res
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(12)).unwrap();
    let edges: Vec<(&str, &str)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split("-").collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut graph: Graph = HashMap::new();
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
    let mut visited: VisitedMap = graph.keys().map(|k| (k.to_string(), 0)).collect();
    let res1 = traverse1(&graph, &"start", &mut visited);
    let res2 = traverse2(&graph, &"start", &mut visited, None);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
