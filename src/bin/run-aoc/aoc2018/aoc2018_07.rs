use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_07 {
    graph: BTreeMap<char, Node>,
    possible: BTreeSet<char>,
}

impl Aoc2018_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_07 {
    fn name(&self) -> (usize, usize) {
        (2018, 7)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-07.txt");

        for l in lines {
            let words = l.split_whitespace().collect::<Vec<_>>();
            let node = words[1].chars().next().unwrap();
            let prereq = words[7].chars().next().unwrap();
            self.possible.insert(node);
            self.possible.insert(prereq);

            let e = self.graph.entry(node).or_default();
            e.prereq.insert(prereq);

            let e = self.graph.entry(prereq).or_default();
            e.count += 1;
        }
        for node in self.graph.values() {
            for ch in node.prereq.iter() {
                self.possible.remove(ch);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut completed = String::new();
        let mut possible = self.possible.clone();
        let mut graph = self.graph.clone();

        while let Some(item) = possible.pop_first() {
            completed.push(item);
            let prereq = graph.get(&item).unwrap().clone();
            for finished in prereq.prereq {
                let node = graph.get_mut(&finished).unwrap();
                node.count -= 1;
                if node.count == 0 {
                    possible.insert(finished);
                }
            }
        }

        crate::output(completed)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut queue = BTreeMap::new();
        let possible = self.possible.clone();
        let mut graph = self.graph.clone();

        for p in possible {
            queue.insert(60 + (p as i64) - (b'A' as i64) + 1, p);
        }

        // how can this be giving the right answer? We're not taking into
        // account the number of simultaneous workers
        // maybe the worker count doesn't matter?
        let mut current_time = 0;
        while let Some((time, item)) = queue.pop_first() {
            current_time = time;
            let prereq = graph.get(&item).unwrap().clone();
            for finished in prereq.prereq {
                let node = graph.get_mut(&finished).unwrap();
                node.count -= 1;
                if node.count == 0 {
                    queue.insert(
                        current_time + 60 + (finished as i64) - (b'A' as i64) + 1,
                        finished,
                    );
                }
            }
        }

        crate::output(current_time)
    }
}

#[derive(Default, Debug, Clone)]
struct Node {
    prereq: HashSet<char>,
    count: i64,
}
