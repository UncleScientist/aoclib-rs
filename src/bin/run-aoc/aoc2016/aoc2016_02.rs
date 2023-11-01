use crate::Runner;

pub struct Aoc2016_02 {
    lines: Vec<Vec<char>>,
}

impl Aoc2016_02 {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Runner for Aoc2016_02 {
    fn name(&self) -> (usize, usize) {
        (2016, 2)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2016-02.txt") {
            let chars = line.chars().collect::<Vec<char>>();
            self.lines.push(chars);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut x = 1usize;
        let mut y = 1usize;
        let keypad = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut answer = String::from("");

        for key in &self.lines {
            for step in key.iter() {
                match step {
                    'U' => {
                        y = y.saturating_sub(1);
                    }
                    'D' => {
                        if y < 2 {
                            y += 1
                        }
                    }
                    'L' => {
                        x = x.saturating_sub(1);
                    }
                    'R' => {
                        if x < 2 {
                            x += 1
                        }
                    }
                    _ => panic!("corrupted input file"),
                }
            }
            answer.push(keypad[y * 3 + x]);
        }

        crate::output(answer)
    }

    fn part2(&mut self) -> Vec<String> {
        let movement = vec![
            (0, 0, 0, 0), // (0, 0)
            (0, 0, 0, 0), // (0, 1)
            (0, 1, 0, 0), // (0, 2)
            (0, 0, 0, 0), // (0, 3)
            (0, 0, 0, 0), // (0, 4)
            (0, 0, 0, 0), // (1, 0)
            (0, 1, 0, 1), // (1, 1)
            (1, 1, 1, 1), // (1, 2)
            (0, 1, 1, 0), // (1, 3)
            (0, 0, 0, 0), // (1, 4)
            (0, 0, 0, 1), // (2, 0)
            (1, 1, 1, 1), // (2, 1)
            (1, 1, 1, 1), // (2, 2)
            (1, 1, 1, 1), // (2, 3)
            (0, 0, 1, 0), // (2, 4)
            (0, 0, 0, 0), // (3, 0)
            (1, 0, 0, 1), // (3, 1)
            (1, 1, 1, 1), // (3, 2)
            (1, 0, 1, 0), // (3, 3)
            (0, 0, 0, 0), // (3, 4)
            (0, 0, 0, 0), // (4, 0)
            (0, 0, 0, 0), // (4, 1)
            (1, 0, 0, 0), // (4, 2)
            (0, 0, 0, 0), // (4, 3)
            (0, 0, 0, 0), // (4, 4)
        ];

        let keypad = [
            [' ', ' ', '1', ' ', ' '],
            [' ', '2', '3', '4', ' '],
            ['5', '6', '7', '8', '9'],
            [' ', 'A', 'B', 'C', ' '],
            [' ', ' ', 'D', ' ', ' '],
        ];

        let mut x = 0usize;
        let mut y = 2usize;

        let mut answer = String::from("");
        for key in &self.lines {
            for step in key.iter() {
                let pos = y * 5 + x;
                match step {
                    'U' => y -= movement[pos].0,
                    'D' => y += movement[pos].1,
                    'L' => x -= movement[pos].2,
                    'R' => x += movement[pos].3,
                    _ => panic!("corrupted input file"),
                }
            }
            answer.push(keypad[y][x]);
        }

        crate::output(answer)
    }
}
