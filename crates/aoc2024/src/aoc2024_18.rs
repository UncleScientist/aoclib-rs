use std::collections::HashMap;

use aoclib::{PointXY, Runner, Size64, DIRS};
use pathfinding::prelude::astar;

#[derive(Default)]
pub struct Aoc2024_18 {
    list: Vec<PointXY>,
    memory: HashMap<PointXY, i64>,
    size: Size64,
}

impl Aoc2024_18 {
    pub fn new() -> Self {
        Self::default()
    }

    fn search(&self, limit: i64) -> Option<(Vec<PointXY>, i64)> {
        astar(
            &PointXY(0i64, 0i64),
            |state| {
                DIRS.iter()
                    .map(|dir| (*state + *dir, 1))
                    .filter(|(pos, _)| *self.memory.get(pos).unwrap_or(&i64::MAX) >= limit)
                    .filter(|(pos, _)| pos.inside(&self.size))
                    .collect::<Vec<_>>()
            },
            |state| (70 - state.0) + (70 - state.1),
            |state| *state == PointXY(70, 70),
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
            .map(|(x, y)| PointXY(x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        self.memory = self
            .list
            .iter()
            .enumerate()
            .map(|(idx, pos)| (*pos, idx as i64))
            .collect();
        self.size = Size64(71, 71);
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
        aoclib::output(self.list[max - 1])
    }
}
