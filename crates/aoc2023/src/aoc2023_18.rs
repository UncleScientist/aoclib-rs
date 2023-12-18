use std::{collections::HashSet, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_18 {
    cmds: Vec<DigCommand>,
}

impl Aoc2023_18 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_18 {
    fn name(&self) -> (usize, usize) {
        (2023, 18)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-18.txt");
        // let lines = aoclib::read_lines("test-input");

        for line in lines {
            self.cmds.push(line.parse().unwrap());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut trench = HashSet::new();

        let mut row = 0isize;
        let mut col = 0isize;

        trench.insert((row, col));

        let mut min = (0, 0);
        let mut max = (0, 0);

        // outline the trench
        for cmd in &self.cmds {
            let (dir, dist) = (cmd.dir.get_dir(), cmd.dist);
            for _ in 0..dist {
                row += dir.0;
                col += dir.1;
                trench.insert((row, col));
            }
            min = (min.0.min(row), min.1.min(col));
            max = (max.0.max(row), max.1.max(col));
        }

        // find where to start
        let mut start_col = None;
        for col in min.1..=max.1 {
            if trench.contains(&(min.0, col)) {
                start_col = Some(col);
                break;
            }
        }

        let Some(start_col) = start_col else {
            panic!("Could not find a place to start");
        };

        // dig out the trench
        let mut stack = vec![(min.0 + 1, start_col + 1)];

        while let Some(pos) = stack.pop() {
            if !trench.insert(pos) {
                continue;
            }
            stack.push((pos.0 - 1, pos.1));
            stack.push((pos.0 + 1, pos.1));
            stack.push((pos.0, pos.1 - 1));
            stack.push((pos.0, pos.1 + 1));
        }

        /*
        for row in min.0..=max.0 {
            for col in min.1..=max.1 {
                if trench.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        */

        aoclib::output(trench.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct DigCommand {
    dir: Direction,
    dist: usize,
    _color: Color,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_dir(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug)]
struct Color(usize);

impl FromStr for DigCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<_>>();
        let dir: Direction = words[0].parse().unwrap();
        let dist: usize = words[1].parse().unwrap();
        let _color: Color = words[2].parse().unwrap();
        Ok(DigCommand { dir, dist, _color })
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(format!("Couldn't interpret direction {s}")),
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(color) = s.strip_suffix(')') else {
            return Err(format!("Bad color format {s}"));
        };
        let Some(color) = color.strip_prefix("(#") else {
            return Err(format!("Bad color format {s}"));
        };
        Ok(Color(usize::from_str_radix(color, 16).map_err(|_| {
            format!("could not parse {color} as a color")
        })?))
    }
}
