use aoclib::Runner;

use crate::asm::*;

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
            self.vm.push(Inst::parse(&l));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.vm.run(vec![(Register::C, 0)]))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.vm.run(vec![(Register::C, 1)]))
    }
}
