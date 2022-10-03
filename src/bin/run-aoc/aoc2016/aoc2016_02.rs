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

        /*
        let mylines = vec![
            "ULL".to_string().chars().collect::<Vec<char>>(),
            "RRDDD".to_string().chars().collect::<Vec<char>>(),
            "LURDL".to_string().chars().collect::<Vec<char>>(),
            "UUUUD".to_string().chars().collect::<Vec<char>>(),
        ];
        */

        for key in &self.lines {
            for step in key.iter() {
                match step {
                    'U' => {
                        if y > 0 {
                            y -= 1
                        }
                    }
                    'D' => {
                        if y < 2 {
                            y += 1
                        }
                    }
                    'L' => {
                        if x > 0 {
                            x -= 1
                        }
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
        crate::output("unsolved")
    }
}
