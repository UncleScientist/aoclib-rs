use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_09 {
    stream: Vec<char>,
    garbage_count: usize,
}

impl Aoc2017_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_09 {
    fn name(&self) -> (usize, usize) {
        (2017, 9)
    }

    fn parse(&mut self) {
        self.stream = aoclib::read_to_chars("input/2017-09.txt");
        if *self.stream.last().unwrap() == '\n' {
            self.stream.pop();
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut state = State::Normal;
        let mut prev_state = State::Normal;
        let mut depth = 0;
        let mut score = 0;

        for &ch in &self.stream {
            match state {
                State::Normal => {
                    if ch == '!' {
                        state = State::Skip;
                        prev_state = State::Normal;
                    } else if ch == '<' {
                        state = State::Garbage;
                    } else if ch == '{' {
                        depth += 1;
                    } else if ch == '}' {
                        score += depth;
                        depth -= 1;
                    }
                }
                State::Skip => state = prev_state,
                State::Garbage => {
                    if ch == '>' {
                        state = State::Normal;
                    } else if ch == '!' {
                        state = State::Skip;
                        prev_state = State::Garbage;
                    } else {
                        self.garbage_count += 1;
                    }
                }
            }
        }
        crate::output(score)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.garbage_count)
    }
}

#[derive(Copy, Clone)]
enum State {
    Garbage,
    Skip,
    Normal,
}
