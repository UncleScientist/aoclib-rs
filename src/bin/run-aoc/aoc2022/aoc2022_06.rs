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
        crate::output(sliding_window(&self.input, 4).unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(sliding_window(&self.input, 14).unwrap())
    }
}

fn _first_marker(stream: &[char], width: usize) -> Option<usize> {
    'next_window: for (index, window) in stream.windows(width).enumerate() {
        for i in 0..width - 1 {
            for j in i + 1..width {
                if window[i] == window[j] {
                    continue 'next_window;
                }
            }
        }

        return Some(index + width);
    }

    None
}

fn sliding_window(stream: &[char], width: usize) -> Option<usize> {
    let mut start = width + 1;
    let mut counts = vec![0; 26];
    let mut dups = 0;

    let mut front = stream.iter();
    let mut back = stream.iter();

    for _ in 0..width {
        let index = ((*front.next().unwrap() as u8) - b'a') as usize;
        if counts[index] == 1 {
            dups += 1;
        }
        counts[index] += 1;
    }

    while start < stream.len() {
        let first = ((*back.next().unwrap() as u8) - b'a') as usize;
        let last = ((*front.next().unwrap() as u8) - b'a') as usize;

        counts[first] -= 1;
        if counts[first] == 1 {
            dups -= 1;
        }

        counts[last] += 1;
        if counts[last] == 2 {
            dups += 1;
        }

        if dups == 0 {
            return Some(start);
        }

        start += 1;
    }

    None
}
