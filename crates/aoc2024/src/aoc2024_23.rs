use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2024_23 {
    connections: Vec<Connection>,
    network: HashMap<String, Vec<String>>,
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
        let _lines = aoclib::read_lines("test23-1.txt");
        let lines = aoclib::read_lines("input/2024-23.txt");
        self.connections = lines.parse_lines();
        for pair in &self.connections {
            self.network
                .entry(pair.left.clone())
                .or_default()
                .push(pair.right.clone());
            self.network
                .entry(pair.right.clone())
                .or_default()
                .push(pair.left.clone());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut answer = HashSet::new();
        for (computer, connections) in self
            .network
            .iter()
            .filter(|(computer, _)| computer.starts_with("t"))
        {
            for i in 0..connections.len() - 1 {
                for j in i + 1..connections.len() {
                    if self.network[&connections[i]].contains(&connections[j]) {
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
        let mut passwords = HashMap::<String, HashSet<String>>::new();
        for (computer, connections) in &self.network {
            let entry = passwords.entry(computer.clone()).or_default();
            entry.insert(computer.clone());
            entry.extend(connections.iter().cloned());
            for i in 0..connections.len() - 1 {
                for j in i + 1..connections.len() {
                    if !self.network[&connections[i]].contains(&connections[j]) {
                        entry.remove(&connections[i]);
                    }
                }
            }
        }

        let mut password = None;
        let mut max = 0;
        for values in passwords.values() {
            let mut vec = values.iter().cloned().collect::<Vec<_>>();
            if max < vec.len() {
                vec.sort();
                max = vec.len();
                password = Some(vec.join(","));
            }
        }

        aoclib::output(format!("{}", password.unwrap()))
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
