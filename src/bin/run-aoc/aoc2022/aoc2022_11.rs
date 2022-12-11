use std::collections::VecDeque;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_11 {
    monkey: Vec<Monkey>,
}

impl Aoc2022_11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_11 {
    fn name(&self) -> (usize, usize) {
        (2022, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-11.txt");

        let mut monkey = Monkey::default();

        for line in lines {
            let words = line.trim().split(' ').collect::<Vec<&str>>();
            match words[0] {
                "Monkey" => monkey = Monkey::default(),
                "Starting" => {
                    let (_, strlist) = line.split_once(": ").unwrap();

                    monkey.items = strlist.split(", ").map(|w| w.parse().unwrap()).collect()
                }
                "Operation:" => {
                    monkey.op = if words[4] == "+" {
                        if words[5] == "old" {
                            Op::AddSelf
                        } else {
                            Op::Add(words[5].parse().unwrap())
                        }
                    } else if words[5] == "old" {
                        Op::MulSelf
                    } else {
                        Op::Mul(words[5].parse().unwrap())
                    };
                }
                "Test:" => monkey.test = words[3].parse().unwrap(),
                "If" => {
                    if words[1] == "true:" {
                        monkey.dest.0 = words[5].parse().unwrap();
                    } else {
                        monkey.dest.1 = words[5].parse().unwrap();
                        self.monkey.push(monkey);
                        monkey = Monkey::default();
                    }
                }
                _ => panic!("can't handle '{}' yet", words[0]),
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut part1 = self.monkey.clone();

        for _ in 0..20 {
            round(&mut part1, true);
        }

        let mut monkey_business = part1.iter().map(|m| m.count).collect::<Vec<usize>>();
        monkey_business.sort_by(|a, b| b.cmp(a));

        crate::output(monkey_business[0] * monkey_business[1])
    }

    fn part2(&mut self) -> Vec<String> {
        let mut part2 = self.monkey.clone();

        for _ in 0..10000 {
            round(&mut part2, false);
        }

        let mut monkey_business = part2.iter().map(|m| m.count).collect::<Vec<usize>>();
        monkey_business.sort_by(|a, b| b.cmp(a));

        crate::output(monkey_business[0] * monkey_business[1])
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    test: i64,
    dest: (usize, usize),
    count: usize,
}

#[derive(Debug, Default, Clone)]
enum Op {
    #[default]
    AddSelf,
    MulSelf,
    Mul(i64),
    Add(i64),
}

impl Op {
    fn calc(&self, val: i64) -> i64 {
        match self {
            Self::AddSelf => val + val,
            Self::MulSelf => val * val,
            Self::Mul(n) => val * *n,
            Self::Add(n) => val + *n,
        }
    }
}

fn round(mvec: &mut Vec<Monkey>, part1: bool) {
    let modval: i64 = mvec.iter().map(|m| m.test).product();

    for i in 0..mvec.len() {
        while let Some(item) = mvec[i].items.pop_front() {
            let worry = if part1 {
                mvec[i].op.calc(item) / 3
            } else {
                mvec[i].op.calc(item) % modval
            };
            let dest = if worry % mvec[i].test == 0 {
                mvec[i].dest.0
            } else {
                mvec[i].dest.1
            };
            mvec[dest].items.push_back(worry);
            mvec[i].count += 1;
        }
    }
}
