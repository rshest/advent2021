use crate::common;

fn get_most_common<'a>(nums: impl Iterator<Item = &'a Vec<u8>>, pos: usize) -> u8 {
    let mut n0 = 0;
    let mut n1 = 0;
    for num in nums {
        n0 += (num[pos] == 0) as usize;
        n1 += (num[pos] == 1) as usize;
    }
    if n1 >= n0 {
        1
    } else {
        0
    }
}

fn part1(nums: &Vec<Vec<u8>>) -> i64 {
    let mut lc: i64 = 0;
    let mut mc: i64 = 0;
    for i in 0..nums[0].len() {
        let m = get_most_common(nums.iter(), i) as i64;
        mc = mc * 2 + m;
        lc = lc * 2 + (1 - m);
    }
    mc * lc
}

fn digits_to_decimal(digits: &Vec<u8>) -> i64 {
    digits.iter().fold(0i64, |v, d| v * 2 + *d as i64)
}

fn part2(nums: &Vec<Vec<u8>>) -> i64 {
    let mut mnums: Vec<usize> = (0..nums.len()).collect();
    let mut lnums: Vec<usize> = (0..nums.len()).collect();
    for i in 0..nums[0].len() {
        if mnums.len() > 1 {
            let m = get_most_common(mnums.iter().cloned().map(|x| &nums[x]), i);
            mnums = mnums.iter().cloned().filter(|&x| nums[x][i] == m).collect();
        }
        if lnums.len() > 1 {
            let l = 1 - get_most_common(lnums.iter().cloned().map(|x| &nums[x]), i);
            lnums = lnums.iter().cloned().filter(|&x| nums[x][i] == l).collect();
        }
    }
    digits_to_decimal(&nums[mnums[0]]) * digits_to_decimal(&nums[lnums[0]])
}

pub(crate) fn solution() {
    let nums: Vec<Vec<u8>> = common::read_lines(&common::data_file(3))
        .unwrap()
        .iter()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    println!("Answer 1: {}\nAnswger 2: {}", part1(&nums), part2(&nums));
}
