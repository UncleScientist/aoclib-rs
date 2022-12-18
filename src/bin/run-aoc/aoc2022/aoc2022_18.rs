use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_18 {
    cubes: HashSet<(i32, i32, i32)>,
}

impl Aoc2022_18 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_18 {
    fn name(&self) -> (usize, usize) {
        (2022, 18)
    }

    fn parse(&mut self) {
        let _lines: Vec<Vec<i32>> = aoclib::numbers("test-input.txt", ',');
        let lines: Vec<Vec<i32>> = aoclib::numbers("input/2022-18.txt", ',');

        for line in lines {
            self.cubes.insert((line[0], line[1], line[2]));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut sides = self.cubes.len() * 6;
        let delta_xyz = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];

        for c in &self.cubes {
            for d in delta_xyz {
                let pos = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
                if self.cubes.contains(&pos) {
                    sides -= 1;
                }
            }
        }

        crate::output(sides)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
