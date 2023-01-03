use crate::Runner;

use super::utils::knot_hash;

#[derive(Default)]
pub struct Aoc2017_14 {
    key_string: String,
}

impl Aoc2017_14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_14 {
    fn name(&self) -> (usize, usize) {
        (2017, 14)
    }

    fn parse(&mut self) {
        // self.key_string = "flqrgnkx".to_string();
        self.key_string = "jzgqcdpd".to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(
            (0..128)
                .map(|n| knot_hash(&format!("{}-{n}", self.key_string)))
                .fold(0, |a, b| a + b.count_ones()),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
