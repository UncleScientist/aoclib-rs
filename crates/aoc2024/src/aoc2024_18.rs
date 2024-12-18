use std::collections::HashMap;

use aoclib::Runner;
use pathfinding::prelude::astar;

#[derive(Default)]
pub struct Aoc2024_18 {
    list: Vec<(i64, i64)>,
    memory: HashMap<(i64, i64), i64>,
}

impl Aoc2024_18 {
    pub fn new() -> Self {
        Self::default()
    }

    fn search(&self, limit: i64) -> Option<(Vec<(i64, i64)>, i64)> {
        astar(
            &(0i64, 0i64),
            |state| {
                DIRS.iter()
                    .map(|dir| ((state.0 + dir.0, state.1 + dir.1), 1))
                    .filter(|(pos, _)| *self.memory.get(&pos).unwrap_or(&i64::MAX) >= limit)
                    .filter(|(pos, _)| pos.0 >= 0 && pos.1 >= 0 && pos.0 <= 70 && pos.1 <= 70)
                    .collect::<Vec<_>>()
            },
            |state| (70 - state.0) + (70 - state.1),
            |state| *state == (70, 70),
        )
    }
}

impl Runner for Aoc2024_18 {
    fn name(&self) -> (usize, usize) {
        (2024, 18)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-18.txt");
        self.list = lines
            .iter()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        self.memory = self
            .list
            .iter()
            .enumerate()
            .map(|(idx, pos)| (*pos, idx as i64))
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.search(1024).unwrap().1)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut min = 0;
        let mut max = self.list.len();
        while max > min {
            let mid = (max + min) / 2;
            if self.search(mid as i64).is_none() {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        aoclib::output(format!("{},{}", self.list[max - 1].0, self.list[max - 1].1))
    }
}

const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
