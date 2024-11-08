use aoclib::Runner;

use crate::intcode::Intcode;

#[derive(Default)]
pub struct Aoc2019_05 {
    computer: Intcode,
}

impl Aoc2019_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_05 {
    fn name(&self) -> (usize, usize) {
        (2019, 5)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2019-05.txt");
        self.computer = Intcode::new(&lines[0]);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.run_computer_with(1))
    }

    fn part2(&mut self) -> Vec<String> {
        self.computer.reset();
        aoclib::output(self.run_computer_with(5))
    }
}

impl Aoc2019_05 {
    fn run_computer_with(&mut self, val: i64) -> i64 {
        self.computer.push(val);
        while let Some(val) = self.computer.run_until_output() {
            if val != 0 {
                assert!(self.computer.run_until_output().is_none());
                return val;
            }
        }
        panic!("no solution found");
    }
}
