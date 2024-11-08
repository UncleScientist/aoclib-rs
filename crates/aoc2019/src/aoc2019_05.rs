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
        self.computer.push(1);
        while let Some(val) = self.computer.run_until_output() {
            if val != 0 {
                assert!(self.computer.run_until_output().is_none());
                return aoclib::output(val);
            }
        }
        panic!("no solution found");
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
