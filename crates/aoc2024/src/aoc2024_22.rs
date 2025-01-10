use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_22 {
    secrets: Vec<Secret>,
}

impl Aoc2024_22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_22 {
    fn name(&self) -> (usize, usize) {
        (2024, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-22.txt");
        self.secrets = lines.iter().map(|line| line.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self
            .secrets
            .iter_mut()
            .map(|secret| secret.nth(1999).unwrap())
            .sum::<u64>();
        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, Clone)]
struct Secret(u64);

impl FromStr for Secret {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().unwrap()))
    }
}

impl Iterator for Secret {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let step1 = ((self.0 * 64) ^ self.0) % 16777216;
        let step2 = ((step1 / 32) ^ step1) % 16777216;
        let step3 = ((step2 * 2048) ^ step2) % 16777216;
        self.0 = step3;
        Some(step3)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_secret_1() {
        let mut secret = Secret(1);
        let two_thousandth = secret.nth(1999).unwrap();
        assert_eq!(8685429, two_thousandth);
    }
}
