use std::cmp::Ordering;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_24 {
    components: Vec<Component>,
}

impl Aoc2017_24 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_24 {
    fn name(&self) -> (usize, usize) {
        (2017, 24)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2017-24.txt") {
            self.components.push(Component::parse(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(find_strongest_from(0, &self.components))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(find_longest_from(0, &self.components).1)
    }
}

fn find_strongest_from(start: usize, components: &[Component]) -> usize {
    let mut strongest = 0;

    if !components.is_empty() {
        for i in 0..components.len() {
            if let Some(next) = components[i].connects(start) {
                let mut rest = components.to_vec();
                rest.remove(i);
                let strength = components[i].strength() + find_strongest_from(next, &rest);
                strongest = strongest.max(strength);
            }
        }
    }

    strongest
}

fn find_longest_from(start: usize, components: &[Component]) -> (usize, usize) {
    let mut longest = (0, 0); // length, strength

    if !components.is_empty() {
        for i in 0..components.len() {
            if let Some(next) = components[i].connects(start) {
                let mut rest = components.to_vec();
                rest.remove(i);

                let result = find_longest_from(next, &rest);

                let length = (1 + result.0, components[i].strength() + result.1);

                match length.0.cmp(&longest.0) {
                    Ordering::Equal => {
                        if length.1 > longest.1 {
                            longest = length;
                        }
                    }
                    Ordering::Greater => longest = length,
                    Ordering::Less => {}
                }
            }
        }
    }

    longest
}

#[derive(Debug, Clone)]
struct Component(usize, usize);

impl Component {
    fn parse(s: &str) -> Self {
        let (left, right) = s.split_once('/').unwrap();
        Self(left.parse().unwrap(), right.parse().unwrap())
    }

    fn connects(&self, from: usize) -> Option<usize> {
        if self.0 == from {
            Some(self.1)
        } else if self.1 == from {
            Some(self.0)
        } else {
            None
        }
    }

    fn strength(&self) -> usize {
        self.0 + self.1
    }
}
