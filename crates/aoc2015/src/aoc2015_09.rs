use aoclib::{read_lines, Permutations};
use std::collections::{HashMap, HashSet};

use aoclib::Runner;

pub struct Aoc2015_09 {
    dist: HashMap<(String, String), u64>,
    cities: HashSet<String>,
    longest: u64,
}

impl Aoc2015_09 {
    pub fn new() -> Self {
        Self {
            dist: HashMap::new(),
            cities: HashSet::new(),
            longest: 0,
        }
    }
}

impl Runner for Aoc2015_09 {
    fn parse(&mut self) {
        for l in read_lines("input/2015-09.txt") {
            let line = l.split(' ').collect::<Vec<&str>>();
            let from = line[0];
            let to = line[2];
            let distance = line[4].parse::<u64>().unwrap();

            self.dist
                .insert((from.to_string(), to.to_string()), distance);
            self.dist
                .insert((to.to_string(), from.to_string()), distance);
            self.cities.insert(from.to_string());
            self.cities.insert(to.to_string());
        }
    }
    fn name(&self) -> (usize, usize) {
        (2015, 9)
    }

    fn part1(&mut self) -> Vec<String> {
        let city_list = self
            .cities
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        /*
        let iter = PermutationIterator::new(
            &self
                .cities
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );

        for p in iter {
            */

        let mut shortest = u64::MAX;
        for city in city_list.permutations() {
            let mut total = 0;
            for pair in city.windows(2) {
                total += self.dist.get(&(pair[0].clone(), pair[1].clone())).unwrap();
            }

            shortest = shortest.min(total);
            self.longest = self.longest.max(total);
        }

        aoclib::output(format!("{shortest}"))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.longest)
    }
}
