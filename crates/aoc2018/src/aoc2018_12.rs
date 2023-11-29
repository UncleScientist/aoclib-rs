use std::collections::{HashMap, HashSet, VecDeque};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_12 {
    initial_state: VecDeque<bool>,
    zero_point: i64,
    notes: HashSet<usize>,
}

impl Aoc2018_12 {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&self, mut current_zp: i64, state: &VecDeque<bool>) -> (i64, VecDeque<bool>) {
        current_zp += 4;
        let mut current_state = state.clone();

        current_state.push_front(false);
        current_state.push_front(false);
        current_state.push_front(false);
        current_state.push_front(false);
        current_state.push_back(false);
        current_state.push_back(false);
        current_state.push_back(false);
        current_state.push_back(false);

        let mut next_state = VecDeque::new();
        next_state.push_front(false);
        next_state.push_front(false);
        for group in current_state.make_contiguous().windows(5) {
            next_state.push_back(self.notes.contains(&group.convert()));
        }

        while Some(&false) == next_state.front() {
            next_state.pop_front();
            current_zp -= 1;
        }

        while Some(&false) == next_state.back() {
            next_state.pop_back();
        }

        (current_zp, next_state)
    }
}

impl Runner for Aoc2018_12 {
    fn name(&self) -> (usize, usize) {
        (2018, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-12.txt");
        let (_, istate) = lines[0].split_once(": ").unwrap();

        self.initial_state = istate
            .chars()
            .map(|ch| match ch {
                '.' => false,
                '#' => true,
                _ => panic!("invalid char {ch}"),
            })
            .collect::<VecDeque<_>>();
        self.zero_point = 0;

        for line in lines.iter().skip(1) {
            let (left, right) = line.split_once(" => ").unwrap();
            if right == "#" {
                self.notes.insert(left.convert());
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut current_state = self.initial_state.clone();
        let mut current_zp = self.zero_point;

        for _ in 0..20 {
            (current_zp, current_state) = self.step(current_zp, &current_state);
        }

        aoclib::output(current_state.calc_sum(current_zp))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut states = HashMap::new();
        let mut count = 0;
        let mut current_state = self.initial_state.clone();
        let mut current_zp = self.zero_point;

        // TODO: cleanup: don't assume the state moves 1 by 1 after looping
        loop {
            count += 1;
            (current_zp, current_state) = self.step(current_zp, &current_state);
            if states.insert(current_state.clone(), current_zp).is_some() {
                break;
            }
        }

        let remaining_generations = 50_000_000_000i64 - count;
        aoclib::output(current_state.calc_sum(current_zp - remaining_generations))
    }
}

trait Conversion {
    fn convert(&self) -> usize;
}

impl Conversion for &str {
    fn convert(&self) -> usize {
        let mut result = 0;
        for ch in self.chars() {
            result <<= 1;
            result += (ch == '#') as usize;
        }
        result
    }
}

impl Conversion for &[bool] {
    fn convert(&self) -> usize {
        let mut result = 0;
        for bool in self.iter() {
            result <<= 1;
            result += *bool as usize;
        }
        result
    }
}

trait Summer {
    fn calc_sum(&self, zp: i64) -> i64;
}

impl Summer for VecDeque<bool> {
    fn calc_sum(&self, zp: i64) -> i64 {
        self.iter()
            .enumerate()
            .filter(|(_, pot)| **pot)
            .fold(0, |sum, (index, _)| sum + (index as i64 - zp))
    }
}
