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
        aoclib::output(
            self.machines
                .iter()
                .map(|machine| machine.find_joltage())
                .sum::<usize>(),
        )
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
    joltage: Vec<usize>,
    combinations: Vec<(usize, Vec<usize>)>, // presses, vec<amounts>
}

impl FromStr for Machine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s.split_whitespace().collect::<Vec<_>>();
        let pattern: Lights = entries[0].parse().unwrap();
        let buttons: Vec<Button> = entries[1..entries.len() - 1]
            .iter()
            .map(|e| e.parse().unwrap())
            .collect();
        let jolts = entries[entries.len() - 1];
        let joltage: Vec<usize> = jolts[1..jolts.len() - 1]
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();
        let mut combinations = Vec::new();
        for bitmask in 0..2_usize.pow(buttons.len() as u32) {
            let mut combo = vec![0usize; joltage.len()];
            for (idx, button) in buttons.iter().enumerate() {
                if (1 << idx) & bitmask == 0 {
                    continue;
                }
                for b in &button.0 {
                    combo[*b] += 1;
                }
            }
            combinations.push((bitmask.count_ones() as usize, combo));
        }

        Ok(Self {
            pattern,
            buttons,
            joltage,
            combinations,
        })
    }
}

//  9, 12, 18
//  one press to get to: 8, 12, 18
//  divide by 2: 4, 6, 9
//  n presses to get to 4, 6, 9, multiply by 2
//
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

    fn count_presses(&self, goal: &[usize]) -> Option<usize> {
        if goal.iter().all(|g| *g == 0) {
            return Some(0);
        }

        let mut minimum: Option<usize> = None;

        for (presses, combos) in &self.combinations {
            if goal.iter().zip(combos).any(|(g, c)| g < c) {
                continue;
            }
            let next_goal = goal
                .iter()
                .zip(combos)
                .map(|(g, c)| g - c)
                .collect::<Vec<_>>();
            if next_goal.iter().all(|g| g.is_multiple_of(2)) {
                let next_goal = next_goal.iter().map(|ng| ng / 2).collect::<Vec<_>>();
                if let Some(answer) = self.count_presses(&next_goal) {
                    let total = presses + 2 * answer;
                    if minimum.is_none() || Some(total) < minimum {
                        minimum.replace(total);
                    }
                }
            }
        }

        minimum
    }

    fn find_joltage(&self) -> usize {
        self.count_presses(&self.joltage).unwrap()
    }
}
