use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_16 {
    examples: Vec<Example>,
    program: Vec<[i32; 4]>,
}

impl Aoc2018_16 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_16 {
    fn name(&self) -> (usize, usize) {
        (2018, 16)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-16.txt");

        let mut iter = lines.iter();
        while let Some(line) = iter.next() {
            if line.starts_with("Before:") {
                let before = Example::parse_regs(line);

                let Some(line) = iter.next() else {
                    panic!("missing 'instruction'");
                };
                let cmd = Example::parse_instruction(line);

                let Some(line) = iter.next() else {
                    panic!("missing 'after'");
                };
                let after = Example::parse_regs(line);
                self.examples.push(Example { before, after, cmd });
            } else {
                self.program.push(Example::parse_instruction(line));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut three_or_more = 0;
        'next: for ex in &self.examples {
            let mut count = 0;
            for op in &OPCODES {
                let mut m = Machine::new(ex.before);
                m.step(*op, ex.cmd[1], ex.cmd[2], ex.cmd[3]);
                if m.regs == ex.after {
                    count += 1;
                    if count >= 3 {
                        three_or_more += 1;
                        continue 'next;
                    }
                }
            }
        }
        crate::output(three_or_more)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut possible = vec![HashSet::new(); 16];
        for ex in &self.examples {
            for op in &OPCODES {
                let mut m = Machine::new(ex.before);
                m.step(*op, ex.cmd[1], ex.cmd[2], ex.cmd[3]);
                if m.regs == ex.after {
                    possible[ex.cmd[0] as usize].insert(*op);
                }
            }
        }
        for (i, p) in possible.iter().enumerate() {
            println!("{i:2} -> {p:?}");
        }
        crate::output("unsolved")
    }
}

#[derive(Debug)]
struct Example {
    before: [i32; 4],
    after: [i32; 4],
    cmd: [i32; 4],
}

impl Example {
    fn parse_regs(line: &str) -> [i32; 4] {
        let (_, nums) = line.split_once('[').unwrap();
        let (nums, _) = nums.split_once(']').unwrap();
        let nums = nums.split(", ").collect::<Vec<_>>();
        [
            nums[0].parse().unwrap(),
            nums[1].parse().unwrap(),
            nums[2].parse().unwrap(),
            nums[3].parse().unwrap(),
        ]
    }

    fn parse_instruction(line: &str) -> [i32; 4] {
        let statement = line.split(' ').collect::<Vec<_>>();
        [
            statement[0].parse().unwrap(),
            statement[1].parse().unwrap(),
            statement[2].parse().unwrap(),
            statement[3].parse().unwrap(),
        ]
    }
}

const OPCODES: [Opcode; 16] = [
    Opcode::AddR,
    Opcode::AddI,
    Opcode::MulR,
    Opcode::MulI,
    Opcode::BanR,
    Opcode::BanI,
    Opcode::BorR,
    Opcode::BorI,
    Opcode::SetR,
    Opcode::SetI,
    Opcode::GtIR,
    Opcode::GtRI,
    Opcode::GtRR,
    Opcode::EqIR,
    Opcode::EqRI,
    Opcode::EqRR,
];

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Opcode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

struct Machine {
    regs: [i32; 4],
}

impl Machine {
    fn new(regs: [i32; 4]) -> Self {
        Self { regs }
    }

    fn store(&mut self, reg: i32, val: i32) {
        self.regs[reg as usize] = val;
    }

    fn step(&mut self, op: Opcode, a: i32, b: i32, c: i32) {
        let reg_a = *self.regs.get(a as usize).unwrap_or(&0);
        let reg_b = *self.regs.get(b as usize).unwrap_or(&0);

        match op {
            Opcode::AddR => self.store(c, reg_a + reg_b),
            Opcode::AddI => self.store(c, reg_a + b),
            Opcode::MulR => self.store(c, reg_a * reg_b),
            Opcode::MulI => self.store(c, reg_a * b),
            Opcode::BanR => self.store(c, reg_a & reg_b),
            Opcode::BanI => self.store(c, reg_a & b),
            Opcode::BorR => self.store(c, reg_a | reg_b),
            Opcode::BorI => self.store(c, reg_a | b),
            Opcode::SetR => self.store(c, reg_a),
            Opcode::SetI => self.store(c, a),
            Opcode::GtIR => self.store(c, (a > reg_b) as i32),
            Opcode::GtRI => self.store(c, (reg_a > b) as i32),
            Opcode::GtRR => self.store(c, (reg_a > reg_b) as i32),
            Opcode::EqIR => self.store(c, (a == reg_b) as i32),
            Opcode::EqRI => self.store(c, (reg_a == b) as i32),
            Opcode::EqRR => self.store(c, (reg_a == reg_b) as i32),
        }
    }
}
