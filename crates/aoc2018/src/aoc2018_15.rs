use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet, HashSet, VecDeque},
    fmt::Display,
};

use aoclib::Point;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_15 {
    map: HashSet<Point<usize>>,
    units: BTreeMap<Point<usize>, Unit>,
    rows: usize,
    cols: usize,
}

impl Aoc2018_15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn _dump(&self, units: &BTreeMap<Point<usize>, Unit>) {
        for row in 0..self.rows {
            let mut info = "    ".to_string();
            for col in 0..self.cols {
                let p = Point { x: row, y: col };
                if let Some(unit) = units.get(&p) {
                    print!("{unit}");
                    info.push_str(format!("{unit:?} ").as_str());
                } else if self.map.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("{info}");
        }
        println!("{:?}", units);
    }

    fn surround(
        &self,
        point: &Point<usize>,
        units: &BTreeMap<Point<usize>, Unit>,
    ) -> Vec<Point<usize>> {
        let look = vec![
            Point {
                x: point.x - 1, // up
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y - 1, // left
            },
            Point {
                x: point.x,
                y: point.y + 1, // right
            },
            Point {
                x: point.x + 1, // down
                y: point.y,
            },
        ];

        let mut result = Vec::new();
        for l in look {
            if self.map.contains(&l) || units.contains_key(&l) {
                continue;
            }
            result.push(l);
        }

        result
    }

    fn move_units(&mut self, units: &mut BTreeMap<Point<usize>, Unit>, amount: i64) -> bool {
        let mut unitlist = units.clone();

        while let Some(unit) = unitlist.pop_first() {
            let mut found_an_enemy = false;
            // println!("TURN: {unit:?}");
            // this checks to see if the unit that is about to move has been killed
            if units.get(&unit.0).is_none() {
                continue;
            }

            let mut dest = HashSet::new();
            let mut make_move = true;

            // this bit finds a move to make; if we're within range of an enemy already,
            // don't move anywhere
            for enemy in units.iter().filter(|u| !u.1.same_type(&unit.1)) {
                found_an_enemy = true;
                // println!("ENEMY {enemy:?}");
                if enemy.0.dist_to(&unit.0) == 1 {
                    make_move = false;
                    break;
                }
                for p in self.surround(enemy.0, units) {
                    dest.insert(p);
                }
            }

            if !found_an_enemy {
                return false;
            }

            // println!("{unit:?} found enemy");

            let mut attack_from = unit.0;

            if make_move {
                let unitmove = self.bfs(&unit.0, &dest, units);
                if let Some(dest) = unitmove {
                    let unittype = units.remove(&unit.0).unwrap();
                    // println!("Moving {unit:?} to {dest:?}");
                    units.insert(dest, unittype);
                    attack_from = dest;
                }
            }

            // this bit finds an enemy to attack
            let mut best: Option<(Point<usize>, Unit)> = None;
            // println!("Attack decision: {unit:?}");
            for enemy in units.iter().filter(|u| !u.1.same_type(&unit.1)) {
                // println!("  Considering: {enemy:?}");
                let dist = attack_from.dist_to(enemy.0);
                if dist == 1 {
                    if let Some(b) = best {
                        if b.1.hp() > enemy.1.hp() {
                            // println!("   choosing better: {enemy:?}");
                            best = Some((*enemy.0, *enemy.1));
                        }
                    } else {
                        // println!("   choosing: {enemy:?}");
                        best = Some((*enemy.0, *enemy.1));
                    }
                } else {
                    // println!("   enemy too far {dist}: {enemy:?}");
                }
            }

            if let Some(enemy) = best {
                // println!("unit {unit:?} attacks {enemy:?}");
                let eval = units.get_mut(&enemy.0).unwrap();
                eval.hit(amount);
                if eval.hp() < 0 {
                    unitlist.remove(&enemy.0);
                }
                units.retain(|_, unit| unit.hp() > 0); // TODO: fix
            } else {
                // println!("unit {unit:?} can't find anyone to hit");
            }
        }

        true
    }

