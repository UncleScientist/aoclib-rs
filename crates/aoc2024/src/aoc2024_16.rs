use std::collections::HashSet;

use aoclib::Runner;
use pathfinding::prelude::{astar_bag_collect, dijkstra};

#[derive(Default)]
pub struct Aoc2024_16 {
    maze: HashSet<(i64, i64)>,
    rows: i64,
    cols: i64,
    start: (i64, i64),
    end: (i64, i64),
}

impl Aoc2024_16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn succssors(&self, state: &State) -> Vec<(State, usize)> {
        let mut result = Vec::<(State, usize)>::new();
        let next = (state.pos.0 + state.dir.0, state.pos.1 + state.dir.1);
        if !self.maze.contains(&next) {
            result.push((
                State {
                    pos: next,
                    dir: state.dir,
                },
                1,
            ));
        }

        let left_turn = (-state.dir.1, state.dir.0);
        let next = (state.pos.0 + left_turn.0, state.pos.1 + left_turn.1);
        if !self.maze.contains(&next) {
            result.push((
                State {
                    pos: next,
                    dir: left_turn,
                },
                1001,
            ));
        }

        let right_turn = (state.dir.1, -state.dir.0);
        let next = (state.pos.0 + right_turn.0, state.pos.1 + right_turn.1);
        if !self.maze.contains(&next) {
            result.push((
                State {
                    pos: next,
                    dir: right_turn,
                },
                1001,
            ));
        }

        result
    }
}

impl Runner for Aoc2024_16 {
    fn name(&self) -> (usize, usize) {
        (2024, 16)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-16.txt");
        let _lines = aoclib::read_lines("test16-1.txt");

        self.rows = lines.len() as i64;
        self.cols = lines[0].len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = (row as i64, col as i64);
                if ch == 'S' {
                    self.start = pos;
                } else if ch == 'E' {
                    self.end = pos;
                } else if ch == '#' {
                    self.maze.insert(pos);
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let start_state = State {
            pos: self.start,
            dir: (0, 1),
        };
        let Some((_, cost)) = dijkstra(
            &start_state,
            |state| self.succssors(state),
            |state| state.pos == self.end,
        ) else {
            panic!("no path found");
        };
        aoclib::output(cost)
    }

    fn part2(&mut self) -> Vec<String> {
        let start_state = State {
            pos: self.start,
            dir: (0, 1),
        };
        let Some((result, _)) = astar_bag_collect(
            &start_state,
            |state| self.succssors(state),
            |state| (state.pos.0.abs_diff(self.end.0) + state.pos.1.abs_diff(self.end.1)) as usize,
            |state| state.pos == self.end,
        ) else {
            panic!("no path found");
        };

        let nodes = result
            .iter()
            .flat_map(|x| x.iter().map(|s| s.pos))
            .collect::<HashSet<_>>();

        /*
        for row in 0..self.rows {
            for col in 0..self.cols {
                if nodes.contains(&(row, col)) {
                    print!("O");
                } else if self.maze.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        */

        aoclib::output(nodes.len())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pos: (i64, i64),
    dir: (i64, i64),
}
