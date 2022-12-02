use crate::Runner;

use std::collections::HashMap;

#[derive(Default)]
pub struct Aoc2017_03 {
    // insert items here (or not, i'm not the boss of you)
}

impl Aoc2017_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_03 {
    fn name(&self) -> (usize, usize) {
        (2017, 3)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let mut hs = HashMap::new();
        let left: HashMap<(i32, i32), (i32, i32)> = HashMap::from_iter([
            ((1, 0), (0, 1)),
            ((0, 1), (-1, 0)),
            ((-1, 0), (0, -1)),
            ((0, -1), (1, 0)),
        ]);

        let mut x = 0;
        let mut y = 0;
        let mut cur_num = 1;
        let mut dir = (1, 0);
        hs.insert((x, y), cur_num);

        while cur_num < 312051 {
            cur_num += 1;
            x += dir.0;
            y += dir.1;
            hs.insert((x, y), cur_num);
            let left_dir = left.get(&dir).unwrap();
            if !hs.contains_key(&(x + left_dir.0, y + left_dir.1)) {
                dir = *left_dir;
            }
        }

        crate::output(x.abs() + y.abs())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
