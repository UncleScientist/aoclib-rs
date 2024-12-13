use std::{
    collections::{HashMap, HashSet},
    ops::{AddAssign, Sub},
};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_12 {
    grid: HashMap<(i64, i64), char>,
}

impl Aoc2024_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_12 {
    fn name(&self) -> (usize, usize) {
        (2024, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-12.txt");
        let _lines = aoclib::read_lines("test12-4.txt");

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                self.grid.insert((row as i64, col as i64), ch);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut garden = self.grid.clone();
        let mut total = 0;
        while let Some(plot) = garden.keys().copied().next() {
            let (area, perimeter) = find_edge_segments(&mut garden, plot);
            total += area * perimeter;
        }
        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut garden = self.grid.clone();
        let mut total = 0;
        while let Some(plot) = garden.keys().copied().next() {
            let (area, perimeter) = find_edges(&mut garden, plot);
            total += area * perimeter;
        }
        aoclib::output(total)
    }
}

fn find_edge_segments(garden: &mut HashMap<(i64, i64), char>, pos: (i64, i64)) -> (usize, usize) {
    let visited = flood_fill(garden, pos);
    let area = visited.len();

    let mut perimeter = 0;
    for plot in &visited {
        for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let newloc = (plot.0 + dir.0, plot.1 + dir.1);
            if !visited.contains(&newloc) {
                perimeter += 1;
            }
        }
    }
    (area, perimeter)
}

fn find_edges(garden: &mut HashMap<(i64, i64), char>, pos: (i64, i64)) -> (usize, usize) {
    let visited = flood_fill(garden, pos);
    let area = visited.len();

    let mut edgelist = HashSet::new();
    for plot in &visited {
        for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let newloc = (plot.0 + dir.0, plot.1 + dir.1);
            if !visited.contains(&newloc) {
                edgelist.insert(Edge::new(*plot, newloc));
            }
        }
    }

    let mut perimeter = 0;

    while let Some(mut edge) = edgelist.iter().copied().next() {
        perimeter += 1;

        let delta = if edge.is_horizontal() { (1, 0) } else { (0, 1) };

        // work our way backwards to the beginning of the edge
        let mut newedge = edge - delta;
        while edgelist.contains(&newedge) {
            edge = newedge;
            newedge = edge - delta;
        }

        // remove the whole edge from the hashset
        while edgelist.remove(&edge) {
            edge += delta;
        }
    }

    (area, perimeter)
}

fn flood_fill(garden: &mut HashMap<(i64, i64), char>, pos: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut stack = vec![pos];
    let mut visited = HashSet::new();
    let plant = *garden.get(&pos).unwrap();

    garden.remove(&pos);

    while let Some(loc) = stack.pop() {
        if visited.insert(loc) {
            for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let newloc = (loc.0 + dir.0, loc.1 + dir.1);
                if let Some(p) = garden.get(&newloc) {
                    if *p == plant {
                        garden.remove(&newloc);
                        stack.push(newloc);
                    }
                }
            }
        }
    }

    visited
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Edge {
    a: (i64, i64),
    b: (i64, i64),
}

impl Edge {
    fn new(a: (i64, i64), b: (i64, i64)) -> Self {
        Self { a, b }
    }

    fn is_horizontal(&self) -> bool {
        self.a.0 == self.b.0
    }
}

impl Sub<(i64, i64)> for Edge {
    type Output = Edge;

    fn sub(self, rhs: (i64, i64)) -> Self::Output {
        Edge {
            a: (self.a.0 - rhs.0, self.a.1 - rhs.1),
            b: (self.b.0 - rhs.0, self.b.1 - rhs.1),
        }
    }
}

impl AddAssign<(i64, i64)> for Edge {
    fn add_assign(&mut self, rhs: (i64, i64)) {
        self.a.0 += rhs.0;
        self.a.1 += rhs.1;
        self.b.0 += rhs.0;
        self.b.1 += rhs.1;
    }
}
