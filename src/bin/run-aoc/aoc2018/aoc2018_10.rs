use std::collections::HashSet;

use aoclib::Point;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_10 {
    lights: Vec<Light>,
    count: usize,
}

impl Aoc2018_10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_10 {
    fn name(&self) -> (usize, usize) {
        (2018, 10)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-10.txt");
        for l in lines {
            let words = l.split('<').collect::<Vec<_>>();
            let (pos, _) = words[1].split_once('>').unwrap();
            let (vel, _) = words[2].split_once('>').unwrap();
            self.lights.push(Light {
                pos: pos.parse().unwrap(),
                vel: vel.parse().unwrap(),
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut min_y;
        let mut max_y;
        let mut min_x;
        let mut max_x;

        loop {
            min_y = self.lights[0].pos.y;
            max_y = self.lights[0].pos.y;
            min_x = self.lights[0].pos.x;
            max_x = self.lights[0].pos.x;

            for light in &mut self.lights {
                light.pos += light.vel;
                min_y = min_y.min(light.pos.y);
                max_y = max_y.max(light.pos.y);
                min_x = min_x.min(light.pos.x);
                max_x = max_x.max(light.pos.x);
            }

            self.count += 1;
            if max_y - min_y == 10 {
                break;
            }
        }

        let mut display = HashSet::new();
        for light in &self.lights {
            display.insert((light.pos.x, light.pos.y));
        }

        let mut output = Vec::new();
        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                if display.contains(&(x, y)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            output.push(line);
        }

        output
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.count)
    }
}

#[derive(Debug)]
struct Light {
    pos: Point<i64>,
    vel: Point<i64>,
}
