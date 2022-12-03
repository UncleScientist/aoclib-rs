use std::collections::HashSet;

use crate::Runner;

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for char {
    fn priority(&self) -> i32 {
        (match self {
            'a'..='z' => (*self as u8) - b'a' + 1,
            'A'..='Z' => (*self as u8) - b'A' + 27,
            _ => panic!("bad input"),
        }) as i32
    }
}

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
            sum += (dup.iter().next().unwrap()).priority();
        }
        crate::output(sum)
    }

    fn part2(&mut self) -> Vec<String> {
        let all_chars = HashSet::<char>::from_iter(('a'..='z').chain('A'..='Z'));
        let mut sum = 0;
        for lines in self.lines.chunks(3) {
            let dup = lines
                .iter()
                .map(|l| HashSet::from_iter(l.chars()))
                .fold(all_chars.clone(), |a, b| {
                    a.intersection(&b).copied().collect()
                });
            sum += (dup.iter().next().unwrap()).priority();
        }
        crate::output(sum)
    }
}
