use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_08 {
    map: HashMap<String, Vec<String>>,
    dirs: Vec<usize>,
}

impl Aoc2023_08 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, current: &String, count: usize) -> String {
        self.map.get(current).unwrap()[self.dirs[count % self.dirs.len()]].clone()
    }
}

impl Runner for Aoc2023_08 {
    fn name(&self) -> (usize, usize) {
        (2023, 8)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-08.txt");

        self.dirs = lines[0]
            .chars()
            .map(|ch| if ch == 'L' { 0 } else { 1 })
            .collect();

        for line in lines[1..].iter() {
            let (src, dests) = line.split_once(" = ").unwrap();
            let (left, right) = dests.split_once(", ").unwrap();
            self.map.insert(
                src.to_string(),
                [left[1..].to_string(), right[0..3].to_string()].to_vec(),
            );
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut current = "AAA".to_string();
        let mut count = 0;

        while current != "ZZZ" {
            current = self.get(&current, count);
            count += 1;
        }

        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        let queue = self
            .map
            .keys()
            .filter(|key| key.ends_with('A'))
            .collect::<Vec<_>>();

        let mut ans = Vec::new();
        for entry in queue {
            let mut current = entry.clone();
            let mut count = 0;

            while !current.ends_with('Z') {
                current = self.get(&current, count);
                count += 1;
            }

            ans.push(count)
        }
        aoclib::output(lcm_of(&ans))
    }
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn _gcd_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();

    let mut ans = gcd(first, second);
    for next in iter {
        ans = gcd(ans, *next)
    }

    ans
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn lcm_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();

    let mut ans = lcm(first, second);
    for next in iter {
        ans = lcm(ans, *next)
    }

    ans
}
