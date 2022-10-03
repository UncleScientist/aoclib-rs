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
            let (new_dir, dist) = i.aim(&dir);
            dir = new_dir;

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
        let mut visited = std::collections::HashSet::new();
        visited.insert((0i32, 0i32));

        let mut x = 0i32;
        let mut y = 0i32;
        let mut dir = Direction::North;

        'outer: for i in &self.instr {
            let (new_dir, dist) = i.aim(&dir);
            dir = new_dir;

            let (dx, dy) = match dir {
                Direction::North => (0, -1),
                Direction::South => (0, 1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            };

            for _ in 0..dist {
                x += dx;
                y += dy;
                if visited.contains(&(x, y)) {
                    break 'outer;
                }
                visited.insert((x, y));
            }
        }

        crate::output(x.abs() + y.abs())
    }
}

// --------------------------------------------------------------------------------
#[derive(Debug)]
enum Instruction {
    Left(i32),
    Right(i32),
}

impl Instruction {
    fn aim(&self, cur_dir: &Direction) -> (Direction, i32) {
        match self {
            Instruction::Left(n) => (cur_dir.turn_left(), *n),
            Instruction::Right(n) => (cur_dir.turn_right(), *n),
        }
    }
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
