use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_19 {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Aoc2024_19 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_19 {
    fn name(&self) -> (usize, usize) {
        (2024, 19)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-19.txt");
        self.patterns = lines[0]
            .split(", ")
            .map(|pattern| pattern.to_string())
            .collect::<Vec<_>>();
        self.designs = Vec::from(&lines[1..]);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut cache = Cache::default();
        aoclib::output(
            self.designs
                .iter()
                .filter(|design| cache.can_make_design(design, &self.patterns) > 0)
                .count(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut cache = Cache::default();
        aoclib::output(
            self.designs
                .iter()
                .map(|design| cache.can_make_design(design, &self.patterns))
                .sum::<usize>(),
        )
    }
}

#[derive(Debug, Default)]
struct Cache {
    cache: HashMap<String, usize>,
}

impl Cache {
    fn can_make_design(&mut self, design: &str, patterns: &[String]) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(c) = self.cache.get(design) {
            return *c;
        }

        let count = patterns
            .iter()
            .filter(|pattern| design.starts_with(*pattern))
            .map(|pattern| self.can_make_design(&design[pattern.len()..], patterns))
            .sum();
        self.cache.insert(design.to_string(), count);
        count
    }
}
