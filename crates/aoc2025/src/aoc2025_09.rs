use std::{convert::Infallible, str::FromStr};

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2025_09 {
    tiles: Vec<Point>,
    connections: Vec<Rectangle>,
}

impl Aoc2025_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_09 {
    fn name(&self) -> (usize, usize) {
        (2025, 9)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-09.txt");
        // let lines = aoclib::read_lines("test.txt");
        self.tiles = lines.parse_lines();
        self.connections.extend(
            self.tiles
                .windows(2)
                .map(|pair| Rectangle::new(&pair[0], &pair[1])),
        );
        self.connections.push(Rectangle::new(
            &self.tiles[0],
            &self.tiles[self.tiles.len() - 1],
        ));
    }

    fn part1(&mut self) -> Vec<String> {
        let mut max_area = 0;
        for i in 0..self.tiles.len() - 1 {
            for j in i + 1..self.tiles.len() {
                max_area = max_area.max(self.tiles[i].area_between(&self.tiles[j]));
            }
        }

        aoclib::output(max_area)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max_area = 0;
        for i in 0..self.tiles.len() - 1 {
            for j in i + 1..self.tiles.len() {
                let rectangle = Rectangle::new(&self.tiles[i], &self.tiles[j]);
                if !self
                    .connections
                    .iter()
                    .any(|connection| connection.intersects(&rectangle))
                {
                    max_area = max_area.max(rectangle.area())
                }
            }
        }
        aoclib::output(max_area)
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    fn area_between(&self, other: &Point) -> usize {
        (1 + self.x.abs_diff(other.x)) * (1 + self.y.abs_diff(other.y))
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').unwrap();
        Ok(Point {
            x: left.parse().unwrap(),
            y: right.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Rectangle {
    upper_left: Point,
    lower_right: Point,
}

impl Rectangle {
    fn new(a: &Point, b: &Point) -> Self {
        Self {
            upper_left: Point {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            lower_right: Point {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
        }
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        // adapted from
        // https://stackoverflow.com/questions/306316/determine-if-two-rectangles-overlap-each-other
        self.upper_left.x < other.lower_right.x
            && self.lower_right.x > other.upper_left.x
            && self.upper_left.y < other.lower_right.y
            && self.lower_right.y > other.upper_left.y
    }

    fn area(&self) -> usize {
        self.lower_right.area_between(&self.upper_left)
    }
}
