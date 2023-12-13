use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_13 {
    patterns: Vec<Pattern>,
}

impl Aoc2023_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_13 {
    fn name(&self) -> (usize, usize) {
        (2023, 13)
    }

    fn parse(&mut self) {
        let data = std::fs::read_to_string("input/2023-13.txt").expect("missing file");
        // let data = std::fs::read_to_string("test-input-2").expect("missing file");
        let lines = data.lines();

        let mut pattern = Vec::new();
        let mut width = 0;
        for line in lines {
            if line.is_empty() {
                self.patterns.push(Pattern {
                    pattern,
                    width,
                    score: 0,
                });
                pattern = Vec::new();
            } else {
                width = line.len();
                pattern.push(Pattern::convert(line));
            }
        }

        if !pattern.is_empty() {
            self.patterns.push(Pattern {
                pattern,
                width,
                score: 0,
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        #[cfg(test)]
        let w = self.patterns[0].width;
        #[cfg(test)]
        for row in self.patterns[0].pattern.iter() {
            println!("{:0w$b}", row);
        }

        for p in self.patterns.iter_mut() {
            p.score();
        }

        aoclib::output(self.patterns.iter().map(|p| p.score).sum::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct Pattern {
    pattern: Vec<usize>,
    score: usize,
    width: usize,
}

impl Pattern {
    fn convert(s: &str) -> usize {
        s.chars()
            .fold(0, |acc, ch| (acc << 1) | ((ch == '#') as usize))
    }

    fn score(&mut self) {
        let horizontal = if let Some(horizontal) = self.find_horizontal() {
            horizontal
        } else {
            0
        };
        let vertical = if let Some(vertical) = self.find_vertical() {
            vertical
        } else {
            0
        };

        self.score = horizontal * 100 + vertical;
        println!("{}", self.score);

        if horizontal == 0 && vertical == 0 {
            println!("Could not find score for a pattern");
            let w = self.width;
            for row in self.pattern.iter() {
                println!("{:0w$b}", row);
            }
        }
    }

    fn find_horizontal(&self) -> Option<usize> {
        let len = self.pattern.len() as usize;

        'next_try: for i in 0..len - 1 {
            if self.pattern[i] == self.pattern[i + 1] {
                #[cfg(test)]
                println!("found match at {i}");
                for j in 1..=i {
                    if j + i + 1 >= len {
                        return Some(i + 1);
                    }
                    #[cfg(test)]
                    println!(
                        " - comparing {}[{:b}] with {}[{:b}]",
                        i - j,
                        self.pattern[i - j],
                        i + j + 1,
                        self.pattern[i + j + 1]
                    );
                    if self.pattern[i - j] != self.pattern[i + j + 1] {
                        continue 'next_try;
                    }
                }
                return Some(i + 1);
            }
        }

        None
    }

    // 1010101
    //
    //     101010 | 0001
    //     010101 | 0010
    //     001010 | 0101
    fn find_vertical(&self) -> Option<usize> {
        let mut pat_bits = 0;
        let mut reflect_bits = (1 << self.width) - 1;
        let mut pat = self.pattern.clone();
        let mut reflec = vec![0usize; self.pattern.len()];

        'next_bit: for i in 0..self.width - 1 {
            pat_bits = (pat_bits << 1) | 1;
            reflect_bits >>= 1;
            for (p, r) in pat.iter_mut().zip(reflec.iter_mut()) {
                *r = (*r << 1) | (*p & 1);
                *p >>= 1;
            }
            for row in 0..self.pattern.len() {
                let mask = pat_bits & reflect_bits;
                if cfg!(test) || false {
                    let w = self.width;
                    println!(
                        "{row:2}: ({:0w$b} & {:0w$b}) <-> ({:0w$b} & {:0w$b})",
                        pat[row], reflect_bits, reflec[row], pat_bits
                    );
                }
                if (pat[row] & mask) != (reflec[row] & mask) {
                    continue 'next_bit;
                }
            }
            return Some(self.width - i - 1);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(10, Pattern::convert("#.#."));
    }
}
