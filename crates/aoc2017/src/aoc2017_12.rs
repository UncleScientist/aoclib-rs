use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_12 {
    progmap: HashMap<usize, Vec<usize>>,
}

impl Aoc2017_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_12 {
    fn name(&self) -> (usize, usize) {
        (2017, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-12.txt");

        for line in lines {
            let (prog, list) = line.split_once(" <-> ").unwrap();
            let prog = prog.parse::<usize>().unwrap();
            let mut proglist = Vec::new();

            for num in list.split(", ") {
                proglist.push(num.parse().unwrap());
            }
            self.progmap.insert(prog, proglist);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut stack = vec![0usize];
        let mut visited = HashSet::new();

        while let Some(next) = stack.pop() {
            if visited.insert(next) {
                let to_visit = self.progmap.get(&next).unwrap();
                stack.extend(to_visit.iter().filter(|n| !visited.contains(n)));
            }
        }
        aoclib::output(visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut unvisited = HashSet::new();
        for prog in self.progmap.keys() {
            unvisited.insert(prog);
        }

        let mut group_count = 0;
        while !unvisited.is_empty() {
            group_count += 1;

            let start = **unvisited.iter().next().unwrap();
            unvisited.remove(&start);

            let mut stack = vec![start];
            let mut visited = HashSet::new();

            while let Some(next) = stack.pop() {
                unvisited.remove(&next);
                if visited.insert(next) {
                    let to_visit = self.progmap.get(&next).unwrap();
                    stack.extend(to_visit.iter().filter(|n| !visited.contains(n)));
                }
            }
        }

        aoclib::output(group_count)
    }
}
