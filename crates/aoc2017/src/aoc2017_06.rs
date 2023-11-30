use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_06 {
    banks: Vec<usize>,
}

impl Aoc2017_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_06 {
    fn name(&self) -> (usize, usize) {
        (2017, 6)
    }

    fn parse(&mut self) {
        let mut nums: Vec<Vec<usize>> = aoclib::numbers("input/2017-06.txt", '\t');
        self.banks = nums.pop().unwrap();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut state = HashSet::new();

        aoclib::output(loop_till_repeat(&mut self.banks, &mut state))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut state = HashSet::new();
        state.insert(self.banks.clone());

        aoclib::output(loop_till_repeat(&mut self.banks, &mut state))
    }
}

fn loop_till_repeat(banks: &mut Vec<usize>, state: &mut HashSet<Vec<usize>>) -> usize {
    let mut cycles = 0;

    loop {
        let (mut bank, &(mut count)) = banks
            .iter()
            .rev()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap();

        bank = banks.len() - bank - 1;
        banks[bank] = 0;
        while count > 0 {
            bank = (bank + 1) % banks.len();
            banks[bank] += 1;
            count -= 1;
        }
        cycles += 1;
        if !state.insert(banks.clone()) {
            break;
        }
    }

    cycles
}
