use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_01 {
    nums: Vec<i64>,
}

impl Aoc2018_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_01 {
    fn name(&self) -> (usize, usize) {
        (2018, 1)
    }

    fn parse(&mut self) {
        self.nums = aoclib::read_lines("input/2018-01.txt")
            .into_iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.nums.iter().sum::<i64>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut freq = HashSet::new();
        let mut cur_freq = 0;

        loop {
            for delta in &self.nums {
                cur_freq += *delta;
                if !freq.insert(cur_freq) {
                    return aoclib::output(cur_freq);
                }
            }
        }
    }
}
