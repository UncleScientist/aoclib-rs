use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_12 {
    springs: Vec<Spring>,
}

impl Aoc2023_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_12 {
    fn name(&self) -> (usize, usize) {
        (2023, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-12.txt");

        for line in lines.iter() {
            self.springs.push(line.parse().unwrap());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.springs.iter().map(Spring::score).sum::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

struct Spring {
    pattern: Vec<char>,
    sizes: Vec<usize>,
}

impl FromStr for Spring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, nums) = s.split_once(' ').unwrap();
        let pattern = pattern.chars().collect();
        let sizes = nums.split(',').map(|val| val.parse().unwrap()).collect();
        Ok(Spring { pattern, sizes })
    }
}

impl Spring {
    fn score(&self) -> usize {
        Self::do_score(&self.pattern, &self.sizes)
    }

    fn do_score(pat: &[char], sizes: &[usize]) -> usize {
        if sizes.is_empty() {
            return (!pat.contains(&'#')) as usize;
        }

        let min_remaining = sizes.iter().sum::<usize>() + sizes.len() - 1;

        if pat.len() < min_remaining {
            return 0;
        }

        match pat[0] {
            '.' => Self::do_score(&pat[1..], sizes),
            '#' => Self::do_hash(pat, sizes),
            '?' => Self::do_score(&pat[1..], sizes) + Self::do_hash(pat, sizes),
            _ => panic!("invalid char in input"),
        }
    }

    fn do_hash(pat: &[char], sizes: &[usize]) -> usize {
        if pat.len() < sizes[0] || pat[0..sizes[0]].contains(&'.') {
            return 0;
        }

        if pat.len() == sizes[0] {
            return (sizes.len() == 1) as usize;
        }

        if pat[sizes[0]] == '#' {
            return 0;
        }

        Self::do_score(&pat[sizes[0] + 1..], &sizes[1..])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let spring: Spring = "???.### 1,1,3".parse().unwrap();
        assert_eq!(1, spring.score());
    }

    #[test]
    fn test2() {
        let spring: Spring = ".??..??...?##. 1,1,3".parse().unwrap();
        assert_eq!(4, spring.score());
    }

    #[test]
    fn test3() {
        let spring: Spring = "?#?#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        assert_eq!(1, spring.score());
    }

    #[test]
    fn test4() {
        let spring: Spring = "????.#...#... 4,1,1".parse().unwrap();
        assert_eq!(1, spring.score());
    }

    #[test]
    fn test5() {
        let spring: Spring = "????.######..#####. 1,6,5".parse().unwrap();
        assert_eq!(4, spring.score());
    }

    #[test]
    fn test6() {
        let spring: Spring = "?###???????? 3,2,1".parse().unwrap();
        assert_eq!(10, spring.score());
    }
}
