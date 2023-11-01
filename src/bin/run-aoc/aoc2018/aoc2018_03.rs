use std::collections::HashMap;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_03 {
    claims: Vec<Claim>,
    fabric: HashMap<(usize, usize), usize>,
}

impl Aoc2018_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_03 {
    fn name(&self) -> (usize, usize) {
        (2018, 3)
    }

    fn parse(&mut self) {
        for claim in aoclib::read_lines("input/2018-03.txt").iter() {
            let (id, rest) = claim.split_once(" @ ").unwrap();
            let (xy, wh) = rest.split_once(": ").unwrap();
            let (x, y) = xy.split_once(',').unwrap();
            let (w, h) = wh.split_once('x').unwrap();

            let id = id[1..].parse().unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let w = w.parse().unwrap();
            let h = h.parse().unwrap();

            self.claims.push(Claim { id, x, y, w, h });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for c in &self.claims {
            for x in 0..c.w {
                for y in 0..c.h {
                    *self.fabric.entry((c.x + x, c.y + y)).or_insert(0) += 1;
                }
            }
        }

        crate::output(self.fabric.values().filter(|count| **count >= 2).count())
    }

    fn part2(&mut self) -> Vec<String> {
        'next_claim: for c in &self.claims {
            for x in 0..c.w {
                for y in 0..c.h {
                    if self.fabric.get(&(c.x + x, c.y + y)) != Some(&1) {
                        continue 'next_claim;
                    }
                }
            }
            return crate::output(c.id);
        }

        crate::output("** no answer found **")
    }
}

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}
