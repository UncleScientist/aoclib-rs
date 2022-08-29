use aoclib::{read_lines, PermutationIterator};
use std::collections::{HashMap, HashSet};

use crate::Runner;

pub struct Aoc2015_09 {
    dist: HashMap<(String, String), u64>,
    cities: HashSet<String>,
    longest: u64,
}

impl Aoc2015_09 {
    pub fn new() -> Self {
        let mut dist = HashMap::new();
        let mut cities = HashSet::new();

        for l in read_lines("input/2015-09.txt") {
            let line = l.split(' ').collect::<Vec<&str>>();
            let from = line[0];
            let to = line[2];
            let distance = line[4].parse::<u64>().unwrap();

            dist.insert((from.to_string(), to.to_string()), distance);
            dist.insert((to.to_string(), from.to_string()), distance);
            cities.insert(from.to_string());
            cities.insert(to.to_string());
        }

        Self {
            dist,
            cities,
            longest: 0,
        }
    }
}

impl Runner for Aoc2015_09 {
    fn name(&self) -> (usize, usize) {
        (2015, 9)
    }

    fn part1(&mut self) -> Vec<String> {
        let iter = PermutationIterator::new(
            &self
                .cities
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );

        let mut shortest = u64::MAX;
        for p in iter {
            // permutations.list.iter() {
            let mut total = 0;
            for pair in p.windows(2) {
                total += self.dist.get(&(pair[0].clone(), pair[1].clone())).unwrap();
            }

            shortest = shortest.min(total);
            self.longest = self.longest.max(total);
        }

        crate::output(format!("{shortest}"))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.longest)
    }
}
