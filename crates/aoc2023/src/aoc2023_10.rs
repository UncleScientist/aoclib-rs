use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_10 {
    grid: HashMap<(i64, i64), Tile>,
    start_pos: (i64, i64),
}

impl Aoc2023_10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get(&self, row: i64, col: i64) -> Tile {
        *self.grid.get(&(row, col)).unwrap_or_default()
    }
}

impl Runner for Aoc2023_10 {
    fn name(&self) -> (usize, usize) {
        (2023, 10)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-10.txt");
        let _lines = aoclib::read_lines("test-input.1");

        let mut startpos = None;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let row = row as i64;
                let col = col as i64;
                self.grid.insert(
                    (row, col),
                    match ch {
                        'S' => {
                            startpos = Some((row, col));
                            Tile::Nothing
                        }
                        '|' => Tile::NorthSouth,
                        '-' => Tile::EastWest,
                        'L' => Tile::NorthEast,
                        'J' => Tile::NorthWest,
                        '7' => Tile::SouthWest,
                        'F' => Tile::SouthEast,
                        _ => Tile::Nothing,
                    },
                );
            }
        }

        let Some(startpos) = startpos else {
            panic!("bug in your code");
        };

        self.start_pos = startpos;

        let north = self
            .get(startpos.0 - 1, startpos.1)
            .can_go(Direction::South);
        let south = self
            .get(startpos.0 + 1, startpos.1)
            .can_go(Direction::North);
        let east = self.get(startpos.0, startpos.1 + 1).can_go(Direction::West);
        let west = self.get(startpos.0, startpos.1 - 1).can_go(Direction::East);
        let start_tile = match (north, south, east, west) {
            (true, true, false, false) => Tile::NorthSouth,
            (true, false, true, false) => Tile::NorthEast,
            (true, false, false, true) => Tile::NorthWest,
            (false, true, true, false) => Tile::SouthEast,
            (false, true, false, true) => Tile::SouthWest,
            (false, false, true, true) => Tile::EastWest,
            _ => panic!("{north},{south},{east},{west} does not have exactly 2 directions"),
        };

        self.grid.insert(startpos, start_tile);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut curpos = self.start_pos;
        let mut curtile = self.get(curpos.0, curpos.1);
        let mut curdir = Direction::West;
        for dir in &DIRS {
            if curtile.can_go(*dir) {
                curdir = *dir;
                break;
            }
        }

        let delta = DELTA[curdir as usize];
        curpos = (curpos.0 + delta.0, curpos.1 + delta.1);
        curtile = self.get(curpos.0, curpos.1);
        curdir = curtile.next(curdir);

        let mut steps = 1;
        while curpos != self.start_pos {
            steps += 1;
            let delta = DELTA[curdir as usize];
            curpos = (curpos.0 + delta.0, curpos.1 + delta.1);
            curtile = self.get(curpos.0, curpos.1);
            curdir = curtile.next(curdir);
        }

        aoclib::output(steps / 2)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
enum Tile {
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    #[default]
    Nothing,
}

impl Tile {
    fn next(&self, dir: Direction) -> Direction {
        match (self, dir) {
            (Tile::NorthSouth, Direction::North) => Direction::North,
            (Tile::NorthSouth, Direction::South) => Direction::South,

            (Tile::EastWest, Direction::East) => Direction::East,
            (Tile::EastWest, Direction::West) => Direction::West,

            (Tile::NorthEast, Direction::South) => Direction::East,
            (Tile::NorthEast, Direction::West) => Direction::North,

            (Tile::NorthWest, Direction::South) => Direction::West,
            (Tile::NorthWest, Direction::East) => Direction::North,

            (Tile::SouthWest, Direction::North) => Direction::West,
            (Tile::SouthWest, Direction::East) => Direction::South,

            (Tile::SouthEast, Direction::North) => Direction::East,
            (Tile::SouthEast, Direction::West) => Direction::South,

            _ => panic!("going {dir:?} on tile type {self:?}"),
        }
    }

    fn can_go(&self, dir: Direction) -> bool {
        matches!(
            (self, dir),
            (Tile::NorthSouth, Direction::North | Direction::South)
                | (Tile::EastWest, Direction::East | Direction::West)
                | (Tile::NorthEast, Direction::North | Direction::East)
                | (Tile::NorthWest, Direction::North | Direction::West)
                | (Tile::SouthWest, Direction::South | Direction::West)
                | (Tile::SouthEast, Direction::South | Direction::East)
        )
    }
}

impl Default for &Tile {
    fn default() -> &'static Tile {
        &Tile::Nothing
    }
}

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

const DELTA: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
