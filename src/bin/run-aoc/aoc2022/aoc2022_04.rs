use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_04 {
    assignment: Vec<Assignment>,
}

impl Aoc2022_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_04 {
    fn name(&self) -> (usize, usize) {
        (2022, 4)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2022-04.txt") {
            self.assignment.push(Assignment::parse(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.assignment.iter().filter(Assignment::redundant).count())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.assignment.iter().filter(Assignment::overlaps).count())
    }
}

struct Assignment {
    left: (i32, i32),
    right: (i32, i32),
}

impl Assignment {
    fn parse(s: &str) -> Self {
        let (a, b) = s.split_once(',').unwrap();

        let pair = a.split_once('-').unwrap();
        let first = (pair.0.parse().unwrap(), pair.1.parse().unwrap());

        let pair = b.split_once('-').unwrap();
        let second = (pair.0.parse().unwrap(), pair.1.parse().unwrap());

        let (left, right) = if first.0 < second.0 {
            (first, second)
        } else {
            (second, first)
        };

        Self { left, right }
    }

    fn redundant(a: &&Assignment) -> bool {
        (a.left.0 <= a.right.0 && a.left.1 >= a.right.1)
            || (a.right.0 <= a.left.0 && a.right.1 >= a.left.1)
    }

    fn overlaps(a: &&Assignment) -> bool {
        a.left.1 >= a.right.0
    }
}
