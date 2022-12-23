use std::collections::{HashMap, HashSet};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_23 {
    grove: Grove,
}

impl Aoc2022_23 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_23 {
    fn name(&self) -> (usize, usize) {
        (2022, 23)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-23.txt");
        let _lines = aoclib::read_lines("test-input.txt");
        let _lines = aoclib::read_lines("test2");
        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    self.grove.grove.insert((row as i64, col as i64));
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        // self.grove._draw();

        let mut grove = self.grove.clone();

        for _ in 0..10 {
            grove = grove.round();
        }

        // grove._draw();

        let dim = grove.dimensions();
        let area = (dim.2 - dim.0 + 1) * (dim.3 - dim.1 + 1);

        crate::output(area - grove.elf_count())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Default, Clone)]
struct Grove {
    grove: HashSet<(i64, i64)>,
    dir: usize,
}

impl Grove {
    const DIR: [(i64, i64); 8] = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    const CHECK: [[(i64, i64); 3]; 4] = [
        [(-1, 0), (-1, 1), (-1, -1)],
        [(1, 0), (1, 1), (1, -1)],
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)],
    ];

    fn round(&self) -> Self {
        let mut result = HashSet::new();
        let mut consider: HashMap<(i64, i64), (usize, (i64, i64))> = HashMap::new();
        for g in &self.grove {
            let count = Self::DIR
                .iter()
                .map(|d| (g.0 + d.0, g.1 + d.1))
                .filter(|p| self.grove.contains(p))
                .count();
            if count == 0 {
                result.insert(*g);
            } else {
                let mut found = false;
                for i in 0..4 {
                    let count = Self::CHECK[(i + self.dir) % Self::CHECK.len()]
                        .iter()
                        .map(|d| (g.0 + d.0, g.1 + d.1))
                        .filter(|p| self.grove.contains(p))
                        .count();
                    if count == 0 {
                        let delta = Self::CHECK[(i + self.dir) % Self::CHECK.len()][0];
                        let new_pos = (g.0 + delta.0, g.1 + delta.1);
                        let e = consider.entry(new_pos).or_insert((0, *g));
                        e.0 += 1;
                        if e.0 != 1 {
                            result.insert(*g);
                        } else {
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                    result.insert(*g);
                }
            }
        }

        for (pos, (count, old_pos)) in consider {
            if count == 1 {
                result.insert(pos);
            } else {
                result.insert(old_pos);
            }
        }

        Self {
            grove: result,
            dir: (self.dir + 1) % Self::DIR.len(),
        }
    }

    fn _draw(&self) {
        let (r0, c0, r1, c1) = self.dimensions();

        println!("Elf count: {}", self.grove.len());
        for row in r0..=r1 {
            for col in c0..=c1 {
                if self.grove.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // (rowmin, colmin, rowmax, colmax)
    fn dimensions(&self) -> (i64, i64, i64, i64) {
        self.grove
            .iter()
            .fold((i64::MAX, i64::MAX, i64::MIN, i64::MIN), |a, b| {
                (a.0.min(b.0), a.1.min(b.1), a.2.max(b.0), a.3.max(b.1))
            })
    }

    fn elf_count(&self) -> i64 {
        self.grove.len() as i64
    }
}
