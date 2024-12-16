use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_15 {
    robot: (i64, i64),
    srobot: (i64, i64),
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
                    self.warehouse
                        .wide
                        .insert((row as i64, (col * 2) as i64), Item::Wall);
                    self.warehouse
                        .wide
                        .insert((row as i64, (col * 2) as i64 + 1), Item::Wall);
                } else if ch == 'O' {
                    self.warehouse
                        .floor
                        .insert((row as i64, col as i64), Item::Box);
                    self.warehouse
                        .wide
                        .insert((row as i64, (col * 2) as i64), Item::BoxLeft);
                    self.warehouse
                        .wide
                        .insert((row as i64, (col * 2) as i64 + 1), Item::BoxRight);
                } else if ch == '@' {
                    self.robot = (row as i64, col as i64);
                    self.srobot = (row as i64, 2 * col as i64);
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
        let mut warehouse = self.warehouse.clone();
        let mut robot = self.robot;

        for m in &self.moves {
            let newpos = (robot.0 + m.0, robot.1 + m.1);
            if warehouse.can_move(&newpos, m) {
                robot = newpos;
            }
        }

        aoclib::output(warehouse.score())
    }

    fn part2(&mut self) -> Vec<String> {
        // let mut warehouse = self.warehouse.clone();
        // let mut robot = self.robot;

        self.warehouse.robot = self.srobot;
        // println!("{}", self.warehouse);
        for m in &self.moves {
            let newpos = (self.warehouse.robot.0 + m.0, self.warehouse.robot.1 + m.1);
            if self.warehouse.can_double_move(&newpos, m) {
                self.warehouse.robot = newpos;
                // println!("Robot now at {newpos:?}");
                // println!("{}", self.warehouse);
            }
        }

        aoclib::output(self.warehouse.double_score())
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

#[derive(Debug, Eq, PartialEq, Clone)]
enum Item {
    Box,
    Wall,
    BoxLeft,
    BoxRight,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Box => write!(f, "O"),
            Item::BoxLeft => write!(f, "["),
            Item::BoxRight => write!(f, "]"),
            Item::Wall => write!(f, "#"),
        }
    }
}

#[derive(Default, Clone)]
struct Warehouse {
    rows: i64,
    cols: i64,
    floor: HashMap<(i64, i64), Item>,
    wide: HashMap<(i64, i64), Item>,
    robot: (i64, i64),
}

impl Warehouse {
    fn can_move(&mut self, pos: &(i64, i64), dir: &(i64, i64)) -> bool {
        match self.floor.get(pos) {
            None => true,
            Some(Item::Wall) => false,
            Some(Item::Box) => {
                let nextpos = (pos.0 + dir.0, pos.1 + dir.1);
                if self.can_move(&nextpos, dir) {
                    self.floor.remove(pos);
                    self.floor.insert(nextpos, Item::Box);
                    true
                } else {
                    false
                }
            }
            _ => panic!("wrong function for moves"),
        }
    }

    fn score(&self) -> i64 {
        self.floor
            .iter()
            .filter_map(|(pos, item)| {
                if matches!(item, Item::Box) {
                    Some(pos.0 * 100 + pos.1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn double_score(&self) -> i64 {
        self.wide
            .iter()
            .filter_map(|(pos, item)| {
                if matches!(item, Item::BoxLeft) {
                    Some(pos.0 * 100 + pos.1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn can_double_move(&mut self, pos: &(i64, i64), dir: &(i64, i64)) -> bool {
        if dir.0 == 0 {
            let result = match self.wide.get(pos) {
                None => true,
                Some(Item::Wall) => false,
                Some(Item::BoxLeft | Item::BoxRight) => {
                    let nextpos = (pos.0 + dir.0, pos.1 + dir.1);
                    if self.can_double_move(&nextpos, dir) {
                        // println!("moving {:?} to {nextpos:?}", self.wide.get(pos));
                        let i = self.wide.remove(pos).unwrap();
                        self.wide.insert(nextpos, i);
                        true
                    } else {
                        false
                    }
                }
                _ => panic!("wrong function for moves"),
            };
            // println!("Move {dir:?}");
            // println!("{self}");
            result
        } else {
            // println!("==== Moving {dir:?}");
            let (allowed, mut movelist) = match self.wide.get(&pos) {
                None => return true, // the robot just wants to move up/down
                Some(item) => match item {
                    Item::Wall => return false, // don't move the robot into a wall
                    Item::Box => unreachable!(),
                    Item::BoxLeft => self.get_move_list(&pos, dir),
                    Item::BoxRight => self.get_move_list(&(pos.0, pos.1 - 1), dir),
                },
            };
            // println!("movelist: {movelist:?}");

            if allowed {
                while !movelist.is_empty() {
                    let moveset = movelist.drain(..).collect::<HashSet<(i64, i64)>>();
                    // println!("set = {moveset:?}");
                    for m in moveset {
                        let p1 = (m.0 + dir.0, m.1 + dir.1);
                        let p2 = (m.0 + dir.0, m.1 + dir.1 + 1);
                        if self.wide.contains_key(&p1) || self.wide.contains_key(&p2) {
                            movelist.push(m);
                        } else {
                            assert_eq!(self.wide.get(&m), Some(&Item::BoxLeft));
                            self.wide.remove(&m);
                            self.wide.remove(&(m.0, m.1 + 1));
                            self.wide.insert(p1, Item::BoxLeft);
                            self.wide.insert(p2, Item::BoxRight);
                        }
                    }
                }
            }
            allowed
        }
    }

    // return a list of all the box positions that need to move
    fn get_move_list(&self, pos: &(i64, i64), dir: &(i64, i64)) -> (bool, Vec<(i64, i64)>) {
        let mut mpos = *pos;
        if Some(&Item::BoxRight) == self.wide.get(&mpos) {
            mpos = (mpos.0, mpos.1 - 1);
        }
        let newpos = (mpos.0 + dir.0, mpos.1 + dir.1);
        match (
            self.wide.get(&newpos),
            self.wide.get(&(newpos.0, newpos.1 + 1)),
        ) {
            (None, None) => (true, vec![mpos]),
            (_, Some(Item::Wall)) => (false, vec![]),
            (Some(Item::Wall), _) => (false, vec![]),
            (left, right) => {
                let mut movelist = vec![mpos];
                let (motion, v) = match left {
                    None => (true, vec![]),
                    Some(Item::BoxRight) => self.get_move_list(&(newpos.0, newpos.1 - 1), dir),
                    Some(Item::BoxLeft) => self.get_move_list(&newpos, dir),
                    _ => unreachable!(),
                };
                if !motion {
                    return (false, v);
                }
                movelist.extend(v);
                let (motion, v) = match right {
                    None => (true, vec![]),
                    Some(Item::BoxRight) => (true, vec![]),
                    Some(Item::BoxLeft) => self.get_move_list(&(newpos.0, newpos.1 + 1), dir),
                    _ => unreachable!(),
                };
                if !motion {
                    return (false, v);
                }
                movelist.extend(v);
                (true, movelist)
            }
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols * 2 {
                if self.robot == (row, col) {
                    write!(f, "@")?;
                } else {
                    match self.wide.get(&(row, col)) {
                        None => write!(f, ".")?,
                        Some(item) => write!(f, "{item}")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
