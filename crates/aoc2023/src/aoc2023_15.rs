use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_15 {
    steps: Vec<String>,
}

impl Aoc2023_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_15 {
    fn name(&self) -> (usize, usize) {
        (2023, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-15.txt");
        self.steps = lines[0].split(',').map(|s| s.to_string()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.steps.iter().fold(0, |acc, step| acc + step.aoc_hash()))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

trait AocHash {
    fn aoc_hash(&self) -> usize;
}

impl AocHash for String {
    fn aoc_hash(&self) -> usize {
        self.chars()
            .fold(0, |acc, ch| ((ch as usize + acc) * 17) % 256)
    }
}
