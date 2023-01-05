use crate::Runner;

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
        // let lines = aoclib::read_lines("test-input.txt");

        for line in &lines {
            self.inst.push(Inst::parse(line))
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut machine = Machine::new(&self.inst);

        while machine.recovered_freq.is_none() {
            machine.step();
        }

        crate::output(machine.recovered_freq.unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug)]
enum Inst {
    Snd(Source),
    Set(char, Source),
    Add(char, Source),
    Mul(char, Source),
    Mod(char, Source),
    Rcv(Source),
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
            "rcv" => Inst::Rcv(Source::parse(words[1])),
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
    last_sound: i64,
    recovered_freq: Option<i64>,
    prog: &'a [Inst],
}

impl<'a> Machine<'a> {
    fn new(prog: &'a [Inst]) -> Self {
        Self {
            reg: vec![0; 26],
            prog,
            ..Default::default()
        }
    }

    fn set(&mut self, reg: char, val: i64) {
        self.reg[((reg as u8) - b'a') as usize] = val;
    }

    fn get(&self, reg: char) -> i64 {
        self.reg[((reg as u8) - b'a') as usize]
    }

    fn step(&mut self) {
        self.ip += 1;
        if self.ip >= self.prog.len() {
            return;
        }

        // print!("{}: {:?} -> ", self.ip, self.prog[self.ip]);

        match &self.prog[self.ip - 1] {
            Inst::Snd(source) => self.last_sound = source.value(&self.reg),
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
            Inst::Rcv(source) => {
                if source.value(&self.reg) != 0 {
                    self.recovered_freq = Some(self.last_sound);
                }
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
                    // println!("offset = {offset}, new_ip = {}", self.ip);
                }
            }
        }
        // println!("{:?}", self.reg);
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
