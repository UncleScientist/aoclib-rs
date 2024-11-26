use std::{collections::HashMap, fmt::Display};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2018_22 {
    cave: Cave,
}

impl Aoc2018_22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_22 {
    fn name(&self) -> (usize, usize) {
        (2018, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-22.txt");
        let depth = lines[0].strip_prefix("depth: ").unwrap();
        let target = lines[1].strip_prefix("target: ").unwrap();

        let depth = depth.parse::<i64>().unwrap();
        let (x, y) = target.split_once(',').unwrap();
        let target = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        self.cave = Cave::new(depth, target);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.cave.get_risk_level())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl RegionType {
    fn risk(&self) -> i64 {
        match self {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }
}

impl Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegionType::Rocky => write!(f, "."),
            RegionType::Wet => write!(f, "="),
            RegionType::Narrow => write!(f, "|"),
        }
    }
}

#[derive(Debug, Default)]
struct Cave {
    depth: i64,
    target: (i64, i64),
    map: HashMap<(i64, i64), i64>, // location -> erosion_level
}

impl Cave {
    fn new(depth: i64, target: (i64, i64)) -> Self {
        Self {
            depth,
            target,
            ..Self::default()
        }
    }

    fn get_risk_level(&mut self) -> i64 {
        (0..=self.target.1)
            .map(|y| (0..=self.target.0).fold(0, |sum, x| sum + self.get_type(&(x, y)).risk()))
            .sum::<i64>()
    }

    fn get_type(&mut self, loc: &(i64, i64)) -> RegionType {
        match self.get_erosion_level(loc) % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => panic!("compiler error"),
        }
    }

    fn get_erosion_level(&mut self, loc: &(i64, i64)) -> i64 {
        if let Some(el) = self.map.get(loc) {
            return *el;
        }
        let index = (self.depth + self.get_geologic_index(loc)) % 20183;
        self.map.insert(*loc, index);
        index
    }

    fn get_geologic_index(&mut self, loc: &(i64, i64)) -> i64 {
        match loc {
            (0, 0) => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => {
                if *loc == self.target {
                    0
                } else {
                    self.get_erosion_level(&(x - 1, *y)) * self.get_erosion_level(&(*x, y - 1))
                }
            }
        }
    }

    fn _print(&mut self) {
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                print!("{}", self.get_type(&(x, y)));
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_geologic_index() {
        let mut cave = Cave::new(510, (10, 10));
        assert_eq!(0, cave.get_geologic_index(&(0, 0)));
        assert_eq!(0, cave.get_geologic_index(&(10, 10)));
        assert_eq!(16807, cave.get_geologic_index(&(1, 0)));
        assert_eq!(48271, cave.get_geologic_index(&(0, 1)));
        assert_eq!(145722555, cave.get_geologic_index(&(1, 1)));
    }

    #[test]
    fn test_part1() {
        let mut cave = Cave::new(510, (10, 10));
        cave._print();
        println!("{:?}", cave.map);
        assert_eq!(114, cave.get_risk_level());
    }
}
