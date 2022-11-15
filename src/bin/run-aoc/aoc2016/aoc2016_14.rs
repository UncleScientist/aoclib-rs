use std::collections::VecDeque;

use crate::Runner;

pub struct Aoc2016_14;

impl Aoc2016_14 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2016_14 {
    fn name(&self) -> (usize, usize) {
        (2016, 14)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        crate::output(find_64th_key("ngcjuoqr".to_string(), 0))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(find_64th_key("ngcjuoqr".to_string(), 2016))
    }
}

fn stretch(unhashed: &String, rounds: usize) -> String {
    let mut hash = format!("{:x}", md5::compute(unhashed));
    for _ in 0..rounds {
        hash = format!("{:x}", md5::compute(hash));
    }

    hash
}

fn find_64th_key(salt: String, stretch_count: usize) -> usize {
    let mut list = VecDeque::new();

    for i in 0..1000 {
        list.push_back(stretch(&format!("{}{i}", salt), stretch_count));
    }
    let mut next_hash = 1000;

    let mut count = 0;
    while count < 64 {
        let cur = list.pop_front().unwrap();
        list.push_back(stretch(&format!("{}{next_hash}", salt), stretch_count));
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

    next_hash - 1001
}
