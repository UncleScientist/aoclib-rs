use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2022_14 {
    cave: Cave,
}

impl Aoc2022_14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_14 {
    fn name(&self) -> (usize, usize) {
        (2022, 14)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-14.txt");

        for line in lines {
            let mut iter = line.split(" -> ");
            let mut start = Cave::convert(iter.next()).unwrap();
            while let Some(end) = Cave::convert(iter.next()) {
                self.cave.draw_line(start, end);
                start = end;
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut part1 = self.cave.clone();

        let mut count = 0;
        while part1.drop_one() {
            count += 1;
        }

        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut part2 = self.cave.clone();
        part2.has_floor = true;

        let mut count = 0;
        while part2.drop_one() {
            count += 1;
        }

        aoclib::output(count)
    }
}

#[derive(Default, Clone)]
struct Cave {
    tile: HashMap<(i32, i32), char>,
    bottom: i32,
    has_floor: bool,
}

impl Cave {
    fn convert(s: Option<&str>) -> Option<(i32, i32)> {
        if let Some(s) = s {
            let (x, y) = s.split_once(',').unwrap();
            Some((x.parse().unwrap(), y.parse().unwrap()))
        } else {
            None
        }
    }

    fn draw_line(&mut self, start: (i32, i32), end: (i32, i32)) {
        let dx = (end.0 - start.0).signum();
        let dy = (end.1 - start.1).signum();

        self.bottom = self.bottom.max(start.1.max(end.1));

        let mut point = start;
        self.tile.insert(point, '#');
        while point != end {
            point.0 += dx;
            point.1 += dy;
            self.tile.insert(point, '#');
        }
    }

    fn drop_one(&mut self) -> bool {
        let mut sand = (500, 0);

        if self.has_floor && self.tile.contains_key(&sand) {
            return false;
        }

        while let Some(next_pos) = self.fall(sand) {
            if !self.has_floor && next_pos.1 > self.bottom {
                return false;
            }
            sand = next_pos;
            if self.has_floor && sand.1 == self.bottom + 1 {
                break;
            }
        }

        self.tile.insert(sand, 'o');
        true
    }

    fn fall(&self, pos: (i32, i32)) -> Option<(i32, i32)> {
        for delta_x in [0, -1, 1] {
            let new_pos = (pos.0 + delta_x, pos.1 + 1);
            if !self.tile.contains_key(&new_pos) {
                return Some(new_pos);
            }
        }

        None
    }
}
