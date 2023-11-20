use std::{
    collections::{BTreeMap, BTreeSet, HashSet, VecDeque},
    fmt::Display,
};

use aoclib::Point;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_15 {
    map: HashSet<Point<usize>>,
    units: BTreeMap<Point<usize>, Unit>,
    rows: usize,
    cols: usize,
}

impl Aoc2018_15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn _dump(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let p = Point { x: row, y: col };
                if let Some(unit) = self.units.get(&p) {
                    print!("{unit}");
                } else if self.map.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("{:?}", self.units);
    }

    fn surround(&self, point: &Point<usize>) -> Vec<Point<usize>> {
        let look = vec![
            Point {
                x: point.x - 1, // up
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y - 1, // left
            },
            Point {
                x: point.x,
                y: point.y + 1, // right
            },
            Point {
                x: point.x + 1, // down
                y: point.y,
            },
        ];

        let mut result = Vec::new();
        for l in look {
            if self.map.contains(&l) || self.units.contains_key(&l) {
                continue;
            }
            result.push(l);
        }

        result
    }

    fn move_units(&mut self) {
        let unitlist = self.units.clone();
        for unit in &unitlist {
            let mut dest = HashSet::new();
            for enemy in self.units.iter().filter(|u| !u.1.same_type(unit.1)) {
                for p in self.surround(enemy.0) {
                    dest.insert(p);
                }
            }
            let unitmove = self.bfs(unit.0, &dest);
            if let Some(dest) = unitmove {
                let unit = self.units.remove(&unit.0).unwrap();
                self.units.insert(dest, unit);
            }
        }
    }

    // - takes in a starting location and a list of possible destinations
    // - finds the closest destination (in reading order for tie-breaking)
    // - returns the move to make to go to that desination, if any
    fn bfs(&self, start: &Point<usize>, dest: &HashSet<Point<usize>>) -> Option<Point<usize>> {
        let mut steps = BTreeMap::new(); // (neighbor -> visited node)
        let mut queue = VecDeque::new();
        let mut min_depth = i64::MAX;
        let mut candidates = BTreeSet::new();

        queue.push_back((start.clone(), 0));
        steps.insert(start.clone(), None);

        while let Some((next, depth)) = queue.pop_front() {
            if depth > min_depth {
                continue;
            }
            if dest.contains(&next) {
                if min_depth > depth {
                    min_depth = depth;
                    candidates.insert(next.clone());
                } else if min_depth == depth {
                    candidates.insert(next.clone());
                }
            }

            for neighbor in self.surround(&next) {
                if !steps.contains_key(&neighbor) {
                    steps.insert(neighbor.clone(), Some(next.clone()));
                    queue.push_back((neighbor.clone(), depth + 1));
                }
            }
        }
        if let Some(mut best) = candidates.first() {
            while let Some(Some(prev)) = steps.get(&best) {
                if prev == start {
                    return Some(*best);
                }
                best = prev;
            }
            panic!("no path found");
        } else {
            None
        }
    }
}

impl Runner for Aoc2018_15 {
    fn name(&self) -> (usize, usize) {
        (2018, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("test-input");

        self.rows = lines.len();
        self.cols = lines[0].len();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        self.map.insert(Point { x: row, y: col });
                    }
                    'E' => {
                        self.units.insert(Point { x: row, y: col }, Unit::Elf(200));
                    }
                    'G' => {
                        self.units
                            .insert(Point { x: row, y: col }, Unit::Goblin(200));
                    }
                    '.' => {}
                    _ => panic!("invalid character {ch}"),
                }
            }
        }

        self._dump();
    }

    fn part1(&mut self) -> Vec<String> {
        self.move_units();
        self._dump();
        self.move_units();
        self._dump();
        crate::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Unit {
    Elf(i64),
    Goblin(i64),
}

impl Unit {
    fn same_type(&self, other: &Unit) -> bool {
        matches!(
            (self, other),
            (Self::Elf(_), Self::Elf(_)) | (Self::Goblin(_), Self::Goblin(_))
        )
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Elf(_) => "E",
                Self::Goblin(_) => "G",
            }
        )
    }
}
