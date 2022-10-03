use crate::Runner;

pub struct Aoc2016_01 {
    instr: Vec<Instruction>,
}

impl Aoc2016_01 {
    pub fn new() -> Self {
        Self { instr: Vec::new() }
    }
}

impl Runner for Aoc2016_01 {
    fn name(&self) -> (usize, usize) {
        (2016, 1)
    }

    fn parse(&mut self) {
        let line = &aoclib::read_lines("input/2016-01.txt")[0];
        for inst in line.split(", ") {
            let (dir, dist) = inst.split_at(1);
            let dist = dist.parse::<i32>().unwrap();
            match dir {
                "R" => self.instr.push(Instruction::Right(dist)),
                "L" => self.instr.push(Instruction::Left(dist)),
                _ => panic!("corrupted input file"),
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut dir = Direction::North;
        for i in &self.instr {
            let dist = match i {
                Instruction::Right(dist) => {
                    dir = dir.turn_right();
                    dist
                }
                Instruction::Left(dist) => {
                    dir = dir.turn_left();
                    dist
                }
            };

            match dir {
                Direction::North => y -= dist,
                Direction::South => y += dist,
                Direction::East => x += dist,
                Direction::West => x -= dist,
            }
        }
        crate::output(x.abs() + y.abs())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

// --------------------------------------------------------------------------------
#[derive(Debug)]
enum Instruction {
    Left(i32),
    Right(i32),
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
}
