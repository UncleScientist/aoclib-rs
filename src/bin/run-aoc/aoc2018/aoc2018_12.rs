use std::collections::{HashSet, VecDeque};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_12 {
    initial_state: VecDeque<bool>,
    zero_point: usize,
    notes: HashSet<usize>,
}

impl Aoc2018_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_12 {
    fn name(&self) -> (usize, usize) {
        (2018, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-12.txt");
        // let lines = aoclib::read_lines("test-input");
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
            current_zp += 4;
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

            /*
            let mut iter = next_state.iter();
            while let Some(true) = iter.next() {
                current_zp -= 1;
            }
            current_state.clear();
            current_state.push_front(true);
            current_state.extend(iter);
            */
            current_state = next_state;

            /*
            print!("{current_zp:4}: ");
            for val in current_state.iter() {
                if *val {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
            */
        }

        let val = current_state
            .iter()
            .enumerate()
            .filter(|(_, pot)| **pot)
            .fold(0, |sum, (index, _)| {
                sum + (index as i64 - current_zp as i64)
            });

        crate::output(val)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
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
