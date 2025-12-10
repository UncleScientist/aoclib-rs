use std::{
    collections::{HashSet, VecDeque},
    convert::Infallible,
    str::FromStr,
};

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
        // let lines = aoclib::read_lines("input/2025-10.txt");
        let lines = aoclib::read_lines("first.txt");
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
        let jolts = entries[entries.len() - 1];
        let joltage = jolts[1..jolts.len() - 1]
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();
        Ok(Self {
            pattern,
            buttons,
            joltage,
        })
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

    fn find_joltage(&self) -> usize {
        fn push(
            limit: &[usize],
            joltages: &[usize],
            button: &Button,
            pushes: usize,
        ) -> Option<Vec<usize>> {
            let mut new_jolts = joltages.to_vec();
            for b in &button.0 {
                new_jolts[*b] += pushes;
            }
            if new_jolts.iter().zip(limit).any(|(a, b)| a > b) {
                None
            } else {
                Some(new_jolts)
            }
        }

        let mut tried = HashSet::new();
        let mut starting_stage = vec![(0, vec![0usize; self.joltage.len()])];
        println!("{:?}", self.joltage);

        while tried.len() < self.joltage.len() {
            let mut queue: VecDeque<(usize, Vec<usize>)> = VecDeque::from(starting_stage);
            let mut next_stage = Vec::new();

            let (idx, &smallest) = self
                .joltage
                .iter()
                .enumerate()
                .filter(|(idx, _)| !tried.contains(idx))
                .min_by(|a, b| a.1.cmp(b.1))
                .unwrap();

            println!("  working on {idx}");

            let mut visited = HashSet::new();
            let usable_buttons = self
                .buttons
                .iter()
                .filter(|b| !b.0.iter().any(|button| tried.contains(button)))
                .filter(|b| b.0.contains(&idx))
                .collect::<Vec<_>>();
            println!("  > {usable_buttons:?}");

            tried.insert(idx);

            while let Some((count, jolts)) = queue.pop_front() {
                if jolts[idx] == smallest {
                    next_stage.push((count, jolts.clone()));
                } else if visited.insert(jolts.clone()) {
                    let _pushes = self.joltage[idx] - jolts[idx];
                    for button in &usable_buttons {
                        if let Some(new_state) = push(&self.joltage, &jolts, button, 1) {
                            queue.push_back((count + 1, new_state));
                        }
                    }
                }
            }

            println!("  > found {} possibilities", next_stage.len());
            assert!(next_stage.len() > 0);
            for s in &next_stage {
                println!("    * {s:?}");
            }
            starting_stage = next_stage
        }

        *starting_stage.iter().map(|(c, _)| c).min().unwrap()
    }
}
