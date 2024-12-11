use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_11 {
    stones: Vec<Stone>,
}

impl Aoc2024_11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_11 {
    fn name(&self) -> (usize, usize) {
        (2024, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-11.txt");
        let _lines = [String::from("125 17")];
        self.stones = lines[0]
            .split_whitespace()
            .map(|stone| stone.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut current = self.stones.clone();
        for _ in 0..25 {
            let mut next = Vec::new();
            for stone in current {
                next.extend(stone.split());
            }
            current = next;
        }
        aoclib::output(current.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Stone {
    val: usize,
}

impl Stone {
    fn split(&self) -> Vec<Self> {
        if self.val == 0 {
            return vec![Self { val: 1 }];
        }

        let s = format!("{}", self.val);
        if s.len() % 2 == 0 {
            let mid = s.len() / 2;
            let (s1, s2) = (&s[0..mid], &s[mid..]);
            return vec![s1.parse::<Stone>().unwrap(), s2.parse::<Stone>().unwrap()];
        }

        vec![Self {
            val: self.val * 2024,
        }]
    }
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            val: s.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero() {
        let stone = Stone { val: 0 };
        assert_eq!(vec![Stone { val: 1 }], stone.split());
    }

    #[test]
    fn test_split() {
        let stone = Stone { val: 1000 };
        assert_eq!(vec![Stone { val: 10 }, Stone { val: 0 }], stone.split());
    }

    #[test]
    fn test_split_non_octal() {
        let stone = Stone { val: 1009 };
        assert_eq!(vec![Stone { val: 10 }, Stone { val: 9 }], stone.split());
    }

    #[test]
    fn test_split_multiply() {
        let stone = Stone { val: 100 };
        assert_eq!(vec![Stone { val: 202400 }], stone.split());
    }
}
