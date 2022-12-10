use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_10 {
    stmt: Vec<Instruction>,
    machine: Machine,
}

impl Aoc2022_10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_10 {
    fn name(&self) -> (usize, usize) {
        (2022, 10)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-10.txt");

        for line in lines {
            let words = line.split(' ').collect::<Vec<&str>>();

            self.stmt.push(match words[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(words[1].parse().unwrap()),
                _ => panic!("unknown mnemonic {}", words[0]),
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut strength = 0;
        let mut check = 20;

        self.machine.init();

        for s in &self.stmt {
            match s {
                Instruction::Noop => {
                    self.machine.noop();
                    if self.machine.cycle >= check {
                        strength += check * self.machine.xreg;
                        check += 40;
                    }
                }
                Instruction::AddX(val) => {
                    let prev_x = self.machine.xreg;
                    self.machine.addx(*val);
                    if self.machine.cycle >= check {
                        strength += check * prev_x;
                        check += 40;
                    }
                }
            }
        }

        self.machine.noop();

        crate::output(strength)
    }

    fn part2(&mut self) -> Vec<String> {
        self.machine.crt.clone()
    }
}

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

#[derive(Default)]
struct Machine {
    xreg: i32,
    cycle: i32,
    scanline: String,
    crt: Vec<String>,
}

impl Machine {
    fn init(&mut self) {
        self.xreg = 1;
        self.cycle = 0;
        self.scanline = "".to_string();
        self.crt.clear();
    }

    fn noop(&mut self) {
        self.cycle();
    }

    fn addx(&mut self, delta: i32) {
        self.cycle();
        self.cycle();
        self.xreg += delta;
    }

    fn cycle(&mut self) {
        if self.cycle % 40 == 0 && !self.scanline.is_empty() {
            self.crt.push(self.scanline.clone());
            self.scanline.clear();
        }
        if self.cycle % 40 < self.xreg - 1 || self.cycle % 40 > self.xreg + 1 {
            self.scanline.push('.')
        } else {
            self.scanline.push('#');
        }
        self.cycle += 1;
    }
}
