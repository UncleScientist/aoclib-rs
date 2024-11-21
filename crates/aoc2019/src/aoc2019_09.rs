use aoclib::Runner;

use crate::intcode::Intcode;

#[derive(Default)]
pub struct Aoc2019_09 {
    computer: Intcode,
}

impl Aoc2019_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_09 {
    fn name(&self) -> (usize, usize) {
        (2019, 9)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2019-09.txt");
        self.computer = Intcode::new(&lines[0]);
    }

    fn part1(&mut self) -> Vec<String> {
        self.computer.push(1);
        let answer = self.computer.run_until_output().unwrap();
        if self.computer.run_until_output().is_some() {
            panic!("Error, with answer = {answer}");
        }
        aoclib::output(answer)
    }

    fn part2(&mut self) -> Vec<String> {
        self.computer.reset();
        self.computer.push(2);
        aoclib::output(self.computer.run_until_output().unwrap())
    }
}
