use std::{convert::Infallible, str::FromStr};

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2025_10 {
    machines: Vec<Machine>,
}

impl Aoc2025_10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_10 {
    fn name(&self) -> (usize, usize) {
        (2025, 10)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-10.txt");
        // let lines = aoclib::read_lines("test.txt");
        self.machines = lines.parse_lines();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.machines
                .iter()
                .map(|machine| machine.find_pattern())
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Lights(usize);

impl Lights {
    fn push(&self, button: &Button) -> Lights {
        let mut lights = self.0;
        for b in &button.0 {
            lights ^= 1 << b;
        }
        Lights(lights)
    }
}

impl FromStr for Lights {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lights = 0;
        for (idx, ch) in s.chars().enumerate() {
            if ch == '#' {
                lights |= 1 << (idx - 1);
            }
        }
        Ok(Lights(lights))
    }
}

#[derive(Debug)]
struct Button(Vec<usize>);

impl FromStr for Button {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Button(
            s[1..s.len() - 1]
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect(),
        ))
    }
}

#[derive(Debug)]
struct Machine {
    pattern: Lights,
    buttons: Vec<Button>,
}

impl FromStr for Machine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s.split_whitespace().collect::<Vec<_>>();
        let pattern: Lights = entries[0].parse().unwrap();
        let buttons = entries[1..entries.len() - 1]
            .iter()
            .map(|e| e.parse().unwrap())
            .collect();
        Ok(Self { pattern, buttons })
    }
}

impl Machine {
    fn find_pattern(&self) -> usize {
        aoclib::bfs(
            &(0, Lights(0)),
            |(count, lights): &(usize, Lights)| {
                self.buttons
                    .iter()
                    .map(|b| (count + 1, lights.push(b)))
                    .collect()
            },
            |(_count, lights)| *lights == self.pattern,
        )
        .unwrap()
    }
}
