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
        // let lines = aoclib::read_lines("test-input");
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

        /*
        let mut flattened_ranges = self
            .seeds
            .chunks(2)
            .map(|vec| SingleMap {
                range: Range {
                    start: vec[0],
                    end: vec[0] + vec[1],
                },
                delta: 0,
            })
            .collect::<Vec<_>>();

        flattened_ranges.sort_by_key(|r| r.range.start);

        for map in &self.mapping {
            flattened_ranges = map.flatten_with(&flattened_ranges);
        }

        let mut min = i64::MAX;
        for range in &flattened_ranges {
            min = min.min(range.range.start + range.delta);
        }
        aoclib::output(min)
        */
    }
}

#[derive(Debug, Default, Clone)]
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
        self.map.sort_by_key(|r| r.range.start);
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

    // 10...20
    //            50...80
    //
    //  10..............50              delta: -5
    //         20.............99        delta: 10
    //  10.=19|20...=49|50....99
    //     -5     +5       10

    //  10..............50              delta: -5
    //      20......40                  delta: 10
    //    -5    5      -5

    // 10.............................100
    //     20....40           60..80
    fn _flatten_with(&self, cur: &[SingleMap]) -> Vec<SingleMap> {
        let mut ci = 0;
        let mut mi = 0;

        let mut result = Vec::new();

        while ci < cur.len() && mi < self.map.len() {
            println!("> {result:?}");
            println!("Merging {:?} with\n        {:?}", cur[ci], self.map[mi]);

            let ci_is_left = cur[ci].range.start < self.map[mi].range.start;

            let (left, right) = if ci_is_left {
                (&cur[ci], &self.map[mi])
            } else {
                (&self.map[mi], &cur[ci])
            };

            if left.range.end < right.range.start {
                println!("<appending left>");
                result.push(left.clone());
                if ci_is_left {
                    ci += 1;
                } else {
                    mi += 1;
                }
                continue;
            }

            if left.range.end < right.range.end {
                println!("<left end < right end>");
                result.push(SingleMap {
                    range: Range {
                        start: left.range.start,
                        end: right.range.start,
                    },
                    delta: left.delta,
                });
                result.push(SingleMap {
                    range: Range {
                        start: right.range.start,
                        end: left.range.end,
                    },
                    delta: left.delta + right.delta,
                });
                result.push(SingleMap {
                    range: Range {
                        start: left.range.end,
                        end: right.range.end,
                    },
                    delta: right.delta,
                });
                ci += 1;
                mi += 1;
            } else {
                println!("<right end <= left end>");
                result.push(SingleMap {
                    range: Range {
                        start: left.range.start,
                        end: right.range.start,
                    },
                    delta: left.delta,
                });
                result.push(SingleMap {
                    range: Range {
                        start: right.range.start,
                        end: right.range.end,
                    },
                    delta: left.delta + right.delta,
                });
                result.push(SingleMap {
                    range: Range {
                        start: right.range.end,
                        end: left.range.end,
                    },
                    delta: left.delta,
                });
                mi += 1;
                ci += 1;
            }
        }

        while ci < cur.len() {
            result.push(cur[ci].clone());
            ci += 1;
        }

        while mi < self.map.len() {
            result.push(self.map[mi].clone());
            mi += 1;
        }

        println!(">> {result:?}");
        result
    }
}
