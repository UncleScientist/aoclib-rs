use std::collections::HashMap;

use crate::Runner;

pub struct Aoc2015_16 {
    mapping: HashMap<String, i32>,
    sue: Vec<HashMap<String, i32>>,
}

impl Aoc2015_16 {
    pub fn new() -> Self {
        let mapping = HashMap::from([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]);
        Self {
            mapping,
            sue: Vec::new(),
        }
    }
}

// Sue 1: goldfish: 9, cars: 0, samoyeds: 9

impl Runner for Aoc2015_16 {
    fn name(&self) -> (usize, usize) {
        (2015, 16)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2015-16.txt") {
            let (_, data) = line.split_once(": ").unwrap();
            let items = data.split(", ").collect::<Vec<&str>>();

            let mut hm = HashMap::new();
            for i in items {
                let (name, count) = i.split_once(": ").unwrap();
                hm.insert(name.to_string(), count.parse::<i32>().unwrap());
            }
            self.sue.push(hm);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut found = 0;
        'outer: for (which, sue) in self.sue.iter().enumerate() {
            for (key, val) in &self.mapping {
                if let Some(n) = sue.get(key) {
                    if *val != *n {
                        continue 'outer;
                    }
                }
            }

            found = which + 1;
            break;
        }
        crate::output(found)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
