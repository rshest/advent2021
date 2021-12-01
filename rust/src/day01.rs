extern crate itertools;
use itertools::Itertools;

use crate::common;

pub(crate) fn solution() {
  let nums: Vec<i64> = common::read_integers(&common::data_file(1)).unwrap();
  let res1: i64 = nums.iter()
    .tuple_windows()
    .map(|(a, b)| (b > a) as i64).sum();
  let res2: i64 = nums.iter()
    .tuple_windows::<(_, _, _)>()
    .map(|(a, b, c)| a + b + c)
    .tuple_windows()
    .map(|(a, b)| (b > a) as i64)
    .sum();
  println!("Answer 1: {}\nAnswer 2: {}", res1, res2);
}
