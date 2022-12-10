use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_10 {
    stmt: Vec<Instruction>,
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
        let mut xreg = 1;
        let mut cycle = 0;
        let mut check = 20;

        for s in &self.stmt {
            match s {
                Instruction::Noop => {
                    cycle += 1;
                    if cycle >= check {
                        strength += cycle * xreg;
                        check += 40;
                    }
                }
                Instruction::AddX(val) => {
                    cycle += 2;
                    if cycle >= check {
                        strength += check * xreg;
                        check += 40;
                    }

                    xreg += val;
                }
            }
        }
        crate::output(strength)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}
