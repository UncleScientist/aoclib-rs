use std::{cmp::Ordering, ops::AddAssign};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_20 {
    particles: Vec<Particle>,
}

impl Aoc2017_20 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_20 {
    fn name(&self) -> (usize, usize) {
        (2017, 20)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-20.txt");
        // let lines = aoclib::read_lines("test-input.txt");

        for line in lines {
            self.particles.push(Particle::parse(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let smallest_acc = self
            .particles
            .iter()
            .min_by(|a, b| a.acc.cmp(&b.acc))
            .map(|p| p.acc.dist())
            .unwrap();

        let slowest_particle = self
            .particles
            .iter()
            .enumerate()
            .filter(|(_, p)| p.acc.dist() == smallest_acc)
            .min_by(|a, b| a.1.pos.cmp(&b.1.pos))
            .unwrap()
            .0;

        crate::output(slowest_particle)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut step = 0;
        while step < 5000 {
            step += 1;
            let prev_step = self.particles.clone();

            for p in self.particles.iter_mut() {
                p.step();
            }

            let mut finished = true;
            'outer: for i in 0..self.particles.len() - 1 {
                for j in i + 1..self.particles.len() {
                    let prev_dist = prev_step[i].distance_to(&prev_step[j]);
                    let cur_dist = self.particles[i].distance_to(&self.particles[j]);
                    if cur_dist < prev_dist {
                        finished = false;
                        break 'outer;
                    }
                }
            }

            if finished {
                return crate::output(self.particles.len());
            }

            let mut i = 0;
            while i < self.particles.len() - 1 {
                let comparison = self.particles[i].pos;
                let mut j = i + 1;
                let mut to_remove = false;
                while j < self.particles.len() {
                    if comparison == self.particles[j].pos {
                        self.particles.remove(j);
                        to_remove = true;
                    } else {
                        j += 1;
                    }
                }
                if to_remove {
                    self.particles.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        crate::output("could not find solution after {step} tries")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn parse(s: &str) -> Self {
        let len = s.len();
        let v3 = s[1..len - 1].split(',').collect::<Vec<_>>();
        Self {
            x: v3[0].parse().unwrap(),
            y: v3[1].parse().unwrap(),
            z: v3[2].parse().unwrap(),
        }
    }

    fn dist(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.dist().cmp(&other.dist()))
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn parse(s: &str) -> Self {
        let three_vec = s.split(", ").collect::<Vec<_>>();
        Self {
            pos: Vec3::parse(three_vec[0].split_once('=').unwrap().1),
            vel: Vec3::parse(three_vec[1].split_once('=').unwrap().1),
            acc: Vec3::parse(three_vec[2].split_once('=').unwrap().1),
        }
    }

    fn step(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }

    fn distance_to(&self, other: &Self) -> i64 {
        (self.pos.dist() - other.pos.dist()).abs()
    }
}
