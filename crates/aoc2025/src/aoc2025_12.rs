use std::{collections::HashSet, convert::Infallible, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_12 {
    presents: Vec<Present>,
    trees: Vec<Tree>,
}

impl Aoc2025_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_12 {
    fn name(&self) -> (usize, usize) {
        (2025, 12)
    }

    fn parse(&mut self) {
        let records = aoclib::read_text_records("input/2025-12.txt");
        for (idx, record) in records[0..records.len() - 1].iter().enumerate() {
            let lines = record.split('\n').collect::<Vec<_>>();
            let num = lines[0][0..lines[0].len() - 1].parse::<usize>().unwrap();
            assert_eq!(idx, num);
            self.presents.push(Present::from(&lines[1..]));
        }
        self.trees = records[records.len() - 1]
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count = 0;
        for tree in &self.trees {
            let tree_area = tree.area();
            let presents_area = tree
                .quants
                .iter()
                .enumerate()
                .map(|(present, amount)| self.presents[present].pos.len() * amount)
                .sum::<usize>() as isize;
            if tree_area >= presents_area {
                count += 1;
            }
        }
        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("Merry Christmas!")
    }
}

#[derive(Debug)]
struct Present {
    pos: HashSet<(isize, isize)>,
}

impl Present {
    fn from<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut pos = HashSet::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.as_ref().chars().enumerate() {
                if ch == '#' {
                    pos.insert((row as isize, col as isize));
                }
            }
        }
        Self { pos }
    }
}

#[derive(Debug)]
struct Tree {
    width: isize,
    height: isize,
    quants: Vec<usize>,
}

impl Tree {
    fn area(&self) -> isize {
        self.width * self.height
    }
}

impl FromStr for Tree {
    type Err = Infallible;

    // 47x39: 64 44 38 43 38 53
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let (width, height) = left.split_once('x').unwrap();
        let width = width.parse().unwrap();
        let height = height.parse().unwrap();
        let quants = right
            .split(' ')
            .map(|q| q.parse().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            width,
            height,
            quants,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_present() {
        let expected = HashSet::<(isize, isize)>::from([
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (2, 0),
            (2, 1),
            (2, 2),
        ]);
        assert_eq!(expected, Present::from(&["###", "#..", "###"]).pos);
    }
}
