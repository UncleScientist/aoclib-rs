use crate::Runner;

#[derive(Debug)]
enum Inst {
    Cpy(Source, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Source, i32),
}

impl Inst {
    fn parse(s: &str) -> Self {
        let tok = s.split(' ').collect::<Vec<&str>>();
        match tok[0] {
            "inc" => Inst::Inc(Register::parse(tok[1]).unwrap()),
            "dec" => Inst::Dec(Register::parse(tok[1]).unwrap()),
            "jnz" => Inst::Jnz(Source::parse(tok[1]), tok[2].parse().unwrap()),
            "cpy" => Inst::Cpy(Source::parse(tok[1]), Register::parse(tok[2]).unwrap()),
            _ => panic!("corrupted input file {tok:?}"),
        }
    }
}

#[derive(Debug)]
enum Source {
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
enum Register {
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
struct Machine {
    prog: Vec<Inst>,
}

impl Machine {
    fn run(&mut self, first_c: i32) -> i32 {
        let mut reg: [i32; 4] = [0, 0, first_c, 0];
        let mut ip = 0;

        while ip < self.prog.len() {
            let oldip = ip;
            ip += 1;
            // println!("{reg:?} {ip:02} {:?}", self.prog[oldip]);
            match &self.prog[oldip] {
                Inst::Inc(r) => reg[*r as usize] += 1,
                Inst::Dec(r) => reg[*r as usize] -= 1,
                Inst::Cpy(Source::Reg(r), dest) => reg[*dest as usize] = reg[*r as usize],
                Inst::Cpy(Source::Val(v), dest) => reg[*dest as usize] = *v,
                Inst::Jnz(src, dest) => {
                    let val = match src {
                        Source::Reg(r) => reg[*r as usize],
                        Source::Val(v) => *v,
                    };
                    if val != 0 {
                        if *dest < 0 {
                            ip = oldip - (-*dest) as usize;
                        } else {
                            ip = oldip + *dest as usize;
                        }
                    }
                }
            }
        }

        reg[0]
    }
}

pub struct Aoc2016_12 {
    vm: Machine,
}

impl Aoc2016_12 {
    pub fn new() -> Self {
        Self {
            vm: Machine::default(),
        }
    }
}

impl Runner for Aoc2016_12 {
    fn name(&self) -> (usize, usize) {
        (2016, 12)
    }

    fn parse(&mut self) {
        let _lines = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a"
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let lines = aoclib::read_lines("input/2016-12.txt");

        for l in lines {
            self.vm.prog.push(Inst::parse(&l));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.vm.run(0))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.vm.run(1))
    }
}
