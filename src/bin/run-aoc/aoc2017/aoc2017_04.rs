use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_04 {
    lines: Vec<Vec<String>>,
}

impl Aoc2017_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_04 {
    fn name(&self) -> (usize, usize) {
        (2017, 4)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2017-04.txt") {
            self.lines.push(
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count = 0;
        'next_line: for l in &self.lines {
            let mut hs = HashSet::new();
            for word in l {
                if !hs.insert(word) {
                    continue 'next_line;
                }
            }
            count += 1;
        }
        crate::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut count = 0;
        'next_line: for l in &self.lines {
            let mut hs = HashSet::new();
            for word in l {
                let mut sorted = word.chars().collect::<Vec<char>>();
                sorted.sort();
                if !hs.insert(sorted) {
                    continue 'next_line;
                }
            }
            count += 1;
        }
        crate::output(count)
    }
}
