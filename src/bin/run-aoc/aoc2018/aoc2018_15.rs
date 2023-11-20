use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_15 {
    // insert items here (or not, i'm not the boss of you)
}

impl Aoc2018_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_15 {
    fn name(&self) -> (usize, usize) {
        (2018, 15)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
