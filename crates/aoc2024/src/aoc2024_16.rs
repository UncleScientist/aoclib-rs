use std::collections::HashSet;

use aoclib::Runner;
use pathfinding::prelude::dijkstra;

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
}

impl Runner for Aoc2024_16 {
    fn name(&self) -> (usize, usize) {
        (2024, 16)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-16.txt");

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
            |state| {
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
            },
            |state| state.pos == self.end,
        ) else {
            panic!("no path found");
        };
        aoclib::output(cost)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pos: (i64, i64),
    dir: (i64, i64),
}
