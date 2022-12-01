use crate::Runner;

pub struct Aoc2017_01;

impl Aoc2017_01 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2017_01 {
    fn name(&self) -> (usize, usize) {
        (2017, 1)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
