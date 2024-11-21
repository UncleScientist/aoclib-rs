use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_09 {
    // insert items here (or not, i'm not the boss of you)
}

impl Aoc2019_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_09 {
    fn name(&self) -> (usize, usize) {
        (2019, 9)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
