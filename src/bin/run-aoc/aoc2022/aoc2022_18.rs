use std::collections::{HashSet, VecDeque};

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

        for c in &self.cubes {
            for d in Self::DELTA_XYZ {
                let pos = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
                if self.cubes.contains(&pos) {
                    sides -= 1;
                }
            }
        }

        crate::output(sides)
    }

    // +                         +
    //       ####    ##########
    //    ####    #########
    //    #####      ########
    //    ########     #########
    //    ####################
    //        #############
    // +                         +
    fn part2(&mut self) -> Vec<String> {
        let mut xrange = (i32::MAX, i32::MIN);
        let mut yrange = (i32::MAX, i32::MIN);
        let mut zrange = (i32::MAX, i32::MIN);

        for c in &self.cubes {
            xrange.0 = xrange.0.min(c.0);
            xrange.1 = xrange.1.max(c.0);
            yrange.0 = yrange.0.min(c.1);
            yrange.1 = yrange.1.max(c.1);
            zrange.0 = zrange.0.min(c.2);
            zrange.1 = zrange.1.max(c.2);
        }

        xrange = (xrange.0 - 1, xrange.1 + 1);
        yrange = (yrange.0 - 1, yrange.1 + 1);
        zrange = (zrange.0 - 1, zrange.1 + 1);

        let mut seen = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((xrange.0, yrange.0, zrange.0));

        let mut count = 0;
        while let Some(pos) = to_visit.pop_front() {
            if !seen.insert(pos) {
                continue;
            }

            for d in Self::DELTA_XYZ {
                let next_pos = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);

                if next_pos.0 < xrange.0
                    || next_pos.0 > xrange.1
                    || next_pos.1 < yrange.0
                    || next_pos.1 > yrange.1
                    || next_pos.2 < zrange.0
                    || next_pos.2 > zrange.1
                {
                    continue;
                }

                if self.cubes.contains(&next_pos) {
                    count += 1;
                } else {
                    to_visit.push_back(next_pos);
                }
            }
        }

        crate::output(count)
    }
}

impl Aoc2022_18 {
    const DELTA_XYZ: [(i32, i32, i32); 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
}
