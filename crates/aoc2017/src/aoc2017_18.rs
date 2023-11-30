use std::collections::VecDeque;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_18 {
    inst: Vec<Inst>,
}

impl Aoc2017_18 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_18 {
    fn name(&self) -> (usize, usize) {
        (2017, 18)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-18.txt");

        for line in &lines {
            self.inst.push(Inst::parse(line))
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut machine = Machine::new(&self.inst, 0);

        while machine.recovered_freq.is_none() {
            machine.step();
        }

        aoclib::output(machine.recovered_freq.unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut m0 = Machine::new(&self.inst, 0);
        let mut m1 = Machine::new(&self.inst, 1);
        let mut changed = true;

        while changed {
            while m0.receive_reg.is_none() {
                if !m0.step() {
                    break;
                }
            }
            while m1.receive_reg.is_none() {
                if !m1.step() {
                    break;
                }
            }

            changed = false;
            if let Some(val) = m1.pop() {
                m0.receive(val);
                changed = true;
            }
            if let Some(val) = m0.pop() {
                m1.receive(val);
                changed = true;
            }
        }

        aoclib::output(m1.send_count)
    }
}

#[derive(Debug)]
enum Inst {
    Snd(Source),
    Set(char, Source),
    Add(char, Source),
    Mul(char, Source),
    Mod(char, Source),
    Rcv(char),
    Jgz(Source, Source),
}

impl Inst {
    fn parse(s: &str) -> Self {
        let words = s.split(' ').collect::<Vec<_>>();
        match words[0] {
            "snd" => Inst::Snd(Source::parse(words[1])),
            "set" => Inst::Set(words[1].first_char(), Source::parse(words[2])),
            "add" => Inst::Add(words[1].first_char(), Source::parse(words[2])),
            "mul" => Inst::Mul(words[1].first_char(), Source::parse(words[2])),
            "mod" => Inst::Mod(words[1].first_char(), Source::parse(words[2])),
            "rcv" => Inst::Rcv(words[1].first_char()),
            "jgz" => Inst::Jgz(Source::parse(words[1]), Source::parse(words[2])),
            _ => panic!("invalid instruction"),
        }
    }
}

#[derive(Debug)]
enum Source {
    Reg(char),
    Val(i64),
}

impl Source {
    fn parse(s: &str) -> Self {
        if let Ok(num) = s.parse::<i64>() {
            Source::Val(num)
        } else {
            Source::Reg(s.first_char())
        }
    }

    fn value(&self, reg: &[i64]) -> i64 {
        match self {
            Self::Reg(c) => reg[((*c as u8) - b'a') as usize],
            Self::Val(v) => *v,
        }
    }
}

#[derive(Debug, Default)]
struct Machine<'a> {
    reg: Vec<i64>,
    ip: usize,
    last_sound: VecDeque<i64>,
    recovered_freq: Option<i64>,
    receive_reg: Option<char>,
    send_count: usize,
    prog: &'a [Inst],
}

impl<'a> Machine<'a> {
    fn new(prog: &'a [Inst], p_reg: i64) -> Self {
        let mut machine = Self {
            reg: vec![0; 26],
            prog,
            ..Default::default()
        };
        machine.reg[(b'p' - b'a') as usize] = p_reg;
        machine
    }

    fn set(&mut self, reg: char, val: i64) {
        self.reg[((reg as u8) - b'a') as usize] = val;
    }

    fn get(&self, reg: char) -> i64 {
        self.reg[((reg as u8) - b'a') as usize]
    }

    fn pop(&mut self) -> Option<i64> {
        self.last_sound.pop_front()
    }

    fn receive(&mut self, val: i64) {
        if let Some(reg) = self.receive_reg {
            self.set(reg, val);
        }
        self.receive_reg = None;
    }

    fn step(&mut self) -> bool {
        assert!(self.receive_reg.is_none());

        if self.ip >= self.prog.len() {
            return false;
        }

        self.ip += 1;

        match &self.prog[self.ip - 1] {
            Inst::Snd(source) => {
                self.last_sound.push_back(source.value(&self.reg));
                self.send_count += 1;
            }
            Inst::Set(reg, source) => self.set(*reg, source.value(&self.reg)),
            Inst::Add(reg, source) => {
                let val = source.value(&self.reg) + self.get(*reg);
                self.set(*reg, val);
            }
            Inst::Mul(reg, source) => {
                let val = source.value(&self.reg) * self.get(*reg);
                self.set(*reg, val);
            }
            Inst::Mod(reg, source) => {
                let val = self.get(*reg) % source.value(&self.reg);
                self.set(*reg, val);
            }
            Inst::Rcv(reg) => {
                if self.get(*reg) != 0 {
                    if let Some(&freq) = self.last_sound.back() {
                        self.recovered_freq = Some(freq);
                    }
                }
                self.receive_reg = Some(*reg);
            }
            Inst::Jgz(value, offset) => {
                let value = value.value(&self.reg);
                let offset = offset.value(&self.reg);
                if value > 0 {
                    let dist = offset - 1;
                    if dist < 0 {
                        self.ip -= -dist as usize;
                    } else {
                        self.ip += dist as usize;
                    }
                }
            }
        }

        true
    }
}

trait FirstChar {
    fn first_char(&self) -> char;
}

impl FirstChar for &str {
    fn first_char(&self) -> char {
        self.chars().next().unwrap()
    }
}
