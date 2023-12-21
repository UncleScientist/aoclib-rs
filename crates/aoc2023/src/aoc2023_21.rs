use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_21 {
    garden: HashSet<(i64, i64)>,
    start: (i64, i64),
    width: i64,
    height: i64,
}

impl Aoc2023_21 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Debug for Aoc2023_21 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.garden.contains(&(row, col)) {
                    write!(f, "#")?;
                } else if (row, col) == self.start {
                    write!(f, "S")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Runner for Aoc2023_21 {
    fn name(&self) -> (usize, usize) {
        (2023, 21)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-21.txt");
        // let lines = aoclib::read_lines("test-input");

        self.height = lines.len() as i64;
        self.width = lines[0].len() as i64;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        self.garden.insert((row as i64, col as i64));
                    }
                    '.' => {}
                    'S' => {
                        self.start = (row as i64, col as i64);
                    }
                    _ => {
                        panic!("could not understand char {ch}");
                    }
                }
            }
        }

        // println!("{self:?}");
    }

    fn part1(&mut self) -> Vec<String> {
        let lim = if self.height == 131 { 64 } else { 6 };
        let mut visited = HashSet::new();
        let mut next_visited = HashSet::new();

        visited.insert(self.start);

        for _ in 0..lim {
            next_visited.clear();
            for v in &visited {
                for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    let pos = (v.0 + dir.0, v.1 + dir.1);
                    if !self.garden.contains(&pos) {
                        next_visited.insert(pos);
                    }
                }
            }

            std::mem::swap(&mut visited, &mut next_visited);
        }

        aoclib::output(visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
