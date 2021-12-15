use crate::common;

use std::collections::HashMap;

type RuleSet = HashMap<String, String>;
type PatternMap = HashMap<String, usize>;

fn step(hist: &PatternMap, rules: &RuleSet) -> PatternMap {
    let mut res: PatternMap = HashMap::new();
    for (s, cnt) in hist {
        if rules.contains_key(s) {
            let c = &rules[s];
            *res.entry(s[0..1].to_string() + &c).or_insert(0) += cnt;
            *res.entry(c.to_string() + &s[1..2]).or_insert(0) += cnt;
        } else {
            res.insert(s.to_string(), *cnt);
        }
    }
    res
}

fn get_pairs_hist(pattern: &str) -> PatternMap {
    let mut res = HashMap::new();
    for i in 1..pattern.len() {
        *res.entry(pattern[i - 1..i + 1].to_string()).or_insert(0) += 1;
    }
    res
}

fn eval_pattern(pat: &str, rules: &RuleSet, steps: usize) -> usize {
    let mut hist = get_pairs_hist(&pat);
    for _ in 0..steps {
        hist = step(&hist, &rules);
    }

    let mut char_hist = HashMap::new();
    for (s, cnt) in hist {
        *char_hist.entry(s[0..1].to_string()).or_insert(0) += cnt;
    }
    let last_char = pat[pat.len() - 1..pat.len()].to_string();
    *char_hist.entry(last_char).or_insert(0) += 1;

    let mut counts: Vec<&usize> = char_hist.values().collect();
    counts.sort();
    counts[counts.len() - 1] - counts[0]
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(14)).unwrap();
    let start_pattern = lines[0].to_string();
    let rules: RuleSet = lines[2..]
        .iter()
        .map(|s| {
            let parts: Vec<&str> = s.split(" -> ").collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();
    let res1 = eval_pattern(&start_pattern, &rules, 10);
    let res2 = eval_pattern(&start_pattern, &rules, 40);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
