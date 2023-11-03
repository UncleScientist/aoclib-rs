use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_05 {
    reactant: String,
}

impl Aoc2018_05 {
    pub fn new() -> Self {
        Self::default()
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
        crate::output(react(self.reactant.clone()))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut smallest = self.reactant.len();
        for letter in 'a'..='z' {
            let reactant = self
                .reactant
                .chars()
                .filter(|x| x.to_ascii_lowercase() != letter)
                .collect::<String>();
            smallest = smallest.min(react(reactant));
        }
        crate::output(smallest)
    }
}

fn step(s: &str) -> String {
    let mut result = String::new();
    let mut iter = s.chars().chain(['.']);
    let mut left = iter.next().unwrap();
    while let Some(right) = iter.next() {
        if left.is_lowercase() == right.is_lowercase()
            || left.to_ascii_lowercase() != right.to_ascii_lowercase()
        {
            result.push(left);
            left = right;
        } else if let Some(next_left) = iter.next() {
            left = next_left;
        } else {
            break;
        }
    }
    result
}

fn react(mut r: String) -> usize {
    let mut len = r.len();
    let mut new_len = 0;
    while len != new_len {
        len = new_len;
        r = step(&r);
        new_len = r.len();
    }
    len
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_of_string() {
        assert_eq!("bcde".to_string(), step("Aabcde"));
    }

    #[test]
    fn end_of_string() {
        assert_eq!("abcd".to_string(), step("abcdeE"));
    }

    #[test]
    fn test_example() {
        let reactant = step("dabAcCaCBAcCcaDA");
        assert_eq!("dabAaCBAcaDA".to_string(), reactant);
        let reactant = step(&reactant);
        assert_eq!("dabCBAcaDA".to_string(), reactant);
    }
}
