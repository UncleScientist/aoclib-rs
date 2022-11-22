use crate::Runner;

use crate::aoc2016::asm::*;

#[derive(Default)]
pub struct Aoc2016_23 {
    vm: Machine,
}

impl Aoc2016_23 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Runner for Aoc2016_23 {
    fn name(&self) -> (usize, usize) {
        (2016, 23)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-23.txt");
        let _lines = "cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a"
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        for l in lines {
            self.vm.push(Inst::parse(&l));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.vm.run(vec![(Register::A, 7)]))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
