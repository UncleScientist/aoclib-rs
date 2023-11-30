use aoclib::{read_lines, Point};

use aoclib::Runner;

#[derive(Debug)]
enum Command {
    Off(Point<usize>, Point<usize>),
    On(Point<usize>, Point<usize>),
    Toggle(Point<usize>, Point<usize>),
}

pub struct Aoc2015_06 {
    commands: Vec<Command>,
}

impl Aoc2015_06 {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

impl Runner for Aoc2015_06 {
    fn parse(&mut self) {
        for line in read_lines("input/2015-06.txt") {
            let s = line.split(' ').collect::<Vec<&str>>();
            match s[1] {
                "off" => {
                    let start = Point::parse(s[2], ",");
                    let end = Point::parse(s[4], ",");
                    self.commands.push(Command::Off(start, end));
                }
                "on" => {
                    let start = Point::parse(s[2], ",");
                    let end = Point::parse(s[4], ",");
                    self.commands.push(Command::On(start, end));
                }
                _ => {
                    let start = Point::parse(s[1], ",");
                    let end = Point::parse(s[3], ",");
                    self.commands.push(Command::Toggle(start, end));
                }
            }
        }
    }
    fn name(&self) -> (usize, usize) {
        (2015, 6)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = [[false; 1000]; 1000];

        for c in &self.commands {
            match c {
                Command::Off(p1, p2) => {
                    for gx in grid.iter_mut().take(p2.x + 1).skip(p1.x) {
                        for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y) {
                            *gy = false;
                        }
                    }
                }
                Command::On(p1, p2) => {
                    for gx in grid.iter_mut().take(p2.x + 1).skip(p1.x) {
                        for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y) {
                            *gy = true;
                        }
                    }
                }
                Command::Toggle(p1, p2) => {
                    for gx in grid.iter_mut().take(p2.x + 1).skip(p1.x) {
                        for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y) {
                            *gy = !*gy;
                        }
                    }
                }
            }
        }

        let count = grid
            .iter()
            .fold(0, |a, b| a + b.iter().map(|x| *x as u32).sum::<u32>());

        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grid = [[0u32; 1000]; 1000];

        for c in &self.commands {
            match c {
                Command::Off(p1, p2) => {
                    for gx in grid.iter_mut().take(p2.x + 1).skip(p1.x) {
                        for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y) {
                            *gy = gy.saturating_sub(1);
                        }
                    }
                }
                Command::On(p1, p2) => {
                    for gx in grid.iter_mut().take(p2.x + 1).skip(p1.x) {
                        for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y) {
                            *gy += 1;
                        }
                    }
                }
                Command::Toggle(p1, p2) => {
                    for gx in grid.iter_mut().take(p2.x + 1).skip(p1.x) {
                        for gy in gx.iter_mut().take(p2.y + 1).skip(p1.y) {
                            *gy += 2;
                        }
                    }
                }
            }
        }

        let count = grid.iter().fold(0, |a, b| a + b.iter().sum::<u32>());

        aoclib::output(count)
    }
}
