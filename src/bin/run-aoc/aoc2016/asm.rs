use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Inst {
    Cpy(Source, Source),
    Inc(Source),
    Dec(Source),
    Jnz(Source, Source),
    Tgl(Source),
    Out(Source),
}

impl Inst {
    pub fn parse(s: &str) -> Self {
        let tok = s.split(' ').collect::<Vec<&str>>();
        match tok[0] {
            "inc" => Inst::Inc(Source::parse(tok[1])),
            "dec" => Inst::Dec(Source::parse(tok[1])),
            "jnz" => Inst::Jnz(Source::parse(tok[1]), Source::parse(tok[2])),
            "cpy" => Inst::Cpy(Source::parse(tok[1]), Source::parse(tok[2])),
            "tgl" => Inst::Tgl(Source::parse(tok[1])),
            "out" => Inst::Out(Source::parse(tok[1])),
            _ => panic!("corrupted input file {tok:?}"),
        }
    }
    pub fn toggle(&self) -> Self {
        match self {
            Inst::Inc(r) => Inst::Dec(r.clone()),
            Inst::Dec(r) => Inst::Inc(r.clone()),
            Inst::Jnz(s, d) => Inst::Cpy(s.clone(), d.clone()),
            Inst::Cpy(s, d) => Inst::Jnz(s.clone(), d.clone()),
            Inst::Tgl(d) => Inst::Inc(d.clone()),
            Inst::Out(d) => Inst::Inc(d.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Source {
    Reg(Register),
    Val(i32),
}

impl Source {
    fn parse(s: &str) -> Self {
        if let Ok(reg) = Register::parse(s) {
            Source::Reg(reg)
        } else {
            Source::Val(s.parse().unwrap())
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn parse(s: &str) -> Result<Self, ()> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(()),
        }
    }
}

#[derive(Default, Clone)]
struct State {
    ip: usize,
    reg: [i32; 4],
    togg: Vec<bool>,
}

#[derive(Default, Clone)]
pub struct Machine {
    state: State,
    output: Option<i32>,
    prog: Vec<Inst>,
    history: HashSet<State>,
    looped: bool,
}

impl Hash for State {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.ip.hash(state);
        self.reg.iter().for_each(|x| x.hash(state));
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip && self.reg == other.reg
    }
}

impl Eq for State {}

impl Machine {
    pub fn push(&mut self, inst: Inst) {
        self.prog.push(inst);
    }

    pub fn looped(&self) -> bool {
        self.looped
    }

    pub fn reset(&mut self, initial_state: Vec<(Register, i32)>) {
        self.state.reg = [0, 0, 0, 0];
        self.state.togg = vec![false; self.prog.len()];
        for i in initial_state {
            self.state.reg[i.0 as usize] = i.1;
        }
        self.state.ip = 0;
        self.history.clear();
        self.looped = false;
    }

    pub fn run_to_output(&mut self) -> Option<i32> {
        if !self.looped {
            self.looped = !self.history.insert(self.state.clone());
        }

        self.output = None;

        while self.state.ip < self.prog.len() {
            self.step();
            if self.output.is_some() {
                return self.output;
            }
        }

        None
    }

    pub fn run(&mut self, initial_state: Vec<(Register, i32)>) -> i32 {
        self.reset(initial_state);

        while self.state.ip < self.prog.len() {
            self.step();
        }

        self.state.reg[0]
    }

    fn step(&mut self) {
        let oldip = self.state.ip;
        self.state.ip += 1;
        let inst = if self.state.togg[oldip] {
            self.prog[oldip].toggle()
        } else {
            self.prog[oldip].clone()
        };
        match inst {
            Inst::Inc(Source::Reg(r)) => self.state.reg[r as usize] += 1,
            Inst::Inc(Source::Val(_)) => { /* invalid */ }
            Inst::Dec(Source::Reg(r)) => self.state.reg[r as usize] -= 1,
            Inst::Dec(Source::Val(_)) => { /* invalid */ }
            Inst::Cpy(Source::Reg(r), Source::Reg(dest)) => {
                self.state.reg[dest as usize] = self.state.reg[r as usize]
            }
            Inst::Cpy(Source::Val(v), Source::Reg(dest)) => self.state.reg[dest as usize] = v,
            Inst::Cpy(_, Source::Val(_)) => { /* invalid */ }
            Inst::Jnz(src, dest) => {
                let val = match src {
                    Source::Reg(r) => self.state.reg[r as usize],
                    Source::Val(v) => v,
                };
                let dest = match dest {
                    Source::Reg(r) => self.state.reg[r as usize],
                    Source::Val(v) => v,
                };
                if val != 0 {
                    if dest < 0 {
                        self.state.ip = oldip - (-dest) as usize;
                    } else {
                        self.state.ip = oldip + dest as usize;
                    }
                }
            }
            Inst::Tgl(src) => {
                let dest = match src {
                    Source::Reg(r) => self.state.reg[r as usize],
                    Source::Val(v) => v,
                };
                let loc = if dest < 0 {
                    oldip - (-dest) as usize
                } else {
                    oldip + dest as usize
                };
                if loc < self.state.togg.len() {
                    self.state.togg[loc] = !self.state.togg[loc];
                }
            }
            Inst::Out(src) => {
                self.output = Some(match src {
                    Source::Reg(r) => self.state.reg[r as usize],
                    Source::Val(v) => v,
                })
            }
        }
    }
}
