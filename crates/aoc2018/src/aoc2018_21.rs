use std::{collections::HashSet, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2018_21 {
    computer: Machine,
}

impl Aoc2018_21 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_21 {
    fn name(&self) -> (usize, usize) {
        (2018, 21)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-21.txt");
        let mut iter = lines.iter();
        let (_, ip_reg_num) = iter.next().unwrap().split_once(' ').unwrap();

        self.computer = Machine::new(ip_reg_num.parse().unwrap());

        for line in iter {
            let instruction: Vec<&str> = line.split(' ').collect();
            self.computer.program.push((
                instruction[0].parse().unwrap(),
                instruction[1].parse().unwrap(),
                instruction[2].parse().unwrap(),
                instruction[3].parse().unwrap(),
            ));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        self.computer.regs[0] = 0;
        aoclib::output(self.computer.run(true))
    }

    fn part2(&mut self) -> Vec<String> {
        self.computer.reset();
        aoclib::output(self.computer.run(false))
    }
}

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

impl FromStr for Opcode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "addr" => Ok(Opcode::AddR),
            "addi" => Ok(Opcode::AddI),
            "mulr" => Ok(Opcode::MulR),
            "muli" => Ok(Opcode::MulI),
            "banr" => Ok(Opcode::BanR),
            "bani" => Ok(Opcode::BanI),
            "borr" => Ok(Opcode::BorR),
            "bori" => Ok(Opcode::BorI),
            "setr" => Ok(Opcode::SetR),
            "seti" => Ok(Opcode::SetI),
            "gtir" => Ok(Opcode::GtIR),
            "gtri" => Ok(Opcode::GtRI),
            "gtrr" => Ok(Opcode::GtRR),
            "eqir" => Ok(Opcode::EqIR),
            "eqri" => Ok(Opcode::EqRI),
            "eqrr" => Ok(Opcode::EqRR),
            _ => Err(format!("Unknown opcode {value}")),
        }
    }
}

#[derive(Default)]
struct Machine {
    program: Vec<(Opcode, i64, i64, i64)>,
    regs: [i64; 6],
    ip: usize, // which register is the ip
    steps: usize,
}

impl Machine {
    fn new(ip: usize) -> Self {
        Self {
            ip,
            ..Self::default()
        }
    }

    fn reset(&mut self) {
        self.regs = [0; 6];
    }

    fn store(&mut self, reg: i64, val: i64) {
        self.regs[reg as usize] = val;
    }

    fn step(&mut self, op: Opcode, a: i64, b: i64, c: i64) {
        let reg_a = *self.regs.get(a as usize).unwrap_or(&0);
        let reg_b = *self.regs.get(b as usize).unwrap_or(&0);

        self.steps += 1;
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
            Opcode::GtIR => self.store(c, (a > reg_b) as i64),
            Opcode::GtRI => self.store(c, (reg_a > b) as i64),
            Opcode::GtRR => self.store(c, (reg_a > reg_b) as i64),
            Opcode::EqIR => self.store(c, (a == reg_b) as i64),
            Opcode::EqRI => self.store(c, (reg_a == b) as i64),
            Opcode::EqRR => self.store(c, (reg_a == reg_b) as i64),
        }
    }

    fn run(&mut self, part1: bool) -> i64 {
        let mut seen = HashSet::<i64>::new();
        let mut prev = 0;
        loop {
            let mut ip = self.regs[self.ip] as usize;

            if ip >= self.program.len() {
                return self.regs[3];
            }

            if ip == 28 {
                if part1 {
                    return self.regs[3];
                }
                if seen.insert(self.regs[3]) {
                    prev = self.regs[3];
                } else {
                    return prev;
                }
            }

            if ip == 18 {
                self.regs[1] /= 256;
                self.regs[self.ip] = 8;
                ip = 8;
            }

            //print!("{ip:3} : {:?} {:?} | => ", self.regs, self.program[ip]);
            self.step(
                self.program[ip].0,
                self.program[ip].1,
                self.program[ip].2,
                self.program[ip].3,
            );
            self.regs[self.ip] += 1;
            //println!("{:?}", self.regs);
        }
    }
}
