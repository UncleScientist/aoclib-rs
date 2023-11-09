use std::slice::Iter;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_08 {
    tree: Vec<u64>,
}

impl Aoc2018_08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_08 {
    fn name(&self) -> (usize, usize) {
        (2018, 8)
    }

    fn parse(&mut self) {
        self.tree = aoclib::numbers::<&str, u64>("input/2018-08.txt", ' ')
            .first()
            .unwrap()
            .clone();
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(sum_metadata(&mut self.tree.iter()))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

fn sum_metadata(iter: &mut Iter<u64>) -> u64 {
    let child_count = *iter.next().unwrap();
    let metadata_count = *iter.next().unwrap();

    let mut result = 0;
    for _ in 0..child_count {
        result += sum_metadata(iter);
    }

    for _ in 0..metadata_count {
        result += *iter.next().unwrap();
    }

    result
}
