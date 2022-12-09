use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_09 {
    steps: Vec<(Direction, i32)>,
}

impl Aoc2022_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_09 {
    fn name(&self) -> (usize, usize) {
        (2022, 9)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2022-09.txt") {
            let (dir, dist) = line.split_once(' ').unwrap();
            let dir = Direction::parse(dir);
            let dist = dist.parse().unwrap();
            self.steps.push((dir, dist));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.run_snake(2))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.run_snake(10))
    }
}

impl Aoc2022_09 {
    fn run_snake(&self, len: usize) -> usize {
        let mut snake = Snake::new(len);

        for (dir, amount) in &self.steps {
            for _ in 0..*amount {
                snake.make_move(dir);
            }
        }

        snake.visited.len()
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("invalid direction '{s}'"),
        }
    }
}

#[derive(Default)]
struct Snake {
    seg: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Snake {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn new(len: usize) -> Self {
        Self {
            seg: vec![(0, 0); len],
            visited: HashSet::new(),
        }
    }

    fn make_move(&mut self, dir: &Direction) {
        let delta = Self::DIR[*dir as usize];
        self.seg[0].0 += delta.0;
        self.seg[0].1 += delta.1;

        for i in 1..self.seg.len() {
            let rowdiff = self.seg[i - 1].0 - self.seg[i].0;
            let coldiff = self.seg[i - 1].1 - self.seg[i].1;

            if rowdiff.abs() > 1 || coldiff.abs() > 1 {
                self.seg[i].0 += rowdiff.signum();
                self.seg[i].1 += coldiff.signum();
            }
        }

        self.visited.insert(self.seg[self.seg.len() - 1]);
    }
}
