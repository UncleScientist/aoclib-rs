use crate::Runner;

pub struct Aoc2016_03 {
    list: Vec<(i32, i32, i32)>,
}

impl Aoc2016_03 {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
}

impl Runner for Aoc2016_03 {
    fn name(&self) -> (usize, usize) {
        (2016, 3)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2016-03.txt") {
            let mut iter = line.split_whitespace();
            let side1 = iter.next().unwrap().parse::<i32>().unwrap();
            let side2 = iter.next().unwrap().parse::<i32>().unwrap();
            let side3 = iter.next().unwrap().parse::<i32>().unwrap();
            self.list.push((side1, side2, side3));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut possible = 0;
        for check in &self.list {
            possible += valid_triangle(check.0, check.1, check.2);
        }
        crate::output(possible)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut possible = 0;

        for i in (0..self.list.len()).step_by(3) {
            possible += valid_triangle(self.list[i].0, self.list[i + 1].0, self.list[i + 2].0);
            possible += valid_triangle(self.list[i].1, self.list[i + 1].1, self.list[i + 2].1);
            possible += valid_triangle(self.list[i].2, self.list[i + 1].2, self.list[i + 2].2);
        }

        crate::output(possible)
    }
}

fn valid_triangle(a: i32, b: i32, c: i32) -> i32 {
    (a + b > c && a + c > b && b + c > a) as i32
}
