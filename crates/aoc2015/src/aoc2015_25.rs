use aoclib::power_mod;
use aoclib::Runner;

pub struct Aoc2015_25;

impl Aoc2015_25 {
    pub fn new() -> Self {
        Self {}
    }

    fn num_at_rc(row: u64, col: u64) -> u64 {
        1 + (col + row - 1) * (col + row) / 2 - row
    }
}

impl Runner for Aoc2015_25 {
    fn name(&self) -> (usize, usize) {
        (2015, 25)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let row = 2978;
        let col = 3083;
        aoclib::output(power_mod(
            20151125,
            252533,
            Self::num_at_rc(row, col) - 1,
            33554393,
        ))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("ta-da!")
    }
}
