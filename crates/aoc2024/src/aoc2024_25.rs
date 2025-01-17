use std::str::FromStr;

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2024_25 {
    locks: Vec<Schematic>,
    keys: Vec<Schematic>,
}

impl Aoc2024_25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_25 {
    fn name(&self) -> (usize, usize) {
        (2024, 25)
    }

    fn parse(&mut self) {
        let records = aoclib::read_text_records("input/2024-25.txt");
        let schematics: Vec<Schematic> = records.parse_lines();
        let (locks, keys): (Vec<_>, Vec<_>) = schematics
            .into_iter()
            .partition(|schem| matches!(schem, Schematic::Lock(..)));
        self.locks = locks;
        self.keys = keys;
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if key.fits(lock) {
                    count += 1;
                }
            }
        }

        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("Merry Christmas!")
    }
}

#[derive(Debug)]
enum Schematic {
    Lock([usize; 5]),
    Key([usize; 5]),
}

impl Schematic {
    fn fits(&self, lock: &Schematic) -> bool {
        assert!(matches!(self, Schematic::Key(..)));
        let (Schematic::Lock(l), Schematic::Key(k)) = (lock, self) else {
            panic!("did not get lock & key: {lock:?} {self:?}");
        };
        l.iter()
            .zip(k.iter())
            .map(|(lval, kval)| lval + kval)
            .all(|sum| sum <= 5)
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .split('\n')
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let mut pattern = [0; 5];
        for line in &lines[1..lines.len() - 1] {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    pattern[col] += 1;
                }
            }
        }
        if lines[0] == "#####" {
            Ok(Self::Lock(pattern))
        } else {
            Ok(Self::Key(pattern))
        }
    }
}
