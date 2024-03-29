use std::collections::{HashMap, HashSet};

use aoclib::Runner;

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
        aoclib::output(hs.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut molecule = self.molecule.clone();
        let mut count = 0;

        loop {
            let mut done = true;
            for (key, val) in &self.combos {
                if let Some(pos) = molecule.find(key) {
                    let (left, right) = molecule.split_at(pos);
                    let right = right.to_string().split_off(key.len());
                    molecule = format!("{left}{val}{right}");
                    done = false;
                    count += 1;
                }
            }
            if done {
                break;
            }
        }

        aoclib::output(count)
    }
}
