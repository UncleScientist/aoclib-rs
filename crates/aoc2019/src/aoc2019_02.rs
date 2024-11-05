use aoclib::Runner;

use crate::intcode::*;

#[derive(Default)]
pub struct Aoc2019_02 {
    computer: Intcode,
}

impl Aoc2019_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_02 {
    fn name(&self) -> (usize, usize) {
        (2019, 2)
    }

    fn parse(&mut self) {
        let input = std::fs::read_to_string("input/day02.txt").expect("file not found");
        let (intdata, _) = input.split_once('\n').expect("missing newline");
        self.computer = Intcode::new_from_string(&intdata);
    }

    fn part1(&mut self) -> Vec<String> {
        self.computer.poke(1, 12);
        self.computer.poke(2, 2);
        self.computer.run();
        aoclib::output(self.computer.peek(0))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
