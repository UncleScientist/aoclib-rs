use std::{collections::HashMap, str::FromStr};

use aoclib::Runner;

type Price = i64;

#[derive(Default)]
pub struct Aoc2024_22 {
    prices: Vec<Vec<Price>>,
}

impl Aoc2024_22 {
    pub fn new() -> Self {
        Self::default()
    }

    fn build_secrets(&mut self, secrets: &mut [Secret]) {
        for secret in secrets {
            let mut list = vec![secret.0];
            list.extend(secret.take(2000));
            self.prices.push(list);
        }
    }

    fn bananas(&mut self) -> usize {
        let mut sequences = HashMap::<[i64; 4], usize>::new();
        let mut deltas = Vec::new();
        for price in &self.prices {
            let delta = price
                .windows(2)
                .map(|pair| (pair[1] % 10) - (pair[0] % 10))
                .collect::<Vec<_>>();
            for quad in delta.windows(4) {
                *sequences
                    .entry([quad[0], quad[1], quad[2], quad[3]])
                    .or_default() += 1;
            }
            deltas.push(delta);
        }

        let mut sequences = sequences.iter().collect::<Vec<_>>();
        sequences.sort_by(|a, b| b.1.cmp(a.1));

        let mut max = 0;
        for (seq, count) in sequences {
            if count * 9 < max {
                break;
            }
            let mut total = 0;
            for (which, delta) in deltas.iter().enumerate() {
                if let Some((index, _)) = delta.windows(4).enumerate().find(|(_, quad)| {
                    quad[0] == seq[0] && quad[1] == seq[1] && quad[2] == seq[2] && quad[3] == seq[3]
                }) {
                    total += (self.prices[which][index + 4] % 10) as usize;
                }
            }
            max = max.max(total);
        }

        max
    }
}

impl Runner for Aoc2024_22 {
    fn name(&self) -> (usize, usize) {
        (2024, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-22.txt");
        let mut secrets: Vec<Secret> = lines.iter().map(|line| line.parse().unwrap()).collect();
        self.build_secrets(&mut secrets);
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self
            .prices
            .iter()
            .map(|list| list.last().unwrap())
            .sum::<Price>();
        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.bananas())
    }
}

#[derive(Debug, Clone)]
struct Secret(Price);

impl FromStr for Secret {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().unwrap()))
    }
}

impl Iterator for Secret {
    type Item = Price;

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

    #[test]
    fn test_four_monkies() {
        let mut secrets = vec![Secret(1), Secret(2), Secret(3), Secret(2024)];
        let mut aoc = Aoc2024_22::default();
        aoc.build_secrets(&mut secrets);

        assert_eq!(23, aoc.bananas());
    }
}
