use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_03 {
    wire1: HashMap<(i64, i64), usize>,
    wire2: HashMap<(i64, i64), usize>,
    intersections: Vec<(i64, i64)>,
}

impl Aoc2019_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_03 {
    fn name(&self) -> (usize, usize) {
        (2019, 3)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/day03.txt");
        parse_line(&lines[0], &mut self.wire1);
        parse_line(&lines[1], &mut self.wire2);
    }

    fn part1(&mut self) -> Vec<String> {
        let w1: HashSet<(i64, i64)> = self.wire1.keys().copied().collect();
        let w2: HashSet<(i64, i64)> = self.wire2.keys().copied().collect();
        self.intersections = w1.intersection(&w2).copied().collect();
        let min_dist = self
            .intersections
            .iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap();
        aoclib::output(min_dist)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

fn parse_line(lines: &str, wire: &mut HashMap<(i64, i64), usize>) {
    let (mut x, mut y) = (0, 0);
    let mut steps = 0;

    for inst in lines.split(',') {
        let dir = &inst[0..1];
        let amt = inst[1..].parse::<usize>().expect("can't parse '{inst}'");
        let (dx, dy) = match dir {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("invalid direction {dir}"),
        };

        for _ in 0..amt {
            x += dx;
            y += dy;
            wire.entry((x, y)).or_insert(steps);
            steps += 1;
        }
    }
}
