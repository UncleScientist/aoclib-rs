use std::collections::HashSet;

use aoclib::Runner;
use pathfinding::prelude::astar;

#[derive(Default)]
pub struct Aoc2024_18 {
    list: Vec<(i64, i64)>,
}

impl Aoc2024_18 {
    pub fn new() -> Self {
        Self::default()
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
    }

    fn part1(&mut self) -> Vec<String> {
        let memory: HashSet<&(i64, i64)> = self.list.iter().take(1024).collect();
        let Some((_path, cost)) = astar(
            &(0i64, 0i64),
            |state| {
                DIRS.iter()
                    .map(|dir| ((state.0 + dir.0, state.1 + dir.1), 1))
                    .filter(|(pos, _)| !memory.contains(&pos))
                    .filter(|(pos, _)| pos.0 >= 0 && pos.1 >= 0 && pos.0 <= 70 && pos.1 <= 70)
                    .collect::<Vec<_>>()
            },
            |state| (70 - state.0) + (70 - state.1),
            |state| *state == (70, 70),
        ) else {
            panic!("no path found");
        };
        aoclib::output(cost)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
