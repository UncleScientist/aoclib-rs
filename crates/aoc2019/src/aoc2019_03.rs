use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_03 {
    wire1: HashSet<(i64, i64)>,
    wire2: HashSet<(i64, i64)>,
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
        let intersection: Vec<_> = self.wire1.intersection(&self.wire2).collect();
        let min_dist = intersection
            .iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap();
        aoclib::output(min_dist)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.wire2.len())
    }
}

fn parse_line(lines: &str, wire: &mut HashSet<(i64, i64)>) {
    let (mut x, mut y) = (0, 0);

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
            wire.insert((x, y));
        }
    }
}
