use aoclib::read_lines;
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

struct Perms<T> {
    list: Vec<Vec<T>>,
}

impl<T: Clone> Perms<T> {
    fn generate(&mut self, vec: &mut [T], size: usize) {
        if size == 1 {
            self.list.push(Vec::from(vec));
        } else {
            self.generate(vec, size - 1);

            for i in 0..size - 1 {
                if size % 2 == 0 {
                    vec.swap(i, size - 1)
                } else {
                    vec.swap(0, size - 1);
                }
                self.generate(vec, size - 1);
            }
        }
    }
}

impl Runner for Aoc2015_09 {
    fn name(&self) -> (usize, usize) {
        (2015, 9)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut permutations = Perms { list: Vec::new() };
        let mut city_list = self
            .cities
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        permutations.generate(&mut city_list, self.cities.len());

        let mut shortest = u64::MAX;
        for p in permutations.list.iter() {
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
