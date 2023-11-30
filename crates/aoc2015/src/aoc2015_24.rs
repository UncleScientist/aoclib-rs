use aoclib::CombinationsOf;

use aoclib::Runner;

pub struct Aoc2015_24 {
    gifts: Vec<i64>,
}

impl Aoc2015_24 {
    pub fn new() -> Self {
        Self { gifts: Vec::new() }
    }

    fn qe_of_division(&self, divisor: i64) -> Option<i64> {
        let target = self.gifts.iter().sum::<i64>() / divisor;

        let mut done = false;
        let mut qe = i64::MAX;
        for n in 1..self.gifts.len() {
            for combo in self.gifts.combinations_of(n) {
                let sum: i64 = combo.iter().sum();
                if sum == target {
                    qe = qe.min(combo.iter().product());
                    done = true;
                }
            }
            if done {
                return Some(qe);
            }
        }

        None
    }
}

impl Runner for Aoc2015_24 {
    fn name(&self) -> (usize, usize) {
        (2015, 24)
    }

    fn parse(&mut self) {
        self.gifts = aoclib::read_lines("input/2015-24.txt")
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.qe_of_division(3).unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.qe_of_division(4).unwrap())
    }
}
