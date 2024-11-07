use std::{collections::HashMap, fmt::Display};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2018_18 {
    area: HashMap<(usize, usize), Contents>,
    rows: usize,
    cols: usize,
}

impl Aoc2018_18 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_18 {
    fn name(&self) -> (usize, usize) {
        (2018, 18)
    }

    fn parse(&mut self) {
        let data = aoclib::read_lines("input/2018-18.txt");
        self.extract(&data);
    }

    fn part1(&mut self) -> Vec<String> {
        for _ in 0..10 {
            self.step();
        }
        aoclib::output(self.resource_value())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

impl Aoc2018_18 {
    fn extract(&mut self, data: &[impl AsRef<str>]) {
        self.rows = data.len();
        self.cols = data[0].as_ref().len();
        for (row, line) in data.iter().enumerate() {
            for (col, ch) in line.as_ref().chars().enumerate() {
                let contents = match ch {
                    '#' => Contents::Lumberyard,
                    '|' => Contents::Wooded,
                    '.' => Contents::Open,
                    _ => panic!("error in your code"),
                };
                self.area.insert((row, col), contents);
            }
        }
    }

    fn step(&mut self) {
        let mut new_area = HashMap::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let dirs = [
                    (row.wrapping_sub(1), col.wrapping_sub(1)),
                    (row.wrapping_sub(1), col),
                    (row.wrapping_sub(1), col + 1),
                    (row, col.wrapping_sub(1)),
                    // (row, col),
                    (row, col + 1),
                    (row + 1, col.wrapping_sub(1)),
                    (row + 1, col),
                    (row + 1, col + 1),
                ];
                let mut wooded = 0;
                let mut lumberyards = 0;
                for dir in &dirs {
                    match self.area.get(dir) {
                        Some(Contents::Wooded) => wooded += 1,
                        Some(Contents::Lumberyard) => lumberyards += 1,
                        Some(Contents::Open) | None => {}
                    }
                }
                let new = match self.area.get(&(row, col)).unwrap() {
                    Contents::Open if wooded >= 3 => Contents::Wooded,
                    Contents::Wooded if lumberyards >= 3 => Contents::Lumberyard,
                    Contents::Lumberyard if lumberyards == 0 || wooded == 0 => Contents::Open,
                    x => *x,
                };
                new_area.insert((row, col), new);
            }
        }
        self.area = new_area;
    }

    fn resource_value(&self) -> usize {
        let mut wooded = 0;
        let mut lumberyards = 0;
        for c in self.area.values() {
            match c {
                Contents::Open => {}
                Contents::Wooded => wooded += 1,
                Contents::Lumberyard => lumberyards += 1,
            }
        }
        wooded * lumberyards
    }
}

#[derive(Copy, Clone)]
enum Contents {
    Open,
    Wooded,
    Lumberyard,
}

impl Display for Aoc2018_18 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.area.get(&(row, col)).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Display for Contents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Contents::Open => write!(f, "."),
            Contents::Wooded => write!(f, "|"),
            Contents::Lumberyard => write!(f, "#"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = vec![
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ];
        let mut aoc = Aoc2018_18::default();
        aoc.extract(&data);
        for _ in 0..10 {
            aoc.step();
            println!("{aoc}");
        }
        assert_eq!(1147, aoc.resource_value());
    }
}
