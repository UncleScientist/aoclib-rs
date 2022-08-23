use crate::Runner;
use aoclib::read_lines;
use std::iter::zip;

pub struct Aoc2015_05 {
    data: Vec<String>,
}

impl Aoc2015_05 {
    pub fn new() -> Self {
        Self {
            data: read_lines("input/2015-05.txt"),
        }
    }

    pub fn new_from_string(s: String) -> Self {
        Self { data: vec![s] }
    }
}

impl Runner for Aoc2015_05 {
    fn name(&self) -> (usize, usize) {
        (2015, 5)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut nice = 0;
        'outer: for d in &self.data {
            let vowels = d
                .chars()
                .filter(|x| matches!(x, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count();

            if vowels < 3 {
                continue;
            }

            let mut found = false;
            for pair in zip(d.chars(), d.chars().skip(1)) {
                if pair.0 == pair.1 {
                    found = true;
                    break;
                }
            }

            if !found {
                continue;
            }

            for bad in vec!["ab", "cd", "pq", "xy"] {
                if d.find(bad).is_some() {
                    continue 'outer;
                }
            }

            nice += 1;
        }

        vec![format!("{nice}")]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut nice = 0;
        for d in &self.data {
            let mut found = false;
            for pair in zip(d.chars(), d.chars().skip(1)).enumerate() {
                let fstring = format!("{}{}", pair.1 .0, pair.1 .1);
                let f = fstring.as_str();
                if let Some(index) = d.rfind(f) {
                    if index > pair.0 + 1 {
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                continue;
            }

            let mut found = false;
            for pair in zip(d.chars(), d.chars().skip(2)) {
                if pair.0 == pair.1 {
                    found = true;
                    break;
                }
            }

            if found {
                nice += 1;
            }
        }

        vec![format!("{nice}")]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_nice() {
        let mut x = Aoc2015_05::new_from_string("qjhvhtzxzqqjkmpb".to_string());
        assert_eq!(x.part2(), vec!["1".to_string()]);
    }

    #[test]
    fn should_be_naughty() {
        let mut x = Aoc2015_05::new_from_string("uurcxstgmygtbstg".to_string());
        assert_eq!(x.part2(), vec!["0".to_string()]);
    }

    #[test]
    fn should_be_naughty2() {
        let mut x = Aoc2015_05::new_from_string("ieodomkazucvgmuy".to_string());
        assert_eq!(x.part2(), vec!["0".to_string()]);
    }

    #[test]
    fn should_be_naughty3() {
        let mut x = Aoc2015_05::new_from_string("aaa".to_string());
        assert_eq!(x.part2(), vec!["0".to_string()]);
    }

    #[test]
    fn should_be_nice2() {
        let mut x = Aoc2015_05::new_from_string("xxyxx".to_string());
        assert_eq!(x.part2(), vec!["1".to_string()]);
    }
}
