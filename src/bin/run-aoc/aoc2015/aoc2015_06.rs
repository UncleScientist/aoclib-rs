use aoclib::{read_lines, Point};

use crate::Runner;

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
        let mut commands = Vec::new();
        for line in read_lines("input/2015-06.txt") {
            let s = line.split(' ').collect::<Vec<&str>>();
            match s[1] {
                "off" => {
                    let start = Point::parse(s[2]);
                    let end = Point::parse(s[4]);
                    commands.push(Command::Off(start, end));
                }
                "on" => {
                    let start = Point::parse(s[2]);
                    let end = Point::parse(s[4]);
                    commands.push(Command::On(start, end));
                }
                _ => {
                    let start = Point::parse(s[1]);
                    let end = Point::parse(s[3]);
                    commands.push(Command::Toggle(start, end));
                }
            }
        }

        Self { commands }
    }
}

impl Runner for Aoc2015_06 {
    fn name(&self) -> (usize, usize) {
        (2015, 6)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = [[false; 1000]; 1000];

        for c in &self.commands {
            match c {
                Command::Off(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            grid[x][y] = false;
                        }
                    }
                }
                Command::On(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            grid[x][y] = true;
                        }
                    }
                }
                Command::Toggle(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            grid[x][y] = !grid[x][y];
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                if grid[i][j] {
                    count += 1;
                }
            }
        }

        vec![format!("{count}")]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grid = [[0u32; 1000]; 1000];

        for c in &self.commands {
            match c {
                Command::Off(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            grid[x][y] = grid[x][y].saturating_sub(1);
                        }
                    }
                }
                Command::On(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            grid[x][y] += 1;
                        }
                    }
                }
                Command::Toggle(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            grid[x][y] += 2;
                        }
                    }
                }
            }
        }

        let count1 = grid.iter().fold(0, |a, b| a + b.iter().sum::<u32>());
        let mut count = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                count += grid[i][j];
            }
        }
        println!("{count1} vs {count}");

        vec![format!("{count}")]
    }
}
