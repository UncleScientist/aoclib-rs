use crate::Runner;
use aoclib::Combinations;

pub struct Aoc2015_17 {
    cap: Vec<i32>,
}

impl Aoc2015_17 {
    pub fn new() -> Self {
        Self { cap: Vec::new() }
    }
}

impl Runner for Aoc2015_17 {
    fn name(&self) -> (usize, usize) {
        (2015, 17)
    }

    fn parse(&mut self) {
        self.cap = aoclib::read_lines("input/2015-17.txt")
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count = 0;
        for c in self.cap.combinations() {
            if c.iter().sum::<i32>() == 150 {
                count += 1;
            }
        }
        crate::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut count = vec![0; self.cap.len()];
        for c in self.cap.combinations() {
            if c.iter().sum::<i32>() == 150 {
                count[c.len()] += 1;
            }
        }
        crate::output(count.iter().filter(|&x| *x != 0).min().unwrap())
    }
}
