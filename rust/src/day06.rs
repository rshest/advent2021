use crate::common;
use std::collections::HashMap;

fn get_n_spawned(n: u64, days: u64, spawned:&mut HashMap<(u64, u64), u64>) -> u64 {
    if days <= n {
        return 0;
    }
    let key = (n, days);
    if spawned.contains_key(&key) {
        return spawned[&key];
    }
    let mut res = (days - n - 1) /7 + 1;
    for i in 0..res {
        res += get_n_spawned(8, days - n - i * 7 - 1, spawned);
    }
    spawned.insert(key, res);
    res
}

fn sum_spawned(nums: &Vec<u64>, total_days: u64) -> u64 {
    let mut spawned: HashMap<(u64, u64), u64> = HashMap::new();
    let s: u64 = nums.iter().map(|x| get_n_spawned(*x, total_days, &mut spawned)).sum();
    s + nums.len() as u64
}

pub(crate) fn solution() {
    let line: String = common::read_lines(&common::data_file(6)).unwrap().first().unwrap().clone();
    let nums: Vec<u64> = line.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
    println!("Answer 1: {}\nAnswer 2: {}", sum_spawned(&nums, 80), sum_spawned(&nums, 256));
}
