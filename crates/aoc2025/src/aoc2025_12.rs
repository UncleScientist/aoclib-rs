use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_12 {
    // insert items here (or not, i'm not the boss of you)
}

impl Aoc2025_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_12 {
    fn name(&self) -> (usize, usize) {
        (2025, 12)
    }

    fn parse(&mut self) {
        // let lines = aoclib::read_lines("input/2025-12.txt");
        // self.data = lines.parse_lines(); // 'use aoclib::LineParser;'
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
