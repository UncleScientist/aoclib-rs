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
        let mut snake = Snake::default();

        for (dir, amount) in &self.steps {
            for _ in 0..*amount {
                snake.make_move(dir);
            }
        }

        crate::output(snake.visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
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
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Snake {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn make_move(&mut self, dir: &Direction) {
        let delta = Self::DIR[*dir as usize];
        self.head.0 += delta.0;
        self.head.1 += delta.1;

        let rowdiff = self.head.0 - self.tail.0;
        let coldiff = self.head.1 - self.tail.1;

        if rowdiff == 0 && coldiff.abs() > 1 {
            self.tail.1 += coldiff.signum();
        } else if coldiff == 0 && rowdiff.abs() > 1 {
            self.tail.0 += rowdiff.signum();
        } else if rowdiff.abs() > 1 || coldiff.abs() > 1 {
            self.tail.0 += rowdiff.signum();
            self.tail.1 += coldiff.signum();
        }

        self.visited.insert(self.tail);

        /*
        for row in -5..5 {
            for col in -5..5 {
                if (row, col) == self.head {
                    print!("H");
                } else if (row, col) == self.tail {
                    print!("T");
                } else if self.visited.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!("-----");
        */
    }
}
