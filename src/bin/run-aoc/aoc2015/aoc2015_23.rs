use crate::Runner;

pub struct Aoc2015_23 {
    program: Vec<Instruction>,
    machine: Machine,
}

impl Aoc2015_23 {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            machine: Machine { a: 0, b: 0, ip: 0 },
        }
    }

    fn reg(s: &str) -> Register {
        match s {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("input file corrupted"),
        }
    }

    fn run(&mut self) {
        while self.machine.ip < self.program.len() {
            match &self.program[self.machine.ip] {
                Instruction::Hlf(r) => {
                    if *r == Register::A {
                        self.machine.a /= 2;
                    } else {
                        self.machine.b /= 2;
                    }
                    self.machine.ip += 1;
                }

                Instruction::Tpl(r) => {
                    if *r == Register::A {
                        self.machine.a *= 3;
                    } else {
                        self.machine.b *= 3;
                    }
                    self.machine.ip += 1;
                }
                Instruction::Inc(r) => {
                    if *r == Register::A {
                        self.machine.a += 1;
                    } else {
                        self.machine.b += 1;
                    }
                    self.machine.ip += 1;
                }
                Instruction::Jmp(offset) => {
                    self.machine.jump(*offset);
                }
                Instruction::Jie(r, offset) => {
                    let value = if *r == Register::A {
                        self.machine.a
                    } else {
                        self.machine.b
                    };
                    if value % 2 == 0 {
                        self.machine.jump(*offset);
                    } else {
                        self.machine.ip += 1;
                    }
                }
                Instruction::Jio(r, offset) => {
                    let value = if *r == Register::A {
                        self.machine.a
                    } else {
                        self.machine.b
                    };
                    if value == 1 {
                        self.machine.jump(*offset);
                    } else {
                        self.machine.ip += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Register {
    A,
    B,
}

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

struct Machine {
    a: i32,
    b: i32,
    ip: usize,
}

impl Machine {
    fn jump(&mut self, offset: i32) {
        if offset < 0 {
            self.ip -= (-offset) as usize;
        } else {
            self.ip += offset as usize;
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.a = 0;
        self.b = 0;
    }
}

impl Runner for Aoc2015_23 {
    fn name(&self) -> (usize, usize) {
        (2015, 23)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2015-23.txt") {
            let (mnemonic, rest) = line.split_once(' ').unwrap();
            let (reg, offset) = rest.split_once(", ").unwrap_or(("", ""));
            let inst = match mnemonic {
                "hlf" => Instruction::Hlf(Self::reg(rest)),
                "tpl" => Instruction::Tpl(Self::reg(rest)),
                "inc" => Instruction::Inc(Self::reg(rest)),
                "jmp" => Instruction::Jmp(rest.parse().unwrap()),
                "jie" => Instruction::Jie(Self::reg(reg), offset.parse().unwrap()),
                "jio" => Instruction::Jio(Self::reg(reg), offset.parse().unwrap()),
                _ => {
                    panic!("input file corrupted");
                }
            };

            self.program.push(inst)
        }
    }

    fn part1(&mut self) -> Vec<String> {
        self.run();
        crate::output(self.machine.b)
    }

    fn part2(&mut self) -> Vec<String> {
        self.machine.reset();
        self.machine.a = 1;
        self.run();
        crate::output(self.machine.b)
    }
}
