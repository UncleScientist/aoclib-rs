use std::str::FromStr;

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2025_03 {
    power_banks: Vec<PowerBank>,
}

impl Aoc2025_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_03 {
    fn name(&self) -> (usize, usize) {
        (2025, 3)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-03.txt");
        self.power_banks = lines.parse_lines();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.power_banks
                .iter()
                .map(|bank| bank.best_number_by_digits(2))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(
            self.power_banks
                .iter()
                .map(|bank| bank.best_number_by_digits(12))
                .sum::<usize>(),
        )
    }
}

#[derive(Debug)]
struct PowerBank {
    bank: Vec<u8>,
}

impl PowerBank {
    fn best_number_by_digits(&self, digits: usize) -> usize {
        let mut start = 0;
        let mut answer = 0;
        for digit in 0..digits {
            let limit = digits - digit - 1;
            let Some((pos, &next_digit)) = self.bank[start..self.bank.len() - limit]
                .iter()
                .enumerate()
                .rev()
                .max_by(|a, b| a.1.cmp(b.1))
            else {
                panic!("no digits found");
            };
            start += pos + 1;
            answer = (answer * 10) + next_digit as usize;
        }
        answer
    }
}

impl FromStr for PowerBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PowerBank {
            bank: s.chars().map(|ch| ch as u8 - b'0').collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_data = [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];
        for test in test_data {
            let bank: PowerBank = test.0.parse().unwrap();
            assert_eq!(test.1, bank.best_number_by_digits(2));
        }
    }

    #[test]
    fn test_part_2() {
        let test_data = [
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
        ];
        for test in test_data {
            let bank: PowerBank = test.0.parse().unwrap();
            assert_eq!(test.1, bank.best_number_by_digits(12));
        }
    }
}
