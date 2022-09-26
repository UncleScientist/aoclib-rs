use aoclib::CombinationsOf;

use crate::Runner;

pub struct Aoc2015_24;

impl Aoc2015_24 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2015_24 {
    fn name(&self) -> (usize, usize) {
        (2015, 24)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let gifts: Vec<i64> = aoclib::read_lines("input/2015-24.txt")
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let target = gifts.iter().sum::<i64>() / 3;

        let mut done = false;
        let mut qe = i64::MAX;
        for n in 1..gifts.len() {
            for combo in gifts.combinations_of(n) {
                let sum: i64 = combo.iter().sum();
                if sum == target {
                    qe = qe.min(combo.iter().product());
                    done = true;
                }
            }
            if done {
                break;
            }
        }
        crate::output(qe)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
