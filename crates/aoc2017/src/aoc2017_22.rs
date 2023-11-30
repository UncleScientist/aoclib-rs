use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_22 {
    nodes: Vec<((i64, i64), State)>,
    width: i64,
    height: i64,
}

impl Aoc2017_22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_22 {
    fn name(&self) -> (usize, usize) {
        (2017, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-22.txt");

        self.width = lines[0].len() as i64;
        self.height = lines.len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                self.nodes.push((
                    (row as i64, col as i64),
                    if ch == '#' {
                        State::Infected
                    } else {
                        State::Clean
                    },
                ));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = Grid::from_iter(self.nodes.clone(), self.width, self.height);
        for _ in 0..10_000 {
            grid.step(true);
        }
        aoclib::output(grid.infection_count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grid = Grid::from_iter(self.nodes.clone(), self.width, self.height);
        for _ in 0..10000000 {
            grid.step(false);
        }
        aoclib::output(grid.infection_count)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug)]
struct Grid {
    nodes: HashMap<(i64, i64), State>,
    pos: (i64, i64),
    dir: usize,
    infection_count: usize,
}

impl Grid {
    const DIR: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    const RIGHT: usize = 1;
    const REVERSE: usize = 2;
    const LEFT: usize = 3;

    fn _test_new() -> Self {
        let testdata: [((i64, i64), State); 2] =
            [((0, 2), State::Infected), ((1, 0), State::Infected)];
        Self {
            nodes: HashMap::from_iter(testdata),
            dir: 0,
            pos: (1, 1),
            infection_count: 0,
        }
    }

    fn from_iter<I: IntoIterator<Item = ((i64, i64), State)>>(
        iter: I,
        width: i64,
        height: i64,
    ) -> Self {
        Self {
            nodes: HashMap::from_iter(iter),
            pos: (width / 2, height / 2),
            dir: 0,
            infection_count: 0,
        }
    }

    fn step(&mut self, part1: bool) {
        let e = self.nodes.entry(self.pos).or_insert(State::Clean);
        if part1 {
            if *e == State::Infected {
                self.dir = (self.dir + Self::RIGHT) % Self::DIR.len();
                *e = State::Clean;
            } else {
                self.dir = (self.dir + Self::LEFT) % Self::DIR.len();
                *e = State::Infected;
                self.infection_count += 1;
            }
        } else {
            match *e {
                State::Clean => {
                    self.dir = (self.dir + Self::LEFT) % Self::DIR.len();
                    *e = State::Weakened;
                }
                State::Weakened => {
                    self.infection_count += 1;
                    *e = State::Infected;
                }
                State::Infected => {
                    self.dir = (self.dir + Self::RIGHT) % Self::DIR.len();
                    *e = State::Flagged;
                }
                State::Flagged => {
                    self.dir = (self.dir + Self::REVERSE) % Self::DIR.len();
                    *e = State::Clean;
                }
            }
        }

        self.pos.0 += Self::DIR[self.dir].0;
        self.pos.1 += Self::DIR[self.dir].1;
    }
}
