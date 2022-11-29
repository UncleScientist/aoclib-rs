use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::Runner;
use aoclib::{dijkstra_search, DijkstraSearch, Permutations};

#[derive(Default)]
pub struct Aoc2016_24 {
    points: HashMap<char, (i32, i32)>,
    map: Rc<HashSet<(i32, i32)>>,
    width: i32,
    height: i32,
}

impl Aoc2016_24 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2016_24 {
    fn name(&self) -> (usize, usize) {
        (2016, 24)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-24.txt");
        self.width = lines[0].len() as i32;
        self.height = lines.len() as i32;

        let mut map = HashSet::new();

        for row in 0..self.height {
            for col in lines[row as usize].chars().enumerate() {
                match col.1 {
                    '.' => {
                        map.insert((row, col.0 as i32));
                    }
                    '0'..='7' => {
                        self.points.insert(col.1, (row, col.0 as i32));
                        map.insert((row, col.0 as i32));
                    }
                    _ => {} // don't care
                }
            }
        }

        self.map = Rc::new(map);
    }

    fn part1(&mut self) -> Vec<String> {
        let pointlist: Vec<(&char, &(i32, i32))> = self.points.iter().collect();
        let mut dist: HashMap<(char, char), i32> = HashMap::new();

        for i in 0..pointlist.len() - 1 {
            for j in i + 1..pointlist.len() {
                let mr = MazeRunner {
                    map: self.map.clone(),
                    loc: *pointlist[i].1,
                    end: *pointlist[j].1,
                };
                if let Some((_, distance)) = dijkstra_search(&mr) {
                    dist.insert((*pointlist[i].0, *pointlist[j].0), distance as i32);
                    dist.insert((*pointlist[j].0, *pointlist[i].0), distance as i32);
                } else {
                    panic!(
                        "no path found from {} to {}",
                        pointlist[i].0, pointlist[j].0
                    );
                }
            }
        }

        let nonzero: Vec<(&char, &(i32, i32))> =
            self.points.iter().filter(|(&x, _)| x != '0').collect();

        let mut shortest = i32::MAX;
        for p in nonzero.permutations() {
            let mut cur_char = '0';
            let mut total = 0;
            for (next_char, _) in p {
                total += dist.get(&(cur_char, *next_char)).unwrap();
                cur_char = *next_char;
            }
            shortest = shortest.min(total);
        }

        crate::output(shortest)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Clone, Eq)]
struct MazeRunner {
    map: Rc<HashSet<(i32, i32)>>,
    loc: (i32, i32),
    end: (i32, i32),
}

impl std::hash::Hash for MazeRunner {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.loc.hash(state);
        self.end.hash(state);
    }
}

impl std::cmp::PartialEq for MazeRunner {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.end == other.end
    }
}

impl DijkstraSearch for MazeRunner {
    fn moves(&self) -> Vec<Self> {
        let mut result = Vec::new();
        let directions = vec![
            (self.loc.0 - 1, self.loc.1),
            (self.loc.0 + 1, self.loc.1),
            (self.loc.0, self.loc.1 - 1),
            (self.loc.0, self.loc.1 + 1),
        ];

        for loc in directions {
            if self.map.contains(&loc) {
                result.push(Self {
                    map: self.map.clone(),
                    loc,
                    end: self.end,
                });
            }
        }

        result
    }

    fn is_win_state(&self) -> bool {
        self.loc == self.end
    }
}