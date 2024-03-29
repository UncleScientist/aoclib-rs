use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos(i64, i64);

#[derive(Default)]
pub struct Aoc2023_23 {
    maze: HashMap<Pos, Tile>,
    width: i64,
    height: i64,
    start: Pos,
    end: Pos,
    verticies: HashMap<Pos, HashSet<(Pos, i64)>>, // (row, col), dist
}

impl Aoc2023_23 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_23 {
    fn name(&self) -> (usize, usize) {
        (2023, 23)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-23.txt");
        self.height = lines.len() as i64;
        self.width = lines[0].len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let row = row as i64;
                let col = col as i64;
                match ch {
                    '#' => {}
                    '.' => {
                        self.maze.insert(Pos(row, col), Tile::Space);
                    }
                    '>' => {
                        self.maze.insert(Pos(row, col), Tile::Slope(Pos(0, 1)));
                    }
                    'v' => {
                        self.maze.insert(Pos(row, col), Tile::Slope(Pos(1, 0)));
                    }
                    _ => panic!("invalid character"),
                }
            }
        }

        for col in 0..self.width {
            if self.maze.contains_key(&Pos(0, col)) {
                self.start = Pos(0, col);
            } else if self.maze.contains_key(&Pos(self.height - 1, col)) {
                self.end = Pos(self.height - 1, col);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        // last intersection, overall distance to last intersection, curpos, curdir, curdist
        let mut stack = vec![(self.start, 0, self.start, Pos(1, 0), 0)];
        let mut max_dist = 0;

        while let Some((lastpos, lastdist, curpos, curdir, dist)) = stack.pop() {
            if curpos == self.end {
                max_dist = max_dist.max(dist);
                self.verticies
                    .entry(lastpos)
                    .or_default()
                    .insert((self.end, dist - lastdist));
                continue;
            }

            let mut slopecount = 0;
            let mut nextstack = Vec::new();

            for dir in &DIRS {
                if Pos(-dir.0, -dir.1) == curdir {
                    continue;
                }
                let nextpos = Pos(curpos.0 + dir.0, curpos.1 + dir.1);
                if let Some(tile) = self.maze.get(&nextpos) {
                    match tile {
                        Tile::Space => {
                            nextstack.push((nextpos, *dir, dist + 1));
                        }
                        Tile::Slope(slope) => {
                            slopecount += 1;
                            if slope == dir {
                                nextstack.push((nextpos, *dir, dist + 1));
                            }
                        }
                    }
                }
            }

            let mut origin = lastpos;
            let mut odist = lastdist;
            if slopecount > 1 {
                // how we got here
                self.verticies
                    .entry(curpos)
                    .or_default()
                    .insert((lastpos, dist - lastdist));

                // how we get back
                self.verticies
                    .entry(lastpos)
                    .or_default()
                    .insert((curpos, dist - lastdist));

                origin = curpos;
                odist = dist;
            }

            for n in nextstack {
                stack.push((origin, odist, n.0, n.1, n.2));
            }
        }

        aoclib::output(max_dist)
    }

    fn part2(&mut self) -> Vec<String> {
        // test input: start -> first = 15
        //             first -> lower one = 22
        //             first -> righter one = 22

        let mut stack = vec![(self.start, 0, HashSet::new())];
        let mut max_dist = 0;

        while let Some((curpos, dist, mut visited)) = stack.pop() {
            if self.end == curpos {
                max_dist = max_dist.max(dist);
                continue;
            }
            if !visited.insert(curpos) {
                continue;
            }
            for (nextpos, nextdist) in self.verticies.get(&curpos).unwrap().iter() {
                let v = visited.clone();
                stack.push((*nextpos, nextdist + dist, v));
            }
        }

        aoclib::output(max_dist)
    }
}

const DIRS: [Pos; 4] = [Pos(-1, 0), Pos(1, 0), Pos(0, -1), Pos(0, 1)];

#[derive(Debug)]
enum Tile {
    Space,
    Slope(Pos),
}
