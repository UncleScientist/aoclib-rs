use std::{cmp::Ordering, collections::HashSet, fmt::Display, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_14 {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

impl Aoc2024_14 {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.tick(self.width, self.height);
        }
    }

    fn quadrants(&self) -> [usize; 4] {
        let mut counts = [0_usize; 4];

        for robot in &self.robots {
            let x = robot.pos.0.cmp(&(self.width / 2));
            let y = robot.pos.1.cmp(&(self.height / 2));

            if let Some(quadrant) = match (x, y) {
                (Ordering::Less, Ordering::Less) => Some(0),
                (Ordering::Less, Ordering::Greater) => Some(1),
                (Ordering::Greater, Ordering::Less) => Some(2),
                (Ordering::Greater, Ordering::Greater) => Some(3),
                _ => None,
            } {
                counts[quadrant] += 1;
            }
        }

        counts
    }

    fn is_tree(&self) -> bool {
        let floor = self
            .robots
            .iter()
            .map(|robot| robot.pos)
            .collect::<HashSet<_>>();

        for y in 0..self.height - 33 {
            let mut seq = 0;
            for x in 0..self.width - 31 {
                if floor.contains(&(x, y)) && floor.contains(&(x, y + 32)) {
                    seq += 1;
                } else if seq == 31 {
                    return true;
                } else {
                    seq = 0;
                }
            }
        }

        false
    }
}

impl Runner for Aoc2024_14 {
    fn name(&self) -> (usize, usize) {
        (2024, 14)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-14.txt");
        self.robots = lines.iter().map(|robot| robot.parse().unwrap()).collect();
        self.width = 101;
        self.height = 103;
    }

    fn part1(&mut self) -> Vec<String> {
        for _ in 0..100 {
            self.step();
        }
        let quads = self.quadrants();
        aoclib::output(quads.iter().product::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        // h + 103 : 175 278 381
        // v + 101 : 103 204 305 406
        let mut seconds = 100;
        while !self.is_tree() {
            self.step();
            seconds += 1;
        }
        aoclib::output(seconds)
    }
}

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}
impl Robot {
    fn tick(&mut self, width: i64, height: i64) {
        self.pos.0 = (self.pos.0 + self.vel.0 + width) % width;
        self.pos.1 = (self.pos.1 + self.vel.1 + height) % height;
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" v=").unwrap();
        let (_, p) = p.split_once('=').unwrap();

        fn get_xy(s: &str) -> (i64, i64) {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }

        Ok(Self {
            pos: get_xy(p),
            vel: get_xy(v),
        })
    }
}

impl Display for Aoc2024_14 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let floor = self
            .robots
            .iter()
            .map(|robot| robot.pos)
            .collect::<HashSet<_>>();

        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if floor.contains(&(x, y)) { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        let robot: Robot = "p=2,4 v=2,-3".parse().unwrap();
        assert_eq!((2, 4), robot.pos);
        assert_eq!((2, -3), robot.vel);
    }

    #[test]
    fn test_one_tick() {
        let mut robot: Robot = "p=2,4 v=2,-3".parse().unwrap();
        robot.tick(11, 7);
        assert_eq!((4, 1), robot.pos);
    }

    #[test]
    fn test_wrapping() {
        let mut robot: Robot = "p=2,4 v=2,-3".parse().unwrap();
        robot.tick(11, 7);
        robot.tick(11, 7);
        assert_eq!((6, 5), robot.pos);
    }

    #[test]
    fn test_quadrants() {
        let lines = [
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ];

        let mut aoc = Aoc2024_14::default();
        aoc.width = 11;
        aoc.height = 7;
        for line in lines {
            aoc.robots.push(line.parse::<Robot>().unwrap());
        }
        for _ in 0..100 {
            aoc.step();
        }

        let quads = aoc.quadrants();

        assert_eq!(12, quads.iter().product::<usize>());
    }
}
