extern crate lazy_static;
extern crate regex;

use std::time;

mod common;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

const SOLUTIONS: [Option<fn() -> ()>; 25] = [
    Some(day01::solution),
    Some(day02::solution),
    Some(day03::solution),
    Some(day04::solution),
    Some(day05::solution),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

fn main() {
    let mut total_elapsed: time::Duration = time::Duration::new(0, 0);
    let mut total_problems: usize = 0;
    let mut timings: Vec<u128> = Vec::new();
    for (i, solution) in SOLUTIONS.iter().enumerate() {
        println!("--- Day{:02} ---", i + 1);
        match solution {
            None => println!("<TODO>"),
            Some(f) => {
                let now = time::Instant::now();
                f();
                let elapsed = now.elapsed();
                timings.push(elapsed.as_micros());
                println!("Elapsed: {:.2?}", elapsed);
                total_elapsed += elapsed;
                total_problems += 1;
            }
        }
    }
    println!(
        "Total problems: {}, elapsed: {:.4?}s",
        total_problems, total_elapsed.as_micros() as f64 / 1000000.0
    );
    println!("Problem timings (µs): {:?}", timings);
}
