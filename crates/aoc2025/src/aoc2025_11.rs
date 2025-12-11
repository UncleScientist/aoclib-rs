use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_11 {
    reactor: HashMap<String, Vec<String>>,
}

impl Aoc2025_11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_11 {
    fn name(&self) -> (usize, usize) {
        (2025, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-11.txt");
        for line in lines {
            let (reactor, nodes) = line.split_once(": ").unwrap();
            let nodes = nodes.split(' ').map(|n| n.to_string()).collect();
            self.reactor.insert(reactor.into(), nodes);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut cache = cache::CacheFinder::new();
        aoclib::output(cache.find(
            &"you".to_string(),
            &|state| self.reactor.get(state).cloned(),
            &|state| {
                if state.as_str() == "out" {
                    Some(1)
                } else {
                    None
                }
            },
        ))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut cache = cache::CacheFinder::new();
        aoclib::output(cache.find(
            &SearchState::new("svr"),
            &|state| self.part2_neighbors(state),
            &|state| {
                if self.part2_done(state) {
                    Some(1)
                } else {
                    None
                }
            },
        ))
    }
}

impl Aoc2025_11 {
    fn part2_neighbors(&self, state: &SearchState) -> Option<Vec<SearchState>> {
        if let Some(neighbors) = self.reactor.get(&state.nodename) {
            Some(
                neighbors
                    .iter()
                    .map(|node| {
                        let dac = state.dac || node == "dac";
                        let fft = state.fft || node == "fft";
                        SearchState {
                            nodename: node.clone(),
                            dac,
                            fft,
                        }
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    fn part2_done(&self, state: &SearchState) -> bool {
        state.nodename.as_str() == "out" && state.dac && state.fft
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchState {
    nodename: String,
    dac: bool,
    fft: bool,
}

impl SearchState {
    fn new(start: &str) -> Self {
        Self {
            nodename: start.to_string(),
            dac: false,
            fft: false,
        }
    }
}

mod cache {
    use std::{collections::HashMap, hash::Hash};

    pub struct CacheFinder<T> {
        cache: HashMap<T, usize>,
    }

    impl<T: Hash + PartialEq + Eq + Clone> CacheFinder<T> {
        pub fn new() -> Self {
            Self {
                cache: HashMap::new(),
            }
        }

        pub fn find(
            &mut self,
            start: &T,
            neighbors: &impl Fn(&T) -> Option<Vec<T>>,
            end_check: &impl Fn(&T) -> Option<usize>,
        ) -> usize {
            if let Some(result) = end_check(start) {
                return result;
            }

            if let Some(result) = self.cache.get(start) {
                return *result;
            }

            let count = if let Some(nodelist) = neighbors(start) {
                nodelist
                    .iter()
                    .map(|node| self.find(node, neighbors, end_check))
                    .sum()
            } else {
                0
            };
            self.cache.insert(start.clone(), count);
            count
        }
    }
}
