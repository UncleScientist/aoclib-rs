use std::{collections::HashMap, fmt::Display};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2018_17 {
    grid: HashMap<(i64, i64), Scan>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    highlight: Option<(i64, i64)>,
}

impl Aoc2018_17 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get(&self, row: i64, col: i64) -> Scan {
        *self.grid.get(&(row, col)).unwrap_or_default()
    }

    fn put(&mut self, row: i64, col: i64, item: Scan) {
        if row <= self.max_y {
            self.grid.insert((row, col), item);
        }
    }

    fn look(
        &self,
        pos: (i64, i64),
        dir: i64,
    ) -> (bool, i64, Option<(i64, i64)>, Option<(i64, i64)>) {
        let mut push = None;
        let mut count = dir;
        let mut edge = false;
        let mut failure = None;

        loop {
            let look = self.get(pos.0, pos.1 + count);
            let below = self.get(pos.0 + 1, pos.1 + count);
            match (look, below) {
                (Scan::Flowing, Scan::Clay)
                | (Scan::Flowing, Scan::Still)
                | (Scan::Sand, Scan::Clay)
                | (Scan::Sand, Scan::Still) => {
                    count += dir;
                }
                (Scan::Flowing, Scan::Sand) => {
                    break;
                }
                (Scan::Sand, Scan::Sand) => {
                    push = Some((pos.0, pos.1 + count));
                    break;
                }
                (Scan::Clay, _) => {
                    edge = true;
                    break;
                }
                (top, bottom) => {
                    println!("Need to handle {top:?}/{bottom:?} in {dir} search");
                    failure = Some(pos);
                }
            }
        }

        (edge, count, push, failure)
    }

    fn _dump(&self) {
        println!("highlight: {:?}", self.highlight);
        for row in self.min_y..=self.max_y {
            for col in self.min_x..=self.max_x {
                if self.highlight == Some((row, col)) {
                    print!("X");
                } else {
                    print!("{}", self.get(row, col));
                }
            }
            println!();
        }
    }
}

impl Runner for Aoc2018_17 {
    fn name(&self) -> (usize, usize) {
        (2018, 17)
    }

    fn parse(&mut self) {
        self.min_x = i64::MAX;
        self.max_x = i64::MIN;
        self.min_y = i64::MAX;
        self.max_y = i64::MIN;

        let lines = aoclib::read_lines("input/2018-17.txt");
        for l in lines {
            let (left, right) = l.split_once(", ").unwrap();
            let (_, base) = left.split_once('=').unwrap();
            let base = base.parse().unwrap();

            let (_, range) = right.split_once('=').unwrap();
            let (min, max) = range.split_once("..").unwrap();
            let (min, max) = (min.parse().unwrap(), max.parse().unwrap());
            if left.starts_with("x") {
                self.min_x = self.min_x.min(base);
                self.max_x = self.max_x.max(base);
                self.min_y = self.min_y.min(min);
                self.max_y = self.max_y.max(max);
                for y in min..=max {
                    self.grid.insert((y, base), Scan::Clay);
                }
            } else {
                self.min_x = self.min_x.min(min);
                self.max_x = self.max_x.max(max);
                self.min_y = self.min_y.min(base);
                self.max_y = self.max_y.max(base);
                for x in min..=max {
                    self.grid.insert((base, x), Scan::Clay);
                }
            }
        }

        // self._dump();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut queue = vec![(0, 500)];

        'panic: while let Some(pos) = queue.pop() {
            if pos.0 > self.max_y {
                continue;
            }
            let cur = self.get(pos.0, pos.1);
            let below = self.get(pos.0 + 1, pos.1);
            match (cur, below) {
                (Scan::Sand, Scan::Sand) | (Scan::Flowing, Scan::Sand) => {
                    queue.push((pos.0 + 1, pos.1));
                    if pos.0 + 1 < self.min_y {
                        continue;
                    }
                    self.put(pos.0 + 1, pos.1, Scan::Flowing);
                }
                (Scan::Flowing, Scan::Clay) | (Scan::Flowing, Scan::Still) => {
                    queue.push((pos.0 - 1, pos.1));

                    let (left_edge, left_count, left_push, left_fail) = self.look(pos, -1);
                    if left_fail.is_some() {
                        self.highlight = left_fail;
                        break 'panic;
                    }
                    if let Some(lp) = left_push {
                        queue.push(lp);
                    }

                    let (right_edge, right_count, right_push, right_fail) = self.look(pos, 1);
                    if right_fail.is_some() {
                        self.highlight = right_fail;
                        break 'panic;
                    }
                    if let Some(rp) = right_push {
                        queue.push(rp);
                    }

                    match (left_edge, right_edge) {
                        (true, true) => {
                            for x in pos.1 + left_count + 1..pos.1 + right_count {
                                self.put(pos.0, x, Scan::Still);
                            }
                        }
                        (true, false) => {
                            for x in pos.1 + left_count + 1..=pos.1 + right_count {
                                self.put(pos.0, x, Scan::Flowing);
                            }
                        }
                        (false, true) => {
                            for x in pos.1 + left_count..pos.1 + right_count {
                                self.put(pos.0, x, Scan::Flowing);
                            }
                        }
                        (false, false) => {
                            for x in pos.1 + left_count..=pos.1 + right_count {
                                self.put(pos.0, x, Scan::Flowing);
                            }
                        }
                    }
                }
                (Scan::Flowing, Scan::Flowing)
                | (Scan::Sand, Scan::Still)
                | (Scan::Sand, Scan::Flowing) => {}

                (top, bottom) => {
                    self.highlight = Some(pos);
                    println!("Need to handle {top:?}/{bottom:?} in water flow search");
                    break 'panic;
                }
            }
        }

        /*
        println!("Work queue: {queue:?}");
        self._dump();
        */

        let water_count = self
            .grid
            .values()
            .filter(|v| **v == Scan::Flowing || **v == Scan::Still)
            .count();
        aoclib::output(water_count)
    }

    fn part2(&mut self) -> Vec<String> {
        let still_count = self.grid.values().filter(|v| **v == Scan::Still).count();
        aoclib::output(still_count)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Scan {
    Sand,
    Clay,
    Flowing,
    Still,
}

impl Default for &Scan {
    fn default() -> Self {
        &Scan::Sand
    }
}

impl Display for Scan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Scan::Sand => '.',
                Scan::Clay => '#',
                Scan::Flowing => '|',
                Scan::Still => '~',
            }
        )
    }
}
