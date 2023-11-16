use std::{collections::HashMap, fmt::Display};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_13 {
    data: HashMap<(i64, i64), TrackType>,
    carts: Vec<Cart>,
}

impl Aoc2018_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_13 {
    fn name(&self) -> (usize, usize) {
        (2018, 13)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-13.txt");

        let mut y = 0;
        for l in lines {
            let mut x = 0;
            for c in l.chars() {
                if matches!(c, '^' | 'v' | '<' | '>') {
                    let dir = match c {
                        '^' => Direction::North,
                        'v' => Direction::South,
                        '>' => Direction::East,
                        '<' => Direction::West,
                        _ => panic!("someone broke the rust compiler"),
                    };
                    self.carts.push(Cart {
                        next_move: Move::TurnLeft,
                        direction: dir,
                        pos: (x, y),
                    })
                } else {
                    match c {
                        '|' => self.data.insert((x, y), TrackType::NorthSouth),
                        '-' => self.data.insert((x, y), TrackType::EastWest),
                        '/' => self.data.insert((x, y), TrackType::NwSeCorner),
                        '\\' => self.data.insert((x, y), TrackType::NeSwCorner),
                        '+' => self.data.insert((x, y), TrackType::Intersection),
                        ' ' => None,
                        _ => panic!("unexpected input {c}"),
                    };
                }
                x += 1;
            }
            y += 1;
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for y in 0..15 {
            for x in 0..80 {
                if let Some(track) = self.data.get(&(x, y)) {
                    print!("{track}");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        crate::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

struct Cart {
    next_move: Move,
    direction: Direction,
    pos: (i64, i64),
}

enum Direction {
    North,
    South,
    East,
    West,
}

enum Move {
    TurnLeft,
    TurnRight,
    StraightOn,
}

impl Move {
    fn succ(&self) -> Self {
        match self {
            Self::TurnLeft => Self::StraightOn,
            Self::StraightOn => Self::TurnRight,
            Self::TurnRight => Self::TurnLeft,
        }
    }
}

enum TrackType {
    NeSwCorner,
    NwSeCorner,
    EastWest,
    NorthSouth,
    Intersection,
}
impl Display for TrackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NeSwCorner => '\\',
                Self::NwSeCorner => '/',
                Self::EastWest => '-',
                Self::NorthSouth => '|',
                Self::Intersection => '+',
            }
        )
    }
}
