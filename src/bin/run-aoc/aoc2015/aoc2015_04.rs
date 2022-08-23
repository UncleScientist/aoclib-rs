use crate::Runner;

pub struct Aoc2015_04 {
    lowest: usize,
}

impl Aoc2015_04 {
    pub fn new() -> Self {
        Self { lowest: 1 }
    }

    fn find_prefix(&mut self, prefix: &str) -> Vec<String> {
        loop {
            let digest = md5::compute(format!("{PREFIX}{}", self.lowest));
            let string_digest = format!("{digest:x}");
            if string_digest.starts_with(prefix) {
                return vec![format!("{}", self.lowest)];
            }
            self.lowest += 1;
        }
    }
}

const PREFIX: &str = "ckczppom";

impl Runner for Aoc2015_04 {
    fn name(&self) -> (usize, usize) {
        (2015, 4)
    }

    fn part1(&mut self) -> Vec<String> {
        self.find_prefix("00000")
    }

    fn part2(&mut self) -> Vec<String> {
        self.find_prefix("000000")
    }
}
