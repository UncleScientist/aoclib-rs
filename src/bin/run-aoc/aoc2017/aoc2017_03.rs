use crate::Runner;

use std::collections::HashMap;

const PUZZLE_INPUT: i32 = 312051;

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
        let mut hm = HashMap::new();
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
        hm.insert((x, y), cur_num);

        while cur_num < PUZZLE_INPUT {
            cur_num += 1;
            x += dir.0;
            y += dir.1;
            hm.insert((x, y), cur_num);
            let left_dir = left.get(&dir).unwrap();
            if !hm.contains_key(&(x + left_dir.0, y + left_dir.1)) {
                dir = *left_dir;
            }
        }

        crate::output(x.abs() + y.abs())
    }

    fn part2(&mut self) -> Vec<String> {
        let surround = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let left: HashMap<(i32, i32), (i32, i32)> = HashMap::from_iter([
            ((1, 0), (0, 1)),
            ((0, 1), (-1, 0)),
            ((-1, 0), (0, -1)),
            ((0, -1), (1, 0)),
        ]);

        let mut hm = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut cur_num = 1;
        let mut dir = (1, 0);
        hm.insert((x, y), cur_num);

        while cur_num < PUZZLE_INPUT {
            cur_num += 1;
            x += dir.0;
            y += dir.1;
            let sum: i32 = surround
                .iter()
                .map(|coord| hm.get(&(x + coord.0, y + coord.1)).unwrap_or(&0))
                .sum();
            hm.insert((x, y), sum);
            if sum > PUZZLE_INPUT {
                return crate::output(sum);
            }
            let left_dir = left.get(&dir).unwrap();
            if !hm.contains_key(&(x + left_dir.0, y + left_dir.1)) {
                dir = *left_dir;
            }
        }
        crate::output("FAILED")
    }
}
