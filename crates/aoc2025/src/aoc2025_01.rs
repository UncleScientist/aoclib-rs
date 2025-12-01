use std::str::FromStr;

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2025_01 {
    turns: Vec<Turn>,
}

impl Aoc2025_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_01 {
    fn name(&self) -> (usize, usize) {
        (2025, 1)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-01.txt");
        self.turns = lines.parse_lines();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut setting = 50;
        let mut counter = 0;
        for turn in &self.turns {
            match turn {
                Turn::Left(amt) => setting = (setting - amt).rem_euclid(100),
                Turn::Right(amt) => setting = (setting + amt) % 100,
            }
            if setting == 0 {
                counter += 1;
            }
        }
        aoclib::output(counter)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut setting = 50isize;
        let mut counter = 0;
        for turn in &self.turns {
            let (dir, mut amount) = match turn {
                Turn::Left(amt) => (-1, *amt),
                Turn::Right(amt) => (1, *amt),
            };
            while amount > 0 {
                setting = (setting + dir).rem_euclid(100);
                if setting == 0 {
                    counter += 1;
                }
                amount -= 1;
            }
        }
        aoclib::output(counter)
    }
}

#[derive(Debug)]
enum Turn {
    Left(isize),
    Right(isize),
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_left = &s[0..1] == "L";
        let amount = s[1..].parse::<isize>().unwrap();
        Ok(if is_left {
            Turn::Left(amount)
        } else {
            Turn::Right(amount)
        })
    }
}
