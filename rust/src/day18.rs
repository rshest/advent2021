use crate::common;
use std::fmt;
use std::iter::Peekable;
use std::ptr;

#[derive(PartialEq)]
enum ReduceOp {
    None,
    Explode,
    Split,
}

#[derive(Clone)]
enum SFNum {
    Regular(u32),
    Nested((Box<SFNum>, Box<SFNum>)),
}

impl fmt::Debug for SFNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SFNum::Regular(n) => write!(f, "{}", n),
            SFNum::Nested((a, b)) => write!(f, "[{:?},{:?}]", a, b),
        }
    }
}

impl SFNum {
    fn magnitude(&self) -> u32 {
        match self {
            SFNum::Regular(n) => *n,
            SFNum::Nested((a, b)) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn add(&self, other: &SFNum) -> SFNum {
        SFNum::Nested((Box::new(self.clone()), Box::new(other.clone())))
    }
}

fn parse_sf_num<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Result<SFNum, String> {
    while let Some(&c) = it.peek() {
        match c {
            _ if char::is_digit(c, 10) => {
                let mut num = 0;
                while let Some(&n) = it.peek() {
                    if !char::is_digit(n, 10) {
                        break;
                    }
                    num = num * 10 + (n as u32 - '0' as u32);
                    it.next();
                }
                return Ok(SFNum::Regular(num));
            }
            '[' => {
                it.next();
                let a = parse_sf_num(it)?;
                let b = parse_sf_num(it)?;
                if it.peek() != Some(&']') {
                    return Err("Expecting closing ']'".to_string());
                }
                it.next();
                return Ok(SFNum::Nested((Box::new(a), Box::new(b))));
            }
            ',' => {
                it.next();
            }
            _ if char::is_whitespace(c) => {
                it.next();
            }
            _ => return Err(format!("Invalid character: '{}'", c)),
        }
    }
    Err("Incomplete stream".to_string())
}

fn reduce_num(num: &mut SFNum, can_split: bool) -> ReduceOp {
    struct Context {
        prev_regular: *mut SFNum,
        next_regular_inc: u32,
        current_op: ReduceOp,
        can_split: bool,
    }

    fn add_to_prev_regular(val: u32, ctx: &mut Context) {
        if !ctx.prev_regular.is_null() {
            unsafe {
                match *ctx.prev_regular {
                    SFNum::Regular(prevn) => *ctx.prev_regular = SFNum::Regular(val + prevn),
                    _ => {}
                }
            }
        }
    }

    fn rec(num: &mut SFNum, depth: u32, ctx: &mut Context) {
        if ctx.current_op != ReduceOp::None {
            match num {
                SFNum::Regular(n) => {
                    *num = SFNum::Regular(*n + ctx.next_regular_inc);
                    ctx.next_regular_inc = 0;
                }
                SFNum::Nested((a, b)) => {
                    rec(a, depth + 1, ctx);
                    rec(b, depth + 1, ctx);
                }
            }
            return;
        }
        match num {
            SFNum::Nested((a, b)) => match (&mut **a, &mut **b) {
                (SFNum::Regular(n0), SFNum::Regular(n1)) if depth >= 4 => {
                    add_to_prev_regular(*n0, ctx);
                    ctx.next_regular_inc = *n1;
                    *num = SFNum::Regular(0);
                    ctx.current_op = ReduceOp::Explode;
                    return;
                }
                _ => {
                    rec(a, depth + 1, ctx);
                    rec(b, depth + 1, ctx);
                }
            },
            SFNum::Regular(n) => {
                if ctx.can_split && *n >= 10 {
                    if depth >= 4 {
                        add_to_prev_regular(*n / 2, ctx);
                        ctx.next_regular_inc = (*n + 1) / 2;
                        *num = SFNum::Regular(0);
                        ctx.current_op = ReduceOp::Explode;
                        return;
                    } else {
                        *num = SFNum::Nested((
                            Box::new(SFNum::Regular(*n / 2)),
                            Box::new(SFNum::Regular((*n + 1) / 2)),
                        ));
                        ctx.current_op = ReduceOp::Split;
                        return;
                    }
                }
                ctx.prev_regular = num;
            }
        }
    }
    let mut ctx: Context = Context {
        prev_regular: ptr::null_mut(),
        next_regular_inc: 0,
        current_op: ReduceOp::None,
        can_split,
    };
    rec(num, 0, &mut ctx);
    ctx.current_op
}

fn eval_nums(nums: &Vec<SFNum>) -> SFNum {
    let mut res = nums[0].clone();
    for i in 1..nums.len() {
        res = res.add(&nums[i]);
        loop {
            let op1 = reduce_num(&mut res, false);
            if op1 == ReduceOp::Explode {
                continue;
            }
            let op2 = reduce_num(&mut res, true);
            if op2 == ReduceOp::None {
                break;
            }
        }
    }
    res
}

pub(crate) fn solution() {
    let lines = common::read_lines(&common::data_file(18)).unwrap();
    let sfnums: Vec<SFNum> = lines
        .iter()
        .map(|s| parse_sf_num(&mut s.chars().peekable()).unwrap())
        .collect();
    let res1 = eval_nums(&sfnums).magnitude();

    let mut res2 = 0;
    for i in 0..sfnums.len() {
        for j in 0..sfnums.len() {
            if i != j {
                let num = eval_nums(&vec![sfnums[i].clone(), sfnums[j].clone()]);
                res2 = res2.max(num.magnitude());
            }
        }
    }

    println!("Answer 1: {:?}\nAnswer 2: {:?}", res1, res2);
}
