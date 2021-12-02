extern crate lazy_static;
extern crate regex;

use std::time;

mod common;

mod day01;
mod day02;

const SOLUTIONS: [Option<fn() -> ()>; 25] = [
    Some(day01::solution),
    Some(day02::solution),
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
            None => println!("<TODO>\n"),
            Some(f) => {
                let now = time::Instant::now();
                f();
                let elapsed = now.elapsed();
                timings.push(elapsed.as_micros());
                println!("Elapsed: {:.2?}\n", elapsed);
                total_elapsed += elapsed;
                total_problems += 1;
            }
        }
    }
    println!(
        "Total problems: {}, elapsed: {:.2?}",
        total_problems, total_elapsed
    );
    println!("\nProblem timings (mus): {:?}", timings);
}
