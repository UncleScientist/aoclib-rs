use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_03 {
    nums: Vec<PartNumber>,
    syms: HashSet<(i64, i64)>,
}

impl Aoc2023_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_03 {
    fn name(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-03.txt");

        let mut cur_number: Option<PartNumber> = None;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    if let Some(ref mut num) = cur_number {
                        num.add_digit(row as i64, col as i64, ch);
                    } else {
                        cur_number = Some(PartNumber::new(row as i64, col as i64, ch));
                    }
                } else {
                    if let Some(num) = cur_number.take() {
                        self.nums.push(num);
                    }
                    if ch != '.' {
                        self.syms.insert((row as i64, col as i64));
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self
            .nums
            .iter()
            .filter(|num| num.points.intersection(&self.syms).next().is_some())
            .map(|num| num.value)
            .sum::<i64>();

        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), // left hand side
            (row - 1, col),
            (row + 1, col), // above and below
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), // right hand side
        ]);
        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }
}
