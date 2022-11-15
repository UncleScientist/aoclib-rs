use std::collections::HashMap;

use crate::Runner;

pub struct Aoc2016_13 {
    dist: HashMap<(i32, i32), i32>,
}

struct Maze {
    num: i32,
}

impl Maze {
    fn new(num: i32) -> Self {
        Self { num }
    }

    fn is_wall_at(&self, x: i32, y: i32) -> bool {
        let val = x * x + 3 * x + 2 * x * y + y + y * y + self.num;
        (val.count_ones() & 1) == 1
    }
}

impl Aoc2016_13 {
    pub fn new() -> Self {
        Self {
            dist: HashMap::new(),
        }
    }
}

impl Runner for Aoc2016_13 {
    fn name(&self) -> (usize, usize) {
        (2016, 13)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let maze = Maze::new(1364);

        let mut stack = vec![(0i32, 1i32), (1, 0), (2, 1), (1, 2)];
        self.dist.insert((1, 1), 0);

        while let Some(loc) = stack.pop() {
            if maze.is_wall_at(loc.0, loc.1) {
                continue;
            }

            let mut min = i32::MAX;
            min = min.min(*self.dist.entry((loc.0 + 1, loc.1)).or_insert(i32::MAX));
            min = min.min(*self.dist.entry((loc.0, loc.1 + 1)).or_insert(i32::MAX));
            min = min.min(*self.dist.entry((loc.0 - 1, loc.1)).or_insert(i32::MAX));
            min = min.min(*self.dist.entry((loc.0, loc.1 - 1)).or_insert(i32::MAX));
            min += 1;
            let cur = self.dist.entry(loc).or_insert(i32::MAX);
            if min < *cur {
                *cur = min;

                if loc.0 < 50 {
                    stack.push((loc.0 + 1, loc.1));
                }
                if loc.1 < 50 {
                    stack.push((loc.0, loc.1 + 1));
                }

                if loc.0 > 0 {
                    stack.push((loc.0 - 1, loc.1));
                }
                if loc.1 > 0 {
                    stack.push((loc.0, loc.1 - 1));
                }
            }
        }

        crate::output(self.dist.get(&(31, 39)).unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.dist.values().filter(|&x| *x <= 50).count())
    }
}
