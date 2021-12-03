use crate::common;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');
        let opstr: &str = it.next().ok_or(())?;
        let val: i32 = it.next().ok_or(())?.parse::<i32>().map_err(|_| ())?;
        match opstr {
            "forward" => Ok(Command::Forward(val)),
            "up" => Ok(Command::Up(val)),
            "down" => Ok(Command::Down(val)),
            _ => Err(()),
        }
    }
}

fn step1((x, depth): (i32, i32), cmd: &Command) -> (i32, i32) {
    match cmd {
        Command::Up(offs) => (x, depth - offs),
        Command::Down(offs) => (x, depth + offs),
        Command::Forward(offs) => (x + offs, depth),
    }
}

fn step2((x, depth, aim): (i32, i32, i32), cmd: &Command) -> (i32, i32, i32) {
    match cmd {
        Command::Up(offs) => (x, depth, aim - offs),
        Command::Down(offs) => (x, depth, aim + offs),
        Command::Forward(offs) => (x + offs, depth + aim * offs, aim),
    }
}

pub(crate) fn solution() {
    let commands: Vec<Command> = common::read_lines(&common::data_file(2))
        .unwrap()
        .iter()
        .filter_map(|line| line.parse().ok())
        .collect();
    let pos1 = commands.iter().fold((0i32, 0i32), step1);
    let pos2 = commands.iter().fold((0i32, 0i32, 0i32), step2);
    println!(
        "Answer 1: {}\nAnswer 2: {}\n",
        pos1.0 * pos1.1,
        pos2.0 * pos2.1
    );
}
