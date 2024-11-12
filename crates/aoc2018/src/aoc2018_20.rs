use std::{collections::HashMap, str::Chars};

use aoclib::Runner;
type Grid = HashMap<(i64, i64), Vec<(i64, i64)>>;

#[derive(Default)]
pub struct Aoc2018_20 {
    regex: String,
}

impl Aoc2018_20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn longest(iter: &mut Chars) -> usize {
        let mut length = 0;
        let mut max_so_far = 0;

        while let Some(ch) = iter.next() {
            match ch {
                'N' | 'S' | 'E' | 'W' => length += 1,
                '(' => length += Self::longest(iter),
                '$' | ')' if length > 0 => return length.max(max_so_far),
                '$' | ')' => return 0,
                '|' => {
                    max_so_far = max_so_far.max(length);
                    length = 0;
                }
                '^' => {}
                _ => panic!("invalid {ch} in input"),
            }
        }

        length.max(max_so_far)
    }

    fn build_grid(grid: &mut Grid, iter: &mut Chars, mut x: i64, mut y: i64) {
        let start_x = x;
        let start_y = y;

        while let Some(ch) = iter.next() {
            match ch {
                '^' => {}
                'N' => {
                    let entry = grid.entry((x, y)).or_default();
                    y -= 1;
                    entry.push((x, y));
                }
                'S' => {
                    let entry = grid.entry((x, y)).or_default();
                    y += 1;
                    entry.push((x, y));
                }
                'E' => {
                    let entry = grid.entry((x, y)).or_default();
                    x += 1;
                    entry.push((x, y));
                }
                'W' => {
                    let entry = grid.entry((x, y)).or_default();
                    x -= 1;
                    entry.push((x, y));
                }
                '(' => Self::build_grid(grid, iter, x, y),
                '$' | ')' => return,
                '|' => {
                    x = start_x;
                    y = start_y;
                }
                _ => panic!("invalid {ch} in input"),
            }
        }
    }
}

impl Runner for Aoc2018_20 {
    fn name(&self) -> (usize, usize) {
        (2018, 20)
    }

    fn parse(&mut self) {
        self.regex = aoclib::read_lines("input/2018-20.txt")[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut iter = self.regex.chars();
        aoclib::output(Self::longest(&mut iter))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grid = HashMap::new();
        let mut iter = self.regex.chars();
        Self::build_grid(&mut grid, &mut iter, 0, 0);

        let mut stepstore = HashMap::<(i64, i64), Option<usize>>::new();

        let mut stack = Vec::new();
        stack.push(((0, 0), 0));

        while let Some((pos, steps)) = stack.pop() {
            let entry = stepstore.entry(pos).or_default();
            if entry.is_none() {
                *entry = Some(steps);
                if let Some(dirs) = grid.get(&pos) {
                    for dir in dirs {
                        stack.push((*dir, steps + 1))
                    }
                }
            }
        }

        aoclib::output(
            stepstore
                .values()
                .filter(|val| val.is_some() && val.unwrap() >= 1000)
                .count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_three_doors() {
        let mut iter = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$".chars();
        assert_eq!(23, Aoc2018_20::longest(&mut iter));
    }

    #[test]
    fn thirty_one_doors() {
        let mut iter = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".chars();
        assert_eq!(31, Aoc2018_20::longest(&mut iter));
    }

    #[test]
    fn eighteen_doors() {
        let mut iter = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".chars();
        assert_eq!(18, Aoc2018_20::longest(&mut iter));
    }

    #[test]
    fn ten_doors() {
        let mut iter = "^ENWWW(NEEE|SSE(EE|N))$".chars();
        assert_eq!(10, Aoc2018_20::longest(&mut iter));
    }
}
