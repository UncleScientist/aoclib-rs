use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_07 {
    // insert items here (or not, i'm not the boss of you)
}

impl Aoc2018_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_07 {
    fn name(&self) -> (usize, usize) {
        (2018, 7)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
