use std::collections::{HashMap, HashSet, VecDeque};

use aoclib::{Position64, Runner, Size64, DIRS};

#[derive(Default)]
pub struct Aoc2024_20 {
    size: Size64,
    start: Position64,
    end: Position64,
    maze: HashSet<Position64>,
    path: VecDeque<Position64>,
}

impl Aoc2024_20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn ways_to_cheat_with(&self, picosec: usize) -> usize {
        // ################
        // #.......abcdefg#
        // ##############h#
        // #.......qpo#kji#
        // ##########nml###
        // ################
        let mut total = 0;
        for i in 0..self.path.len() - 3 {
            for j in i + 3..self.path.len() {
                // compute manhattan dist between i & j
                // if it's less than 3? then we have a potential shortcut
                // if (j-i) > dist, add one to that shortend distance
                let dist = self.path[i].distance_to(&self.path[j]) as usize;
                if dist <= picosec && (j - i) > dist {
                    total += ((j - i) - dist >= 100) as usize;
                }
            }
        }

        total
    }

    fn _display(&self) {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let pos = Position64(row, col);
                if pos == self.start {
                    print!("S");
                } else if pos == self.end {
                    print!("E");
                } else if self.maze.contains(&pos) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

impl Runner for Aoc2024_20 {
    fn name(&self) -> (usize, usize) {
        (2024, 20)
    }

    fn parse(&mut self) {
        let _lines = aoclib::read_lines("test20-1.txt");
        let lines = aoclib::read_lines("input/2024-20.txt");

        self.size = Size64(lines.len() as i64, lines[0].len() as i64);

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = Position64(row as i64, col as i64);
                if ch == 'S' {
                    self.start = pos;
                } else if ch == 'E' {
                    self.end = pos;
                }
                if ch != '#' {
                    self.maze.insert(pos);
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut queue = VecDeque::from([(0, self.start)]);
        let mut visited = HashSet::new();

        let mut dist = HashMap::<Position64, usize>::new();
        let mut backlink = HashMap::<Position64, Position64>::new();

        while let Some((cost, pos)) = queue.pop_front() {
            if pos == self.end {
                break;
            }
            if visited.insert(pos) {
                for dir in &DIRS {
                    let newpos = pos + *dir;
                    let cost = cost + 1;
                    if !self.maze.contains(&newpos) {
                        continue;
                    }

                    if !visited.contains(&newpos) {
                        let entry = dist.entry(pos).or_insert(usize::MAX);
                        if cost < *entry {
                            *entry = cost;
                            backlink.insert(newpos, pos);
                        }
                    }
                    queue.push_back((cost, newpos));
                }
            }
        }

        self.path.push_back(self.end);
        let mut cur = self.end;
        while let Some(next) = backlink.get(&cur) {
            self.path.push_front(*next);
            cur = *next;
        }

        aoclib::output(self.ways_to_cheat_with(2))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.ways_to_cheat_with(20))
    }
}
