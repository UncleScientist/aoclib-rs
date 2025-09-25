use std::collections::HashSet;

use aoclib::Runner;

pub struct Aoc2016_08 {
    inst: Vec<Instruction>,
    answer: HashSet<(usize, usize)>,
}

impl Aoc2016_08 {
    pub fn new() -> Self {
        Self {
            inst: Vec::new(),
            answer: HashSet::new(),
        }
    }
}

impl Runner for Aoc2016_08 {
    fn name(&self) -> (usize, usize) {
        (2016, 8)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2016-08.txt") {
            self.inst.push(Instruction::new(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let _test_input = [
            Instruction::new("rect 3x2"),
            Instruction::new("rotate column x=1 by 1"),
            Instruction::new("rotate row y=0 by 4"),
            Instruction::new("rotate column x=1 by 1"),
        ];
        // for i in &_test_input {

        for i in &self.inst {
            match i {
                Instruction::Rect(col, row) => {
                    for r in 0..*row {
                        for c in 0..*col {
                            self.answer.insert((r, c));
                        }
                    }
                }
                Instruction::Column(col, amount) => {
                    let mut new_hs = HashSet::new();
                    for (r, c) in self.answer.iter() {
                        if *c == *col {
                            new_hs.insert(((*r + amount) % 6, *c));
                        } else {
                            new_hs.insert((*r, *c));
                        }
                    }
                    self.answer = new_hs;
                }
                Instruction::Row(row, amount) => {
                    let mut new_hs = HashSet::new();
                    for (r, c) in self.answer.iter() {
                        if *r == *row {
                            new_hs.insert((*r, (*c + amount) % 50));
                        } else {
                            new_hs.insert((*r, *c));
                        }
                    }
                    self.answer = new_hs;
                }
            }
        }
        aoclib::output(self.answer.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut answer = Vec::new();
        for r in 0..6 {
            let mut line = "".to_string();
            for c in 0..50 {
                if self.answer.contains(&(r, c)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            answer.push(line);
        }

        answer
    }
}

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),   // AxB rectangle
    Column(usize, usize), // rotate column A by B
    Row(usize, usize),    // rotate row A by B
}

impl Instruction {
    fn new(s: &str) -> Self {
        let (cmd, rest) = s.split_once(' ').unwrap();
        match cmd {
            "rect" => {
                let (a, b) = rest.split_once('x').unwrap();
                Self::Rect(a.parse().unwrap(), b.parse().unwrap())
            }
            "rotate" => {
                let (rc, amount) = rest.split_once('=').unwrap();
                let (a, b) = amount.split_once(" by ").unwrap();
                let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
                if rc.starts_with("row") {
                    Self::Row(a, b)
                } else {
                    Self::Column(a, b)
                }
            }
            _ => {
                panic!("invalid input");
            }
        }
    }
}
