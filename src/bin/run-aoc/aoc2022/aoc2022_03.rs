use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_03 {
    lines: Vec<String>,
}

impl Aoc2022_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_03 {
    fn name(&self) -> (usize, usize) {
        (2022, 3)
    }

    fn parse(&mut self) {
        self.lines = aoclib::read_lines("input/2022-03.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut sum = 0;
        for line in &self.lines {
            let len = line.len() / 2;
            let (left, right) = (&line[0..len], &line[len..]);
            let left: HashSet<char> = HashSet::from_iter(left.chars());
            let right: HashSet<char> = HashSet::from_iter(right.chars());
            let dup: HashSet<&char> = left.intersection(&right).collect();
            let ch = **dup.iter().next().unwrap();
            sum += match ch {
                'a'..='z' => (ch as u8) - b'a' + 1,
                'A'..='Z' => (ch as u8) - b'A' + 27,
                _ => panic!("bad input"),
            } as i32;
        }
        crate::output(sum)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut sum = 0;
        for lines in self.lines.chunks(3) {
            let mut sets = lines
                .iter()
                .map(|l| HashSet::from_iter(l.chars()))
                .collect::<Vec<HashSet<char>>>();
            let mut dup = sets.pop().unwrap();
            for set in sets {
                dup = set.intersection(&dup).copied().collect();
            }
            let ch = *dup.iter().next().unwrap();
            sum += match ch {
                'a'..='z' => (ch as u8) - b'a' + 1,
                'A'..='Z' => (ch as u8) - b'A' + 27,
                _ => panic!("bad input"),
            } as i32;
        }
        crate::output(sum)
    }
}