    // - takes in a starting location and a list of possible destinations
    // - finds the closest destination (in reading order for tie-breaking)
    // - returns the move to make to go to that desination, if any
    fn bfs(
        &self,
        start: &Point<usize>,
        dest: &HashSet<Point<usize>>,
        units: &BTreeMap<Point<usize>, Unit>,
    ) -> Option<Point<usize>> {
        let mut steps = BTreeMap::new(); // (neighbor -> visited node)
        let mut queue = VecDeque::new();
        let mut min_depth = i64::MAX;
        let mut candidates = BTreeSet::new();

        queue.push_back((*start, 0));
        steps.insert(*start, None);

        while let Some((next, depth)) = queue.pop_front() {
            if depth > min_depth {
                continue;
            }
            if dest.contains(&next) {
                #[allow(clippy::comparison_chain)]
                if min_depth > depth {
                    min_depth = depth;
                    candidates.insert(next);
                } else if min_depth == depth {
                    candidates.insert(next);
                }
            }

            for neighbor in self.surround(&next, units) {
                if let Entry::Vacant(e) = steps.entry(neighbor) {
                    e.insert(Some(next));
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }
        if let Some(mut best) = candidates.first() {
            while let Some(Some(prev)) = steps.get(best) {
                if prev == start {
                    return Some(*best);
                }
                best = prev;
            }
            panic!("no path found");
        } else {
            None
        }
    }

    fn battle(&mut self, units: &mut BTreeMap<Point<usize>, Unit>, amount: i64) -> (i64, i64) {
        let mut rounds = 0;
        while self.move_units(units, amount) {
            rounds += 1;
            // println!("=========== ROUND {rounds} =============");
            // self._dump(&units);
        }
        // println!("{rounds} rounds");
        // println!("final board:");
        // self._dump();
        (rounds, units.iter().map(|u| u.1.hp()).sum::<i64>())
    }
}

impl Runner for Aoc2018_15 {
    fn name(&self) -> (usize, usize) {
        (2018, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-15.txt");
        // let lines = aoclib::read_lines("test-input");

        self.rows = lines.len();
        self.cols = lines[0].len();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        self.map.insert(Point { x: row, y: col });
                    }
                    'E' => {
                        self.units.insert(Point { x: row, y: col }, Unit::Elf(200));
                    }
                    'G' => {
                        self.units
                            .insert(Point { x: row, y: col }, Unit::Goblin(200));
                    }
                    '.' => {}
                    _ => panic!("invalid character {ch}"),
                }
            }
        }

        // self._dump();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut units = self.units.clone();
        let (rounds, total) = self.battle(&mut units, 3);
        aoclib::output(rounds * total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max = 4;
        let elfcount = self
            .units
            .iter()
            .filter(|(_, u)| matches!(u, Unit::Elf(_)))
            .count();

        loop {
            let mut units = self.units.clone();

            self.battle(&mut units, max);
            let newelfcount = units
                .iter()
                .filter(|(_, u)| matches!(u, Unit::Elf(_)))
                .count();
            if elfcount == newelfcount {
                break;
            }
            max *= 2;
        }

        max /= 2;
        loop {
            let mut units = self.units.clone();
            let (rounds, total) = self.battle(&mut units, max);
            let newelfcount = units
                .iter()
                .filter(|(_, u)| matches!(u, Unit::Elf(_)))
                .count();
            if newelfcount == elfcount {
                return aoclib::output(rounds * total);
            }
            max += 1;
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Unit {
    Elf(i64),
    Goblin(i64),
}

impl Unit {
    fn same_type(&self, other: &Unit) -> bool {
        matches!(
            (self, other),
            (Self::Elf(_), Self::Elf(_)) | (Self::Goblin(_), Self::Goblin(_))
        )
    }

    fn hp(&self) -> i64 {
        match self {
            Self::Elf(hp) => *hp,
            Self::Goblin(hp) => *hp,
        }
    }

    fn hit(&mut self, amount: i64) {
        match self {
            Self::Elf(hp) => *hp -= 3,
            Self::Goblin(hp) => *hp -= amount,
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Elf(_) => "E",
                Self::Goblin(_) => "G",
            }
        )
    }
}
