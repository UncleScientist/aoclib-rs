use std::collections::HashSet;

use crate::Runner;

use super::utils::knot_hash;

#[derive(Default)]
pub struct Aoc2017_14 {
    key_string: String,
}

impl Aoc2017_14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_14 {
    fn name(&self) -> (usize, usize) {
        (2017, 14)
    }

    fn parse(&mut self) {
        // self.key_string = "flqrgnkx".to_string();
        self.key_string = "jzgqcdpd".to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(
            (0..128)
                .map(|n| knot_hash(&format!("{}-{n}", self.key_string)))
                .fold(0, |a, b| a + b.count_ones()),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grid = HashSet::new();
        for row in 0..128 {
            let mut hash = knot_hash(&format!("{}-{row}", self.key_string));
            for col in 0..128 {
                if hash & 1 == 1 {
                    grid.insert((row, col));
                }
                hash >>= 1;
            }
        }

        let mut count = 0;
        let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for row in 0..128 {
            for col in 0..128 {
                if grid.contains(&(row, col)) {
                    count += 1;

                    let mut stack = vec![(row, col)];
                    while let Some(pos) = stack.pop() {
                        grid.remove(&pos);
                        for d in &dir {
                            if grid.contains(&(pos.0 + d.0, pos.1 + d.1)) {
                                stack.push((pos.0 + d.0, pos.1 + d.1));
                            }
                        }
                    }
                }
            }
        }

        crate::output(count)
    }
}
