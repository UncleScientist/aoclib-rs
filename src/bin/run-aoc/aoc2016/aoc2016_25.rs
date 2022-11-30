use crate::aoc2016::asm::*;
use crate::Runner;

#[derive(Default)]
pub struct Aoc2016_25 {
    vm: Machine,
}

impl Aoc2016_25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2016_25 {
    fn name(&self) -> (usize, usize) {
        (2016, 25)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-25.txt");
        for l in lines {
            self.vm.push(Inst::parse(&l));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        'next_number: for init in 1i32.. {
            self.vm.reset(vec![(Register::A, init)]);
            let mut last_output = 1;
            let answer = loop {
                let Some(next_output) = self.vm.run_to_output()
                    else {
                        continue 'next_number;
                    };
                if next_output == last_output {
                    continue 'next_number;
                }
                last_output = next_output;
                if self.vm.looped() {
                    break init;
                }
            };
            return crate::output(answer);
        }
        crate::output("failed")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unneeded")
    }
}
