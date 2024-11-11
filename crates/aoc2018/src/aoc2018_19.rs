use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2018_19 {
    computer: Machine,
    program: Vec<(Opcode, i64, i64, i64)>,
}

impl Aoc2018_19 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_19 {
    fn name(&self) -> (usize, usize) {
        (2018, 19)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-19.txt");
        let mut iter = lines.iter();
        let (_, ip_reg_num) = iter.next().unwrap().split_once(' ').unwrap();

        for line in iter {
            let instruction: Vec<&str> = line.split(' ').collect();
            self.program.push((
                instruction[0].parse().unwrap(),
                instruction[1].parse().unwrap(),
                instruction[2].parse().unwrap(),
                instruction[3].parse().unwrap(),
            ));
        }
        self.computer = Machine::new(ip_reg_num.parse().unwrap());
    }

    fn part1(&mut self) -> Vec<String> {
        loop {
            let ip = self.computer.regs[self.computer.ip] as usize;
            if ip >= self.program.len() {
                break;
            }
            // print!("{ip}: ");
            self.computer.step(
                self.program[ip].0,
                self.program[ip].1,
                self.program[ip].2,
                self.program[ip].3,
            );
            self.computer.regs[self.computer.ip] += 1;
        }
        aoclib::output(self.computer.regs[0])
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
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
    regs: [i64; 6],
    ip: usize, // which register is the ip
}

impl Machine {
    fn new(ip: usize) -> Self {
        Self { regs: [0; 6], ip }
    }

    fn store(&mut self, reg: i64, val: i64) {
        self.regs[reg as usize] = val;
    }

    fn step(&mut self, op: Opcode, a: i64, b: i64, c: i64) {
        // print!("{op:?} {a} {b} {c} | ");
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
            Opcode::GtIR => self.store(c, (a > reg_b) as i64),
            Opcode::GtRI => self.store(c, (reg_a > b) as i64),
            Opcode::GtRR => self.store(c, (reg_a > reg_b) as i64),
            Opcode::EqIR => self.store(c, (a == reg_b) as i64),
            Opcode::EqRI => self.store(c, (reg_a == b) as i64),
            Opcode::EqRR => self.store(c, (reg_a == reg_b) as i64),
        }
        // println!("{:?}", self.regs);
    }
}
