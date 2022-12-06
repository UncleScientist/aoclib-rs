use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_06 {
    input: Vec<char>,
}

impl Aoc2022_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_06 {
    fn name(&self) -> (usize, usize) {
        (2022, 6)
    }

    fn parse(&mut self) {
        self.input = aoclib::read_to_chars("input/2022-06.txt");
        if *self.input.last().unwrap() == '\n' {
            self.input.pop();
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(first_marker(&self.input))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

fn first_marker(stream: &[char]) -> usize {
    'next_window: for (index, window) in stream.windows(4).enumerate() {
        for i in 0..3 {
            for j in i + 1..4 {
                if window[i] == window[j] {
                    continue 'next_window;
                }
            }
        }

        return index + 4;
    }

    0
}
