use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_23 {
    connections: Vec<Connection>,
}

impl Aoc2024_23 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_23 {
    fn name(&self) -> (usize, usize) {
        (2024, 23)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-23.txt");
        self.connections = lines.iter().map(|line| line.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut network = HashMap::<String, Vec<String>>::new();
        for pair in &self.connections {
            network
                .entry(pair.left.clone())
                .or_default()
                .push(pair.right.clone());
            network
                .entry(pair.right.clone())
                .or_default()
                .push(pair.left.clone());
        }

        let mut answer = HashSet::new();
        for (computer, connections) in network
            .iter()
            .filter(|(computer, _)| computer.starts_with("t"))
        {
            for i in 0..connections.len() - 1 {
                for j in i + 1..connections.len() {
                    if network[&connections[i]].contains(&connections[j]) {
                        let mut v = vec![
                            computer.clone(),
                            connections[i].clone(),
                            connections[j].clone(),
                        ];
                        v.sort();
                        answer.insert(v);
                    }
                }
            }
        }

        aoclib::output(answer.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

struct Connection {
    left: String,
    right: String,
}

impl FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').unwrap();
        Ok(Self {
            left: left.into(),
            right: right.into(),
        })
    }
}
