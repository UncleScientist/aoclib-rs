use std::collections::VecDeque;

use crate::Runner;

pub struct Aoc2016_14 {
    salt: String,
}

impl Aoc2016_14 {
    pub fn new() -> Self {
        Self {
            salt: "".to_string(),
        }
    }
}

impl Runner for Aoc2016_14 {
    fn name(&self) -> (usize, usize) {
        (2016, 14)
    }

    fn parse(&mut self) {
        self.salt = "ngcjuoqr".to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut list = VecDeque::new();

        for i in 0..1000 {
            list.push_back(format!("{:x}", md5::compute(format!("{}{i}", self.salt))));
        }
        let mut next_hash = 1000;

        let mut count = 0;
        while count < 64 {
            let cur = list.pop_front().unwrap();
            list.push_back(format!(
                "{:x}",
                md5::compute(format!("{}{next_hash}", self.salt))
            ));
            next_hash += 1;

            let mut last_char = '*';
            let mut triple = 0;
            for ch in cur.chars() {
                if ch == last_char {
                    triple += 1;
                    if triple == 3 {
                        break;
                    }
                } else {
                    last_char = ch;
                    triple = 1;
                }
            }

            if triple != 3 {
                continue;
            }

            let mut quintuple = 0;
            'outer: for l in &list {
                quintuple = 0;
                for ch in l.chars() {
                    if ch == last_char {
                        quintuple += 1;
                        if quintuple == 5 {
                            break 'outer;
                        }
                    } else {
                        quintuple = 0;
                    }
                }
            }

            if quintuple == 5 {
                // println!("found at index {}", next_hash - 1001);
                count += 1;
            }
        }

        crate::output(next_hash - 1001)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
