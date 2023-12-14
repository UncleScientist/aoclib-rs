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
        let mut platform = Platform::default();
        platform.width = self.platform.width;
        platform.height = self.platform.height;

        // println!("before:");
        // self.platform._dump();

        for row in 0..self.platform.height {
            for col in 0..self.platform.width {
                match self.platform.rocks.get(&(row, col)) {
                    Some(Rock::Cube) => {
                        platform.rocks.insert((row, col), Rock::Cube);
                    }
                    Some(Rock::Rounded) => {
                        let mut y = row;
                        while y > 0 && !platform.rocks.contains_key(&(y - 1, col)) {
                            y -= 1;
                        }
                        platform.rocks.insert((y, col), Rock::Rounded);
                    }
                    None => {}
                }
            }
        }

        // println!("after:");
        // platform._dump();

        aoclib::output(platform.load())
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
