use std::{collections::HashMap, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_24 {
    system: HashMap<String, Gate>,
}

impl Aoc2024_24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_value(&self, wire: &str) -> u64 {
        match &self.system[wire] {
            Gate::And(a, b) => self.get_value(a) & self.get_value(b),
            Gate::Or(a, b) => self.get_value(a) | self.get_value(b),
            Gate::Xor(a, b) => self.get_value(a) ^ self.get_value(b),
            Gate::Const(c) => *c,
        }
    }
}

impl Runner for Aoc2024_24 {
    fn name(&self) -> (usize, usize) {
        (2024, 24)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-24.txt");
        for line in lines {
            if let Some((left, right)) = line.split_once(": ") {
                // a value
                self.system.insert(left.into(), right.parse().unwrap());
            } else if let Some((left, right)) = line.split_once(" -> ") {
                // a gate
                self.system.insert(right.into(), left.parse().unwrap());
            } else {
                // a bug
                panic!("bug in line {line}");
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut zs = self
            .system
            .keys()
            .filter(|key| key.starts_with('z'))
            .collect::<Vec<_>>();
        zs.sort();

        let mut result = 0;
        for (shift, z) in zs.iter().enumerate() {
            result |= self.get_value(z) << shift;
        }
        aoclib::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Const(u64),
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<u64>() {
            Ok(Self::Const(value))
        } else {
            let words = s.split_whitespace().collect::<Vec<_>>();
            Ok(match words[1] {
                "AND" => Self::And(words[0].into(), words[2].into()),
                "OR" => Self::Or(words[0].into(), words[2].into()),
                "XOR" => Self::Xor(words[0].into(), words[2].into()),
                _ => panic!("bad gate {}", words[1]),
            })
        }
    }
}
