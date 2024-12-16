use std::{collections::HashMap, fmt::Display};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_15 {
    robot: (i64, i64),
    warehouse: Warehouse,
    moves: Vec<(i64, i64)>,
}

impl Aoc2024_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_15 {
    fn name(&self) -> (usize, usize) {
        (2024, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-15.txt");
        let _lines = aoclib::read_lines("test15-1.txt");
        let mut row = 0;

        while lines[row].starts_with('#') {
            for (col, ch) in lines[row].chars().enumerate() {
                if ch == '#' {
                    self.warehouse
                        .floor
                        .insert((row as i64, col as i64), Item::Wall);
                } else if ch == 'O' {
                    self.warehouse
                        .floor
                        .insert((row as i64, col as i64), Item::Box);
                } else if ch == '@' {
                    self.robot = (row as i64, col as i64);
                }
            }
            row += 1;
        }

        self.moves = lines[row..]
            .iter()
            .flat_map(|line| line.chars().map(char_to_direction).collect::<Vec<_>>())
            .collect();

        self.warehouse.rows = row as i64;
        self.warehouse.cols = lines[0].len() as i64;
    }

    fn part1(&mut self) -> Vec<String> {
        for m in &self.moves {
            let newpos = (self.robot.0 + m.0, self.robot.1 + m.1);
            if self.warehouse.can_move(&newpos, m) {
                self.robot = newpos;
            }
        }

        aoclib::output(self.warehouse.score())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

fn char_to_direction(ch: char) -> (i64, i64) {
    match ch {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("invalid move {ch}"),
    }
}

#[derive(Eq, PartialEq)]
enum Item {
    Box,
    Wall,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Box => write!(f, "O"),
            Item::Wall => write!(f, "#"),
        }
    }
}

#[derive(Default)]
struct Warehouse {
    rows: i64,
    cols: i64,
    floor: HashMap<(i64, i64), Item>,
}

impl Warehouse {
    fn can_move(&mut self, pos: &(i64, i64), dir: &(i64, i64)) -> bool {
        match self.floor.get(pos) {
            None => true,
            Some(Item::Wall) => false,
            Some(Item::Box) => {
                let nextpos = (pos.0 + dir.0, pos.1 + dir.1);
                if self.can_move(&nextpos, &dir) {
                    self.floor.remove(pos);
                    self.floor.insert(nextpos, Item::Box);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn score(&self) -> i64 {
        self.floor
            .iter()
            .filter_map(|(pos, item)| {
                if *item == Item::Box {
                    Some(pos.0 * 100 + pos.1)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                match self.floor.get(&(row, col)) {
                    None => write!(f, ".")?,
                    Some(item) => write!(f, "{item}")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
