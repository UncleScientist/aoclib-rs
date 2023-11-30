use aoclib::Runner;

pub struct Aoc2015_14 {
    deer: Vec<Reindeer>,
}

#[derive(Debug)]
struct Reindeer {
    _name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn distance(&self, seconds: u32) -> u32 {
        let stride = self.duration + self.rest;
        let stride_count = seconds / stride;
        let remainder = seconds % stride;

        stride_count * self.speed * self.duration + self.speed * remainder.min(self.duration)
    }
}

impl Aoc2015_14 {
    pub fn new() -> Self {
        Self { deer: Vec::new() }
    }
}

impl Runner for Aoc2015_14 {
    fn name(&self) -> (usize, usize) {
        (2015, 14)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2015-14.txt") {
            let ary = line.split(' ').collect::<Vec<&str>>();
            let _name = ary[0].to_string();
            let speed = ary[3].parse::<u32>().unwrap();
            let duration = ary[6].parse::<u32>().unwrap();
            let rest = ary[13].parse::<u32>().unwrap();

            self.deer.push(Reindeer {
                _name,
                speed,
                duration,
                rest,
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.deer.iter().map(|d| d.distance(2503)).max().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut far: Vec<u32> = vec![0; self.deer.len()];

        for sec in 1..=2503 {
            let mut furthest = 0;
            let mut max_dist = 0;
            for (idx, d) in self.deer.iter().enumerate() {
                let dist = d.distance(sec);
                if dist > max_dist {
                    max_dist = dist;
                    furthest = idx;
                }
            }
            far[furthest] += 1;
        }

        aoclib::output(far.iter().max().unwrap())
    }
}

const _TEST_DATA: &str = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
