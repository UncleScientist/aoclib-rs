use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_06 {
    orbit: HashMap<String, String>,
}

impl Aoc2019_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_06 {
    fn name(&self) -> (usize, usize) {
        (2019, 6)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2019-06.txt");
        self.init(&lines);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.count_orbits())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.orbital_transfers().unwrap())
    }
}

impl Aoc2019_06 {
    fn init<T: AsRef<str>>(&mut self, lines: &[T]) {
        for line in lines {
            let (left, right) = line.as_ref().split_once(')').unwrap();
            self.orbit.insert(right.to_string(), left.to_string());
        }
    }

    fn count_orbits(&self) -> usize {
        let mut total = 0;
        for mut object in self.orbit.keys() {
            while let Some(parent) = self.orbit.get(object) {
                total += 1;
                object = parent;
            }
        }

        total
    }

    fn orbital_transfers(&self) -> Option<usize> {
        let mut you2com = HashMap::<String, usize>::new();

        let mut object = self.orbit.get("YOU").unwrap();
        let mut steps = 1;
        while let Some(next) = self.orbit.get(object) {
            you2com.insert(next.to_string(), steps);
            steps += 1;
            object = next;
        }

        let mut object = self.orbit.get("SAN").unwrap();
        let mut steps = 1;
        while let Some(next) = self.orbit.get(object) {
            if let Some(yousteps) = you2com.get(next) {
                return Some(steps + yousteps);
            }
            steps += 1;
            object = next;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut aoc = Aoc2019_06::default();
        aoc.init(&[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]);

        assert_eq!(42, aoc.count_orbits());
    }

    #[test]
    fn example2() {
        let mut aoc = Aoc2019_06::default();
        aoc.init(&[
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]);
        assert_eq!(Some(4), aoc.orbital_transfers());
    }
}
