use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_22 {
    bricks: Vec<Brick>,
    removable: HashSet<usize>,
}

impl Aoc2023_22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_22 {
    fn name(&self) -> (usize, usize) {
        (2023, 22)
    }

    fn parse(&mut self) {
        // let lines = aoclib::read_lines("test-input");
        let lines = aoclib::read_lines("input/2023-22.txt");

        for line in lines {
            self.bricks.push(line.parse().unwrap());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut by_height = self
            .bricks
            .iter()
            .enumerate()
            .map(|(index, b)| (b.lo.2, index))
            .collect::<BTreeSet<_>>();

        self.removable = (0..self.bricks.len()).collect::<HashSet<_>>();

        let mut ground = BTreeSet::<(i64, usize)>::new();
        while let Some((_, fb_index)) = by_height.pop_first() {
            let mut saved_height = 0;
            let mut current_bottom = 0;
            let mut supporter = Vec::new();
            for (gb_top, gb_index) in ground.iter().rev() {
                if saved_height > 0 && *gb_top < current_bottom {
                    break;
                }
                if self.bricks[fb_index].collides_xy(&self.bricks[*gb_index]) {
                    let height = self.bricks[fb_index].hi.2 - self.bricks[fb_index].lo.2;
                    saved_height = gb_top + 1 + height;
                    current_bottom = *gb_top;
                    supporter.push(*gb_index);
                }
            }
            if saved_height == 0 {
                let height = self.bricks[fb_index].hi.2 - self.bricks[fb_index].lo.2 + 1;
                #[cfg(test)]
                println!("brick {fb_index} is on the ground, with a top of {height}");
                ground.insert((height, fb_index));
            } else {
                ground.insert((saved_height, fb_index));
                #[cfg(test)]
                println!(
                    "brick {fb_index} is supported by {supporter:?}, at height {saved_height}"
                );
            }
            if supporter.len() == 1 {
                self.removable.remove(&supporter[0]);
            }
        }

        aoclib::output(self.removable.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

// A --> B -> D \
// \        X    > F -> G
//   \-> C -> E /

#[derive(Debug)]
struct Brick {
    lo: (i64, i64, i64),
    hi: (i64, i64, i64),
}

impl Brick {
    fn collides_xy(&self, other: &Self) -> bool {
        self.lo.0 <= other.hi.0
            && other.lo.0 <= self.hi.0
            && self.lo.1 <= other.hi.1
            && other.lo.1 <= self.hi.1
    }
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('~').unwrap();
        let left = left
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let right = right
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        assert!(left[0] <= right[0]);
        assert!(left[1] <= right[1]);
        assert!(left[2] <= right[2]);
        Ok(Brick {
            lo: (left[0], left[1], left[2]),
            hi: (right[0], right[1], right[2]),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collision() {
        let b1 = Brick {
            lo: (0, 0, 0),
            hi: (4, 4, 4),
        };
        let b2 = Brick {
            lo: (3, 4, 0),
            hi: (8, 9, 0),
        };
        assert!(b1.collides_xy(&b2))
    }

    #[test]
    fn no_collision() {
        let b1 = Brick {
            lo: (0, 0, 0),
            hi: (4, 4, 4),
        };
        let b2 = Brick {
            lo: (5, 6, 0),
            hi: (8, 9, 0),
        };
        assert!(!b1.collides_xy(&b2))
    }

    #[test]
    fn test_bricks() {
        let ba = Brick {
            lo: (1, 0, 1),
            hi: (1, 2, 1),
        };
        let bb = Brick {
            lo: (0, 0, 2),
            hi: (2, 0, 2),
        };
        let bc = Brick {
            lo: (0, 2, 3),
            hi: (2, 2, 3),
        };
        let bd = Brick {
            lo: (0, 0, 4),
            hi: (0, 2, 4),
        };
        let be = Brick {
            lo: (2, 0, 5),
            hi: (2, 2, 5),
        };
        let bf = Brick {
            lo: (0, 1, 6),
            hi: (2, 1, 6),
        };
        let bg = Brick {
            lo: (1, 1, 8),
            hi: (1, 1, 9),
        };
        assert!(ba.collides_xy(&bb));
        assert!(ba.collides_xy(&bc));
        assert!(bd.collides_xy(&bb));
        assert!(bd.collides_xy(&bc));
        assert!(be.collides_xy(&bb));
        assert!(be.collides_xy(&bc));
        assert!(bf.collides_xy(&bd));
        assert!(bf.collides_xy(&be));
        assert!(bf.collides_xy(&bg));
    }
}
