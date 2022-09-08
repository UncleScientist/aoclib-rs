use std::collections::{HashMap, HashSet};

use crate::Runner;

pub struct Aoc2015_19 {
    molecule: String,
    combos: HashMap<String, String>,
}

impl Aoc2015_19 {
    pub fn new() -> Self {
        Self {
            molecule: "".to_string(),
            combos: HashMap::new(),
        }
    }
}

impl Runner for Aoc2015_19 {
    fn name(&self) -> (usize, usize) {
        (2015, 19)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2015-19.txt") {
            if let Some((left, right)) = line.split_once(" => ") {
                self.combos.insert(right.to_string(), left.to_string());
            } else {
                self.molecule = line.to_string();
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut hs = HashSet::new();
        for (key, val) in &self.combos {
            for m in self.molecule.match_indices(val) {
                let (left, right) = self.molecule.split_at(m.0);
                let right = right.to_string().split_off(val.len());
                hs.insert(format!("{left}{key}{right}"));
            }
        }
        crate::output(hs.len())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
