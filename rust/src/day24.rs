use crate::common;

use std::str::FromStr;
type Reg = u8;

#[derive(Debug)]
enum RegOrVal {
    Reg(u8),
    Val(i32),
}

#[derive(Debug)]
enum Op {
    Inp(Reg),
    Add(Reg, RegOrVal),
    Mul(Reg, RegOrVal),
    Div(Reg, RegOrVal),
    Mod(Reg, RegOrVal),
    Eql(Reg, RegOrVal),
}

fn parse_reg(s: &str) -> Option<u8> {
    match s {
        "w" => Some(0),
        "x" => Some(1),
        "y" => Some(2),
        "z" => Some(3),
        _ => None,
    }
}

fn parse_reg_or_val(s: &str) -> RegOrVal {
    match parse_reg(s) {
        Some(reg) => RegOrVal::Reg(reg),
        None => RegOrVal::Val(s.parse::<i32>().unwrap()),
    }
}

impl FromStr for Op {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let p = line.split(" ").collect::<Vec<_>>();
        let reg = parse_reg(p[1]).unwrap();
        match p[0] {
            "inp" => Ok(Op::Inp(reg)),
            "add" => Ok(Op::Add(reg, parse_reg_or_val(p[2]))),
            "mul" => Ok(Op::Mul(reg, parse_reg_or_val(p[2]))),
            "div" => Ok(Op::Div(reg, parse_reg_or_val(p[2]))),
            "mod" => Ok(Op::Mod(reg, parse_reg_or_val(p[2]))),
            "eql" => Ok(Op::Eql(reg, parse_reg_or_val(p[2]))),
            _ => Err(format!["Invalid operation: {}", p[0]]),
        }
    }
}

type Program = Vec<Op>;

fn eval_prog(prog: &Program, inp: &[u8]) -> Result<[i32; 4], String> {
    let mut pos = 0;
    let mut reg = [0; 4]; // wxyz

    fn get_val(v: &RegOrVal, reg: &[i32; 4]) -> i32 {
        match v {
            RegOrVal::Reg(reg_idx) => reg[*reg_idx as usize],
            RegOrVal::Val(val) => *val,
        }
    }

    for (i, op) in prog.iter().enumerate() {
        match op {
            Op::Inp(reg_idx) => {
                if pos == inp.len() {
                    return Err(format![
                        "Accessed input outside of size ({} vs {})",
                        pos,
                        inp.len()
                    ]);
                }
                reg[*reg_idx as usize] = inp[pos] as i32;
                pos += 1;
            }
            Op::Add(reg_idx, val) => {
                reg[*reg_idx as usize] += get_val(val, &reg);
            }
            Op::Mul(reg_idx, val) => {
                reg[*reg_idx as usize] *= get_val(val, &reg);
            }
            Op::Div(reg_idx, val) => {
                let v2 = get_val(val, &reg);
                if v2 == 0 {
                    return Err(format!["Division by zero, op {}", i]);
                }
                reg[*reg_idx as usize] /= get_val(val, &reg);
            }
            Op::Mod(reg_idx, val) => {
                let v1 = reg[*reg_idx as usize];
                let v2 = get_val(val, &reg);
                if v1 < 0 || v2 <= 0 {
                    return Err(format![
                        "Incorrect arguments to mod operation ({}, {}), op {}",
                        v1, v2, i
                    ]);
                }
                reg[*reg_idx as usize] = v1 % v2;
            }
            Op::Eql(reg_idx, val) => {
                let v1 = reg[*reg_idx as usize];
                let v2 = get_val(val, &reg);
                reg[*reg_idx as usize] = (v1 == v2) as i32;
            }
        }
    }
    Ok(reg)
}

const MODEL_NUM_LEN: usize = 14;

enum Constraint {
    Range((u8, u8)),
    Derived((u8, i8)),
}

// Extra constraints extracted by meticulousluy reading the input code:
// n2 = 9
// n3 = 1
// n5 = n4 - 2
// n6 = n1 - 3
// n10 = n9 + 5
// n11 = n8 - 5
// n12 = n7 + 4
// n13 = n0 - 1
const CONSTRAINTS: [Constraint; MODEL_NUM_LEN] = [
    Constraint::Range((2, 10)),
    Constraint::Range((4, 10)),
    Constraint::Range((9, 10)),
    Constraint::Range((1, 2)),
    Constraint::Range((3, 10)),
    Constraint::Derived((4, -2)),
    Constraint::Derived((1, -3)),
    Constraint::Range((1, 6)),
    Constraint::Range((6, 10)),
    Constraint::Range((1, 5)),
    Constraint::Derived((9, 5)),
    Constraint::Derived((8, -5)),
    Constraint::Derived((7, 4)),
    Constraint::Derived((0, -1)),
];

fn inp_to_string(inp: &[u8]) -> String {
    inp.iter()
        .map(|c| char::from_digit(*c as u32, 10).unwrap())
        .collect()
}

fn get_min_max(prog: &Program) -> (String, String) {
    let mut res_min = [0; MODEL_NUM_LEN];
    let mut res_max = [0; MODEL_NUM_LEN];

    for (i, c) in CONSTRAINTS.iter().enumerate() {
        match c {
            Constraint::Range((v1, v2)) => {
                res_min[i] = *v1;
                res_max[i] = *v2 - 1;
            }
            Constraint::Derived((v1, v2)) => {
                res_min[i] = (res_min[*v1 as usize] as i8 + v2) as u8;
                res_max[i] = (res_max[*v1 as usize] as i8 + v2) as u8;
            }
        }
    }
    let reg1 = eval_prog(&prog, &res_min).unwrap();
    if reg1[3] != 0 {
        panic!("Incorrect minimum res value: {}", reg1[3]);
    }
    let reg2 = eval_prog(&prog, &res_max).unwrap();
    if reg2[3] != 0 {
        panic!("Incorrect maximum res value: {}", reg2[3]);
    }
    (inp_to_string(&res_min), inp_to_string(&res_max))
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(24)).unwrap();
    let ops: Program = lines.iter().map(|s| Op::from_str(s).unwrap()).collect();
    let (res2, res1) = get_min_max(&ops);
    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
