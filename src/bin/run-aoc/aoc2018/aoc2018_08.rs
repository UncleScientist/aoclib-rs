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
        // self.tree = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(sum_metadata(&mut self.tree.iter()))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(value_of(&mut self.tree.iter()))
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

fn value_of(iter: &mut Iter<u64>) -> u64 {
    let child_count = *iter.next().unwrap();
    let metadata_count = *iter.next().unwrap();

    if child_count == 0 {
        return iter.take(metadata_count as usize).sum();
    }

    let mut child_values = Vec::new();
    for _ in 0..child_count {
        child_values.push(value_of(iter));
    }

    let mut value = 0;
    for _ in 0..metadata_count {
        let index = *iter.next().unwrap() as usize;
        if index > 0 && index <= child_values.len() {
            value += child_values[index - 1];
        }
    }

    value
}
