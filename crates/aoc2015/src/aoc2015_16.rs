use std::collections::HashMap;

use aoclib::Runner;

enum Compare {
    LessThan(i32),
    EqualTo(i32),
    GreaterThan(i32),
}

impl Compare {
    fn as_i32(&self) -> i32 {
        match self {
            Compare::LessThan(n) => *n,
            Compare::GreaterThan(n) => *n,
            Compare::EqualTo(n) => *n,
        }
    }
}

pub struct Aoc2015_16 {
    mapping: HashMap<String, Compare>,
    sue: Vec<HashMap<String, i32>>,
}

impl Aoc2015_16 {
    pub fn new() -> Self {
        let mapping = HashMap::from([
            ("children".to_string(), Compare::EqualTo(3)),
            ("cats".to_string(), Compare::GreaterThan(7)),
            ("samoyeds".to_string(), Compare::EqualTo(2)),
            ("pomeranians".to_string(), Compare::LessThan(3)),
            ("akitas".to_string(), Compare::EqualTo(0)),
            ("vizslas".to_string(), Compare::EqualTo(0)),
            ("goldfish".to_string(), Compare::LessThan(5)),
            ("trees".to_string(), Compare::GreaterThan(3)),
            ("cars".to_string(), Compare::EqualTo(2)),
            ("perfumes".to_string(), Compare::EqualTo(1)),
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
                    if val.as_i32() != *n {
                        continue 'outer;
                    }
                }
            }

            found = which + 1;
            break;
        }
        aoclib::output(found)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut found = 0;
        'outer: for (which, sue) in self.sue.iter().enumerate() {
            for (key, val) in &self.mapping {
                if let Some(n) = sue.get(key) {
                    match val {
                        Compare::GreaterThan(val) => {
                            if val >= n {
                                continue 'outer;
                            }
                        }
                        Compare::LessThan(val) => {
                            if val <= n {
                                continue 'outer;
                            }
                        }
                        Compare::EqualTo(val) => {
                            if val != n {
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            found = which + 1;
            break;
        }
        aoclib::output(found)
    }
}
