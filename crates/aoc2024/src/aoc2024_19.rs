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

    fn can_make_design(&self, design: &str, cache: &mut HashMap<String, usize>) -> usize {
        if let Some(value) = cache.get(design) {
            return *value;
        }
        if design.is_empty() {
            return 1;
        }

        let count = self
            .patterns
            .iter()
            .filter_map(|pattern| {
                if design.starts_with(pattern) {
                    Some(self.can_make_design(&design[pattern.len()..], cache))
                } else {
                    None
                }
            })
            .sum();
        cache.insert(design.to_string(), count);
        count
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
        let mut cache = HashMap::new();
        aoclib::output(
            self.designs
                .iter()
                .filter(|design| self.can_make_design(design, &mut cache) > 0)
                .count(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut cache = HashMap::new();
        aoclib::output(
            self.designs
                .iter()
                .map(|design| self.can_make_design(design, &mut cache))
                .sum::<usize>(),
        )
    }
}
