use std::collections::HashMap;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_08 {
    inst: Vec<Inst>,
}

impl Aoc2017_08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_08 {
    fn name(&self) -> (usize, usize) {
        (2017, 8)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2017-08.txt") {
            let words = line.split(' ').collect::<Vec<&str>>();

            let reg = words[0].to_string();
            let is_inc = words[1] == "inc";
            let amount = words[2].parse().unwrap();
            let lhs = words[4].to_string();
            let op = Op::parse(words[5]);
            let rhs = words[6].parse().unwrap();

            self.inst.push(Inst {
                reg,
                is_inc,
                amount,
                lhs,
                op,
                rhs,
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut reg = HashMap::new();
        for i in &self.inst {
            let lhs = *reg.get(&i.lhs).unwrap_or(&0);
            if i.op.compare(lhs, i.rhs) {
                let val = reg.entry(&i.reg).or_insert(0);
                if i.is_inc {
                    *val += i.amount;
                } else {
                    *val -= i.amount;
                }
            }
        }
        crate::output(reg.values().max().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

struct Inst {
    reg: String,
    is_inc: bool,
    amount: i32,
    lhs: String,
    op: Op,
    rhs: i32,
}

enum Op {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "<" => Self::LessThan,
            ">" => Self::GreaterThan,
            "<=" => Self::LessThanOrEqual,
            ">=" => Self::GreaterThanOrEqual,
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            _ => panic!("Invalid token {s}"),
        }
    }

    fn compare(&self, lhs: i32, rhs: i32) -> bool {
        match self {
            Self::LessThan => lhs < rhs,
            Self::GreaterThan => lhs > rhs,
            Self::LessThanOrEqual => lhs <= rhs,
            Self::GreaterThanOrEqual => lhs >= rhs,
            Self::Equal => lhs == rhs,
            Self::NotEqual => lhs != rhs,
        }
    }
}
