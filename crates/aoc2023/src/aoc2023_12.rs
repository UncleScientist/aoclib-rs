use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_12 {
    springs: Vec<Spring>,
}

impl Aoc2023_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_12 {
    fn name(&self) -> (usize, usize) {
        (2023, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-12.txt");

        for line in lines.iter() {
            let (pattern, nums) = line.split_once(' ').unwrap();
            let pattern = pattern.chars().collect();
            let sizes = nums.split(',').map(|val| val.parse().unwrap()).collect();
            self.springs.push(Spring { pattern, sizes });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.springs.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

struct Spring {
    pattern: Vec<char>,
    sizes: Vec<usize>,
}

impl Spring {
    fn _arr3(&self) -> usize {
        let mut starts = Vec::new();
        for (idx, size) in self.sizes.iter().enumerate() {
            starts.push(idx + size);
        }

        for (idx, s) in starts.iter().enumerate() {
            for i in *s..self.sizes[idx] {
                if self.pattern[i] == '.' {
                    // no match here
                }
            }
        }

        println!("{starts:?}");
        0
    }

    fn _arr2(&self) -> usize {
        Self::_search(&self.pattern, &self.sizes)
    }

    fn _search(pattern: &[char], sizes: &[usize]) -> usize {
        #[cfg(test)]
        println!("> starting with {pattern:?}, {sizes:?}");
        if sizes.is_empty() {
            #[cfg(test)]
            println!("   -> no sizes left");
            return 0;
        }
        if pattern.is_empty() {
            #[cfg(test)]
            println!("   -> no patterns left");
            return 1;
        }

        for i in 0..sizes[0] {
            if pattern[i] == '.' {
                #[cfg(test)]
                println!(
                    "   -> found . when searching {} in pattern {:?}",
                    sizes[0], pattern
                );
                return Self::_search(&pattern[1..], sizes);
            }
        }

        if pattern.len() == sizes[0] {
            #[cfg(test)]
            println!("   -> end of pattern - 1 match");
            return Self::_search(&pattern[1..], &sizes[1..]);
        }

        if pattern[sizes[0]] == '#' {
            #[cfg(test)]
            println!(
                "   -> found # when check pos + 1, {} pattern {:?}",
                sizes[0], pattern
            );
            return Self::_search(&pattern[1..], sizes);
        }

        #[cfg(test)]
        println!("   -> pattern {:?} matches size {}", pattern, sizes[0]);
        1 + Self::_search(&pattern[1..], &sizes[1..])
    }

    fn _arrangements(&self) -> usize {
        let max = self.sizes.iter().sum::<usize>() + self.sizes.len() - 1;
        let mut matches = Vec::new();

        let mut count = 0;
        let restart = self.sizes.len() - 2;
        #[cfg(test)]
        println!("matching on {:?}", self.pattern);

        let mut ptr = 0;
        let mut which = 0;
        'start: while ptr < max {
            let mut matchpoints = Vec::new();

            'next: while which < self.sizes.len() {
                if ptr + self.sizes[which] > self.pattern.len() {
                    #[cfg(test)]
                    println!("   {ptr} + {} > len", self.sizes[which]);
                    if restart > 0 && self.pattern[matchpoints[restart]] == '?' {
                        ptr = matchpoints[restart] + 1;
                        #[cfg(test)]
                        println!("   >> backing up to {ptr} @ {restart}");
                    }
                    continue 'start;
                }

                for i in 0..self.sizes[which] {
                    if self.pattern[ptr + i] == '.' {
                        #[cfg(test)]
                        println!("   does not match {which} at {}", ptr + i);
                        ptr += 1;
                        continue 'next;
                    }
                }

                matchpoints.push(ptr);
                ptr += self.sizes[which];
                if ptr < self.pattern.len() && self.pattern[ptr] == '#' {
                    #[cfg(test)]
                    println!("   has # after pattern at {ptr}");
                    continue 'next;
                }
                ptr += 1;
                which += 1;
            }

            for i in ptr..self.pattern.len() {
                if self.pattern[i] == '#' {
                    #[cfg(test)]
                    println!("   extra # at end, {i}");
                    if restart > 0 && self.pattern[matchpoints[restart]] == '?' {
                        // ptr = matchpoints[restart] + 1;
                        #[cfg(test)]
                        println!("   >> backing up to {ptr} @ {restart}");
                    }
                    continue 'start;
                }
            }
            #[cfg(test)]
            println!("- found match - {matchpoints:?}");
            matches.push(matchpoints.clone());
            count += 1;
            if self.pattern[matchpoints[restart]] == '?' {
                ptr = matchpoints[restart] + 1;
                #[cfg(test)]
                println!("   >> backing up to {ptr} @ {restart}");
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let spring = Spring {
            pattern: "???.###".chars().collect(),
            sizes: vec![1, 1, 3],
        };
        assert_eq!(1, spring._arr3());
    }

    #[test]
    fn test2() {
        let spring = Spring {
            pattern: ".??..??...?##.".chars().collect(),
            sizes: vec![1, 1, 3],
        };
        assert_eq!(4, spring._arr3());
    }
}
