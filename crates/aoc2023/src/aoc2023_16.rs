use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_16 {
    contraption: Vec<Vec<Mirror>>,
    width: isize,
    height: isize,
}

impl Aoc2023_16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn next_pos(&self, pos: Pos, dir: usize) -> Option<Pos> {
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // up, down, left, right
        let new_pos = Pos(pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);

        if new_pos.0 < 0 || new_pos.0 >= self.height || new_pos.1 < 0 || new_pos.1 >= self.width {
            None
        } else {
            Some(new_pos)
        }
    }

    fn count_energized(&self, pos: Pos, dir: usize) -> usize {
        let mut stack = vec![(pos, dir)];
        let mut visited = HashSet::new();

        while let Some((pos, dir)) = stack.pop() {
            if !visited.insert((pos.clone(), dir)) {
                continue;
            }
            match self.contraption[pos.0 as usize][pos.1 as usize] {
                Mirror::SlashMirror => {
                    let new_dir = 3 - dir;
                    if let Some(new_pos) = self.next_pos(pos, new_dir) {
                        stack.push((new_pos, new_dir));
                    }
                }
                Mirror::BackslashMirror => {
                    let new_dir = (dir + 2) % 4;
                    if let Some(new_pos) = self.next_pos(pos, new_dir) {
                        stack.push((new_pos, new_dir));
                    }
                }
                Mirror::HorizSplitter => {
                    if dir == 0 || dir == 1 {
                        for dir in 2..=3 {
                            if let Some(new_pos) = self.next_pos(pos.clone(), dir) {
                                stack.push((new_pos, dir));
                            }
                        }
                    } else {
                        if let Some(new_pos) = self.next_pos(pos, dir) {
                            stack.push((new_pos, dir));
                        }
                    }
                }
                Mirror::VertSplitter => {
                    if dir == 2 || dir == 3 {
                        for dir in 0..=1 {
                            if let Some(new_pos) = self.next_pos(pos, dir) {
                                stack.push((new_pos, dir));
                            }
                        }
                    } else {
                        if let Some(new_pos) = self.next_pos(pos, dir) {
                            stack.push((new_pos, dir));
                        }
                    }
                }
                Mirror::EmptySpace => {
                    if let Some(new_pos) = self.next_pos(pos, dir) {
                        stack.push((new_pos, dir));
                    }
                }
            }
        }

        visited
            .iter()
            .map(|(pos, _dir)| pos)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl Runner for Aoc2023_16 {
    fn name(&self) -> (usize, usize) {
        (2023, 16)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-16.txt");

        for line in lines {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(match ch {
                    '/' => Mirror::SlashMirror,
                    '\\' => Mirror::BackslashMirror,
                    '-' => Mirror::HorizSplitter,
                    '|' => Mirror::VertSplitter,
                    '.' => Mirror::EmptySpace,
                    _ => panic!("invalid char {ch}"),
                });
            }
            self.contraption.push(row);
        }

        self.height = self.contraption.len() as isize;
        self.width = self.contraption[0].len() as isize;
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.count_energized(Pos(0, 0), 3))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max = 0;
        for col in 0..self.width {
            max = max.max(self.count_energized(Pos(0, col), 1));
            max = max.max(self.count_energized(Pos(self.height - 1, col), 0));
        }
        for row in 0..self.height {
            max = max.max(self.count_energized(Pos(row, 0), 3));
            max = max.max(self.count_energized(Pos(row, self.width - 1), 2));
        }
        aoclib::output(max)
    }
}

#[derive(Debug)]
enum Mirror {
    SlashMirror,
    BackslashMirror,
    HorizSplitter,
    VertSplitter,
    EmptySpace,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos(isize, isize);
