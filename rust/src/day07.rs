use crate::common;

fn median(nums: &Vec<f64>) -> f64 {
    let mut nc = nums.clone();
    nc.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = nc.len() / 2;
    if nc.len() % 2 == 1 {
        nc[mid]
    } else {
        (nc[mid - 1] + nc[mid]) / 2.0
    }
}

fn part1(nums: &Vec<f64>) -> i32 {
    let minx = median(nums).round();
    let sum: f64 = nums.iter().map(|n| (n - minx).abs()).sum();
    sum as i32
}

fn min_gradient(p0: f64, f: impl Fn(f64) -> f64, num_it: usize) -> f64 {
    let eps = 0.001;
    let mut p: f64 = p0;
    for _i in 0..num_it {
        let pr = f(p + eps);
        let pl = f(p - eps);
        let dp = (pr - pl) / (2.0 * eps);
        p = p - dp * eps;
    }
    p
}

fn part2(nums: &Vec<f64>) -> i32 {
    let f = |x: f64| {
        let mut res = 0.0;
        for n in nums {
            let d = (x - n).abs();
            res += d * (d + 1.0) / 2.0;
        }
        res
    };
    let minx = min_gradient(median(&nums), f, 1000).round();
    f(minx) as i32
}

pub(crate) fn solution() {
    let line: String = common::read_lines(&common::data_file(7)).unwrap().first().unwrap().clone();
    let nums: Vec<f64> = line.split(",").map(|x| x.parse::<f64>().unwrap()).collect();
    println!("Answer 1: {}\nAnswer 2: {}", part1(&nums), part2(&nums));
}
