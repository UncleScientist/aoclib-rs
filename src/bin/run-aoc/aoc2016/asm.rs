#[derive(Debug, Clone)]
pub enum Inst {
    Cpy(Source, Source),
    Inc(Source),
    Dec(Source),
    Jnz(Source, Source),
    Tgl(Source),
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

#[derive(Default)]
pub struct Machine {
    prog: Vec<Inst>,
    togg: Vec<bool>,
}

impl Machine {
    pub fn push(&mut self, inst: Inst) {
        self.prog.push(inst);
    }

    pub fn run(&mut self, initial_state: Vec<(Register, i32)>) -> i32 {
        let mut reg: [i32; 4] = [0, 0, 0, 0];
        let mut ip = 0;

        for i in initial_state {
            reg[i.0 as usize] = i.1;
        }
        self.togg = vec![false; self.prog.len()];

        while ip < self.prog.len() {
            let oldip = ip;
            ip += 1;
            let inst = if self.togg[oldip] {
                self.prog[oldip].toggle()
            } else {
                self.prog[oldip].clone()
            };
            match inst {
                Inst::Inc(Source::Reg(r)) => reg[r as usize] += 1,
                Inst::Dec(Source::Reg(r)) => reg[r as usize] -= 1,
                Inst::Cpy(Source::Reg(r), Source::Reg(dest)) => {
                    reg[dest as usize] = reg[r as usize]
                }
                Inst::Cpy(Source::Val(v), Source::Reg(dest)) => reg[dest as usize] = v,
                Inst::Jnz(src, dest) => {
                    let val = match src {
                        Source::Reg(r) => reg[r as usize],
                        Source::Val(v) => v,
                    };
                    let dest = match dest {
                        Source::Reg(r) => reg[r as usize],
                        Source::Val(v) => v,
                    };
                    if val != 0 {
                        if dest < 0 {
                            ip = oldip - (-dest) as usize;
                        } else {
                            ip = oldip + dest as usize;
                        }
                    }
                }
                Inst::Tgl(src) => {
                    let dest = match src {
                        Source::Reg(r) => reg[r as usize],
                        Source::Val(v) => v,
                    };
                    let loc = if dest < 0 {
                        oldip - (-dest) as usize
                    } else {
                        oldip + dest as usize
                    };
                    if loc < self.togg.len() {
                        self.togg[loc] = !self.togg[loc];
                    }
                }
                _ =>
                    /* ignore */
                    {}
            }
        }

        reg[0]
    }
}
