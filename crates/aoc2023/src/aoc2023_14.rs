use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_14 {
    platform: Platform,
}

impl Aoc2023_14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_14 {
    fn name(&self) -> (usize, usize) {
        (2023, 14)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-14.txt");
        // let lines = aoclib::read_lines("test-input");

        self.platform.width = lines[0].len() as i64;
        self.platform.height = lines.len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '.' => {}
                    'O' => {
                        self.platform
                            .rocks
                            .insert((row as i64, col as i64), Rock::Rounded);
                    }
                    '#' => {
                        self.platform
                            .rocks
                            .insert((row as i64, col as i64), Rock::Cube);
                    }
                    _ => panic!("unexpected character {ch}"),
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let north_tilted_platform = self.platform.tilt(Direction::North);
        aoclib::output(north_tilted_platform.load())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Rock {
    Rounded,
    Cube,
}

#[derive(Default, Clone)]
struct Platform {
    rocks: HashMap<(i64, i64), Rock>,
    width: i64,
    height: i64,
}

impl Platform {
    fn load(&self) -> i64 {
        self.rocks
            .iter()
            .filter_map(|(k, v)| {
                if *v == Rock::Rounded {
                    Some(self.height - k.0)
                } else {
                    None
                }
            })
            .sum::<i64>()
    }

    fn tilt(&self, dir: Direction) -> Platform {
        let mut platform = Platform::default();
        platform.width = self.width;
        platform.height = self.height;

        let down = (0..self.height).collect::<Vec<_>>();
        let up = (0..self.height).rev().collect::<Vec<_>>();
        let right = (0..self.width).rev().collect::<Vec<_>>();
        let left = (0..self.width).collect::<Vec<_>>();

        let (rows, cols, dr, dc) = match dir {
            Direction::North => (down, right, -1, 0),
            Direction::West => (down, left, 0, -1),
            Direction::South => (up, right, 1, 0),
            Direction::East => (up, left, 0, 1),
        };

        for row in &rows {
            for col in &cols {
                match self.rocks.get(&(*row, *col)) {
                    Some(Rock::Cube) => {
                        platform.rocks.insert((*row, *col), Rock::Cube);
                    }
                    Some(Rock::Rounded) => {
                        let mut r = *row;
                        let mut c = *col;
                        while r + dr >= 0
                            && c + dc >= 0
                            && r + dr < self.height
                            && c + dc < self.height
                            && !platform.rocks.contains_key(&(r + dr, c + dc))
                        {
                            r += dr;
                            c += dc;
                        }
                        platform.rocks.insert((r, c), Rock::Rounded);
                    }
                    None => {}
                }
            }
        }

        platform
    }

    fn _dump(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!(
                    "{}",
                    match self.rocks.get(&(row, col)) {
                        None => '.',
                        Some(Rock::Rounded) => 'O',
                        Some(Rock::Cube) => '#',
                    }
                );
            }
            println!();
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    North,
    West,
    South,
    East,
}
