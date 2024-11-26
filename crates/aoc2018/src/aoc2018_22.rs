use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2018_22 {
    cave: Cave,
}

impl Aoc2018_22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_22 {
    fn name(&self) -> (usize, usize) {
        (2018, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-22.txt");
        let depth = lines[0].strip_prefix("depth: ").unwrap();
        let target = lines[1].strip_prefix("target: ").unwrap();

        let depth = depth.parse::<i64>().unwrap();
        let (x, y) = target.split_once(',').unwrap();
        let target = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        self.cave = Cave::new(depth, target);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.cave.get_risk_level())
    }

    fn part2(&mut self) -> Vec<String> {
        let state = State::default();
        let mut work = BTreeSet::<(i64, State)>::new();
        work.insert((0, state));

        let mut seen = HashSet::new();

        while let Some((time, next)) = work.pop_first() {
            if seen.insert(next) {
                if next.loc == self.cave.target && next.inv == Tool::Torch {
                    return aoclib::output(time);
                }
                for (m, cost) in self.cave.moves(&next) {
                    work.insert((time + cost, m));
                }
            }
        }

        aoclib::output("unsolved")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl RegionType {
    fn risk(&self) -> i64 {
        match self {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }

    fn supports(&self, inv: &Tool) -> bool {
        match self {
            RegionType::Rocky => *inv == Tool::Torch || *inv == Tool::ClimbingGear,
            RegionType::Wet => *inv == Tool::Neither || *inv == Tool::ClimbingGear,
            RegionType::Narrow => *inv == Tool::Torch || *inv == Tool::Neither,
        }
    }
}

impl Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegionType::Rocky => write!(f, "."),
            RegionType::Wet => write!(f, "="),
            RegionType::Narrow => write!(f, "|"),
        }
    }
}

#[derive(Debug, Default)]
struct Cave {
    depth: i64,
    target: (i64, i64),
    map: HashMap<(i64, i64), i64>, // location -> erosion_level
}

impl Cave {
    fn new(depth: i64, target: (i64, i64)) -> Self {
        Self {
            depth,
            target,
            ..Self::default()
        }
    }

    fn get_risk_level(&mut self) -> i64 {
        (0..=self.target.1)
            .map(|y| (0..=self.target.0).fold(0, |sum, x| sum + self.get_type(&(x, y)).risk()))
            .sum::<i64>()
    }

    fn get_type(&mut self, loc: &(i64, i64)) -> RegionType {
        match self.get_erosion_level(loc) % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => panic!("compiler error"),
        }
    }

    fn get_erosion_level(&mut self, loc: &(i64, i64)) -> i64 {
        if let Some(el) = self.map.get(loc) {
            return *el;
        }
        let index = (self.depth + self.get_geologic_index(loc)) % 20183;
        self.map.insert(*loc, index);
        index
    }

    fn get_geologic_index(&mut self, loc: &(i64, i64)) -> i64 {
        match loc {
            (0, 0) => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => {
                if *loc == self.target {
                    0
                } else {
                    self.get_erosion_level(&(x - 1, *y)) * self.get_erosion_level(&(*x, y - 1))
                }
            }
        }
    }

    fn moves(&mut self, state: &State) -> Vec<(State, i64)> {
        let mut movelist = Vec::new();
        let curtype = self.get_type(&state.loc);
        let alttool = state.inv.get_alt(&curtype);
        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let newloc = (state.loc.0 + dir.0, state.loc.1 + dir.1);
            if newloc.0 < 0
                || newloc.1 < 0
                || newloc.0 > self.target.0 + 20
                || newloc.1 > self.target.1 + 20
            {
                continue;
            }

            let newtype = self.get_type(&newloc);

            if newtype.supports(&state.inv) {
                movelist.push((
                    State {
                        loc: newloc,
                        inv: state.inv,
                    },
                    1,
                ));
            }
            if newtype.supports(&alttool) {
                movelist.push((
                    State {
                        loc: newloc,
                        inv: alttool,
                    },
                    8,
                ));
            }
        }

        movelist
    }

    fn _print(&mut self) {
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                print!("{}", self.get_type(&(x, y)));
            }
            println!();
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    loc: (i64, i64),
    inv: Tool,
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Tool {
    #[default]
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn get_alt(&self, curtype: &RegionType) -> Tool {
        match self {
            Tool::Torch => {
                if *curtype == RegionType::Rocky {
                    Tool::ClimbingGear
                } else {
                    Tool::Neither
                }
            }
            Tool::ClimbingGear => {
                if *curtype == RegionType::Rocky {
                    Tool::Torch
                } else {
                    Tool::Neither
                }
            }
            Tool::Neither => {
                if *curtype == RegionType::Wet {
                    Tool::ClimbingGear
                } else {
                    Tool::Torch
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_geologic_index() {
        let mut cave = Cave::new(510, (10, 10));
        assert_eq!(0, cave.get_geologic_index(&(0, 0)));
        assert_eq!(0, cave.get_geologic_index(&(10, 10)));
        assert_eq!(16807, cave.get_geologic_index(&(1, 0)));
        assert_eq!(48271, cave.get_geologic_index(&(0, 1)));
        assert_eq!(145722555, cave.get_geologic_index(&(1, 1)));
    }

    #[test]
    fn test_part1() {
        let mut cave = Cave::new(510, (10, 10));
        cave._print();
        println!("{:?}", cave.map);
        assert_eq!(114, cave.get_risk_level());
    }

    #[test]
    fn test_switch_gear() {
        let mut cave = Cave::new(510, (10, 10));
        let state = State {
            loc: (0, 0),
            inv: Tool::Torch,
        };
        let moves = cave.moves(&state);
        assert_eq!(3, moves.len());
        for m in &moves {
            if m.0.inv == Tool::ClimbingGear {
                assert_eq!(8, m.1);
            } else {
                assert_eq!(Tool::Torch, m.0.inv);
                assert_eq!(1, m.1);
            }
        }
    }

    #[test]
    fn test_default_state() {
        assert_eq!(
            State {
                loc: (0, 0),
                inv: Tool::Torch
            },
            State::default()
        );
    }
}
