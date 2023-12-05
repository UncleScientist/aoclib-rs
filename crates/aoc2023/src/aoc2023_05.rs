use std::ops::Range;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_05 {
    seeds: Vec<i64>,
    mapping: Vec<Mapping>,
}

impl Aoc2023_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_05 {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-05.txt");
        //let lines = aoclib::read_lines("test-input");
        let seeds = lines[0].split_once(": ").unwrap().1;
        self.seeds = seeds.split(' ').map(|seed| seed.parse().unwrap()).collect();

        let mut curmap = Mapping::default();
        for line in lines[2..].iter() {
            if line.contains(':') {
                self.mapping.push(curmap);
                curmap = Mapping::default();
                continue;
            }
            let nums: Vec<i64> = line.split(' ').map(|num| num.parse().unwrap()).collect();
            curmap.add_mapping(nums[0], nums[1], nums[2]);
        }
        if !curmap.map.is_empty() {
            self.mapping.push(curmap);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut min = i64::MAX;

        for seed in &self.seeds {
            let mut cur = *seed;
            for map in &self.mapping {
                cur = map.apply_map(cur);
            }
            min = min.min(cur);
        }
        aoclib::output(min)
    }

    fn part2(&mut self) -> Vec<String> {
        let seed_ranges = self
            .seeds
            .chunks(2)
            .map(|vec| Range {
                start: vec[0],
                end: vec[0] + vec[1],
            })
            .collect::<Vec<_>>();

        let mut location = 1_i64;
        loop {
            let mut cur = location;
            for map in self.mapping.iter().rev() {
                cur = map.reverse_lookup(cur);
            }
            for sr in &seed_ranges {
                if sr.contains(&cur) {
                    return aoclib::output(location);
                }
            }
            location += 1;
        }
    }
}

#[derive(Debug, Default)]
struct SingleMap {
    range: Range<i64>,
    delta: i64,
}

#[derive(Debug, Default)]
struct Mapping {
    map: Vec<SingleMap>,
}

impl Mapping {
    fn add_mapping(&mut self, dest: i64, src: i64, len: i64) {
        self.map.push(SingleMap {
            range: Range {
                start: src,
                end: src + len,
            },
            delta: dest - src,
        });
    }

    fn reverse_lookup(&self, val: i64) -> i64 {
        for map in &self.map {
            let rev = val - map.delta;
            if map.range.contains(&rev) {
                return rev;
            }
        }

        val
    }

    fn apply_map(&self, val: i64) -> i64 {
        for map in &self.map {
            if map.range.contains(&val) {
                return val + map.delta;
            }
        }
        val
    }
}
