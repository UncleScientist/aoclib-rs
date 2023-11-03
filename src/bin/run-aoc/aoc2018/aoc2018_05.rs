use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_05 {
    reactant: String,
}

impl Aoc2018_05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&mut self) {
        let mut result = String::new();
        let mut iter = self.reactant.chars().chain(['.'].into_iter());
        let mut left = iter.next().unwrap();
        while let Some(right) = iter.next() {
            if left.is_lowercase() == right.is_lowercase()
                || left.to_ascii_lowercase() != right.to_ascii_lowercase()
            {
                result.push(left);
                left = right;
                continue;
            }
            if let Some(next_left) = iter.next() {
                left = next_left;
            } else {
                break;
            }
        }
        self.reactant = result;
    }
}

impl Runner for Aoc2018_05 {
    fn name(&self) -> (usize, usize) {
        (2018, 5)
    }

    fn parse(&mut self) {
        self.reactant = aoclib::read_lines("input/2018-05.txt")[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut len = self.reactant.len();
        let mut new_len = 0;
        while len != new_len {
            len = new_len;
            self.step();
            new_len = self.reactant.len();
        }
        crate::output(len)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_of_string() {
        let mut t = Aoc2018_05::new();
        t.reactant = "Aabcde".to_string();
        t.step();
        assert_eq!("bcde".to_string(), t.reactant);
    }

    #[test]
    fn end_of_string() {
        let mut t = Aoc2018_05::new();
        t.reactant = "abcdeE".to_string();
        t.step();
        assert_eq!("abcd".to_string(), t.reactant);
    }

    #[test]
    fn test_example() {
        let mut t = Aoc2018_05::new();
        t.reactant = "dabAcCaCBAcCcaDA".to_string();
        t.step();
        assert_eq!("dabAaCBAcaDA".to_string(), t.reactant);
        t.step();
        assert_eq!("dabCBAcaDA".to_string(), t.reactant);
    }
}
