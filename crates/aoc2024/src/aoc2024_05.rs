use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_05 {
    before: HashMap<i64, HashSet<i64>>,
    updates: Vec<Vec<i64>>,
}

impl Aoc2024_05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn convert(&mut self, lines: &[String]) {
        for line in lines {
            if let Some((left, right)) = line.split_once('|') {
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                let entry = self.before.entry(right).or_default();
                entry.insert(left);
            } else {
                let update = line
                    .split(',')
                    .map(|page| page.parse::<i64>().unwrap())
                    .collect();
                self.updates.push(update);
            }
        }
    }

    fn find_bad(&self, update: &[i64]) -> Option<(usize, usize)> {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                // update[i] is currently before update[j] -- should it be?
                if let Some(hs) = self.before.get(&update[i]) {
                    if hs.contains(&update[j]) {
                        return Some((i, j));
                    }
                }
            }
        }

        None
    }
}

impl Runner for Aoc2024_05 {
    fn name(&self) -> (usize, usize) {
        (2024, 5)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-05.txt");
        self.convert(&lines);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        for update in &self.updates {
            if self.find_bad(update).is_none() {
                total += update[update.len() / 2];
            }
        }
        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for original in &self.updates {
            let mut update = original.clone();
            while let Some((left, right)) = self.find_bad(&update) {
                update.swap(left, right);
            }
            if update != *original {
                total += update[update.len() / 2];
            }
        }
        aoclib::output(total)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn lines() -> [String; 27] {
        [
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ]
    }

    #[test]
    fn test_part_1() {
        let mut aoc = Aoc2024_05::default();
        aoc.convert(&lines());
        assert_eq!(vec!["143".to_string()], aoc.part1());
    }

    #[test]
    fn test_part_2() {
        let mut aoc = Aoc2024_05::default();
        aoc.convert(&lines());
        assert_eq!(vec!["123".to_string()], aoc.part2());
    }
}
