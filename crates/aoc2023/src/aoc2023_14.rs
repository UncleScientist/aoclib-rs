use std::collections::{BTreeSet, HashMap};

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
        let mut tiltify = self.platform.clone();
        let mut loop_detector = HashMap::new();

        let keyvec = tiltify.rocks.keys().copied().collect::<BTreeSet<_>>();
        loop_detector.insert(keyvec, 0);

        let mut i = 0;
        let (start, end) = loop {
            i += 1;
            for dir in &DIRS {
                tiltify = tiltify.tilt(*dir);
            }
            let keyvec = tiltify.rocks.keys().copied().collect::<BTreeSet<_>>();
            if let Some(val) = loop_detector.insert(keyvec, i) {
                break (val, i);
            }
        };

        let diff = end - start;
        let remaining_loops = 1000000000 - start;
        let phase = remaining_loops % diff;

        for _ in 0..phase {
            for dir in &DIRS {
                tiltify = tiltify.tilt(*dir);
            }
        }

        aoclib::output(tiltify.load())
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
    fn prep(&self) -> Platform {
        Platform {
            rocks: HashMap::new(),
            width: self.width,
            height: self.height,
        }
    }

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
        let mut platform = self.prep();

        let down = (0..self.height).collect::<Vec<_>>();
        let up = (0..self.height).rev().collect::<Vec<_>>();
        let right = (0..self.width).rev().collect::<Vec<_>>();
        let left = (0..self.width).collect::<Vec<_>>();

        let (rows, cols, dr, dc) = match dir {
            Direction::North => (down, right, -1, 0),
            Direction::West => (down, left, 0, -1),
            Direction::South => (up, right, 1, 0),
            Direction::East => (up, right, 0, 1),
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

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}
