use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2022_08 {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

impl Aoc2022_08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_08 {
    fn name(&self) -> (usize, usize) {
        (2022, 8)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-08.txt");
        self.width = lines[0].len() as i32;
        self.height = lines.len() as i32;

        for row in lines {
            self.grid.push(row.chars().collect::<Vec<char>>());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = HashSet::new();

        for (start, step, search) in [
            ((0, 0), RIGHT, DOWN),
            ((0, 0), DOWN, RIGHT),
            ((self.height - 1, self.width - 1), UP, LEFT),
            ((self.height - 1, self.width - 1), LEFT, UP),
        ] {
            let mut walk = start;

            while walk.0 >= 0 && walk.0 < self.height && walk.1 >= 0 && walk.1 < self.width {
                let (mut row, mut col) = walk;
                let mut tallest = self.grid[row as usize][col as usize];

                total.insert((row, col));

                while tallest < '9' {
                    row += search.0;
                    col += search.1;

                    if row < 0 || row >= self.height || col < 0 || col >= self.width {
                        break;
                    }

                    let tree = self.grid[row as usize][col as usize];
                    if tree > tallest {
                        total.insert((row, col));
                        tallest = tree;
                    }
                }

                walk.0 += step.0;
                walk.1 += step.1;
            }
        }
        aoclib::output(total.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max_score = 0;

        for row in 1..self.height - 1 {
            for col in 1..self.width - 1 {
                let mut score = 1;
                for step in [UP, DOWN, LEFT, RIGHT] {
                    let mut walk = (row, col);
                    let my_height = self.grid[row as usize][col as usize];
                    walk.0 += step.0;
                    walk.1 += step.1;
                    let mut count = 0;
                    while walk.0 >= 0 && walk.0 < self.height && walk.1 >= 0 && walk.1 < self.width
                    {
                        count += 1;

                        if self.grid[walk.0 as usize][walk.1 as usize] >= my_height {
                            break;
                        }

                        walk.0 += step.0;
                        walk.1 += step.1;
                    }

                    score *= count;
                }

                max_score = max_score.max(score);
            }
        }
        aoclib::output(max_score)
    }
}
