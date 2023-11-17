use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_13 {
    data: HashMap<(usize, usize), TrackType>,
    carts: Vec<Cart>,
}

impl Aoc2018_13 {
    pub fn new() -> Self {
        Self::default()
    }

    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut prev_pos: HashSet<(usize, usize)> =
            HashSet::from_iter(self.carts.iter().map(|c| c.pos));

        for c in self.carts.iter_mut() {
            let mut direction = c.direction;
            let mut next_move = c.next_move;

            let (x, y) = match c.direction {
                Direction::North => (c.pos.0, c.pos.1 - 1),
                Direction::South => (c.pos.0, c.pos.1 + 1),
                Direction::East => (c.pos.0 + 1, c.pos.1),
                Direction::West => (c.pos.0 - 1, c.pos.1),
            };

            if prev_pos.contains(&(x, y)) {
                return Some((x, y));
            }

            let track = self.data.get(&(x, y)).unwrap();
            match track {
                TrackType::EastWest | TrackType::NorthSouth => {}
                TrackType::Intersection => {
                    direction = direction.turn(next_move);
                    next_move = next_move.succ();
                }
                TrackType::NeSwCorner => {
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                    };
                }
                TrackType::NwSeCorner => {
                    direction = match direction {
                        Direction::North => Direction::West,
                        Direction::South => Direction::East,
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                    };
                }
            }

            prev_pos.remove(&c.pos);
            prev_pos.insert((x, y));

            c.direction = direction;
            c.next_move = next_move;
            c.pos = (x, y);
        }

        self.carts.sort_by(|c1, c2| {
            if c1.pos.1 != c2.pos.1 {
                c1.pos.1.cmp(&c2.pos.1)
            } else {
                c1.pos.0.cmp(&c2.pos.0)
            }
        });

        None
    }
}

impl Runner for Aoc2018_13 {
    fn name(&self) -> (usize, usize) {
        (2018, 13)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-13.txt");
        // let lines = aoclib::read_lines("test-input");

        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if matches!(c, '^' | 'v' | '<' | '>') {
                    let dir = match c {
                        '^' => {
                            self.data.insert((x, y), TrackType::NorthSouth);
                            Direction::North
                        }
                        'v' => {
                            self.data.insert((x, y), TrackType::NorthSouth);
                            Direction::South
                        }
                        '>' => {
                            self.data.insert((x, y), TrackType::EastWest);
                            Direction::East
                        }
                        '<' => {
                            self.data.insert((x, y), TrackType::EastWest);
                            Direction::West
                        }
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
                        '/' => self.data.insert((x, y), TrackType::NeSwCorner),
                        '\\' => self.data.insert((x, y), TrackType::NwSeCorner),
                        '+' => self.data.insert((x, y), TrackType::Intersection),
                        ' ' => None,
                        _ => panic!("unexpected input {c}"),
                    };
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        /*
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
        */

        loop {
            if let Some((x, y)) = self.tick() {
                return crate::output(format!("{x},{y}"));
            }
        }
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Cart {
    next_move: Move,
    direction: Direction,
    pos: (usize, usize),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, m: Move) -> Self {
        match self {
            Self::North => match m {
                Move::TurnLeft => Self::West,
                Move::StraightOn => Self::North,
                Move::TurnRight => Self::East,
            },
            Self::South => match m {
                Move::TurnLeft => Self::East,
                Move::StraightOn => Self::South,
                Move::TurnRight => Self::West,
            },
            Self::East => match m {
                Move::TurnLeft => Self::North,
                Move::StraightOn => Self::East,
                Move::TurnRight => Self::South,
            },
            Self::West => match m {
                Move::TurnLeft => Self::South,
                Move::StraightOn => Self::West,
                Move::TurnRight => Self::North,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug)]
enum TrackType {
    NeSwCorner, // forward-slash
    NwSeCorner, // back-slash
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
                Self::NeSwCorner => '/',
                Self::NwSeCorner => '\\',
                Self::EastWest => '-',
                Self::NorthSouth => '|',
                Self::Intersection => '+',
            }
        )
    }
}
