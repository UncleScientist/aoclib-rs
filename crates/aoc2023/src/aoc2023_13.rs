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
            p.score = p.score().iter().sum::<usize>();
        }

        aoclib::output(self.patterns.iter().map(|p| p.score).sum::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        'next_pattern: for pattern in &self.patterns {
            let mut pat = pattern.clone();
            for row in 0..pat.pattern.len() {
                for col in 0..pat.width {
                    let bit = 1 << col;
                    pat.pattern[row] ^= bit;

                    let scores = pat.score();
                    if scores.iter().any(|x| *x != 0) {
                        for score in scores {
                            if score != pat.score {
                                total += score;
                                continue 'next_pattern;
                            }
                        }
                    }

                    pat.pattern[row] ^= bit;
                }
            }
            println!("Could not find a smudge");
            let w = pat.width;
            for row in pat.pattern.iter() {
                println!("{:0w$b}", row);
            }
        }

        aoclib::output(total)
    }
}

#[derive(Debug, Clone)]
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

    fn score(&self) -> Vec<usize> {
        let h = self.find_horizontal();
        let v = self.find_vertical();
        /*
        if h.is_empty() && v.is_empty() {
            println!("could not find a score");
        }
        */
        h.iter().chain(v.iter()).copied().collect()
    }

    fn find_horizontal(&self) -> Vec<usize> {
        let mut answer = Vec::new();
        let len = self.pattern.len();

        'next_try: for i in 0..len - 1 {
            if self.pattern[i] == self.pattern[i + 1] {
                #[cfg(test)]
                println!("found match at {i}");
                for j in 1..=i {
                    if j + i + 1 >= len {
                        answer.push((i + 1) * 100);
                        continue 'next_try;
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
                answer.push((i + 1) * 100);
            }
        }

        answer
    }

    // 1010101
    //
    //     101010 | 0001
    //     010101 | 0010
    //     001010 | 0101
    fn find_vertical(&self) -> Vec<usize> {
        let mut answer = Vec::new();

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
                if cfg!(test) {
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
            answer.push(self.width - i - 1);
        }

        answer
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
