use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use aoclib::{dijkstra_search, Permutations, Searcher};
use aoclib::{Nodes, Runner};

#[derive(Default)]
pub struct Aoc2016_24 {
    points: HashMap<char, (i32, i32)>,
    map: Rc<HashSet<(i32, i32)>>,
    dist: HashMap<(char, char), i32>,
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
        let height = lines.len() as i32;

        let mut map = HashSet::new();

        for row in 0..height {
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

        for i in 0..pointlist.len() - 1 {
            for j in i + 1..pointlist.len() {
                let mr = MazeRunner {
                    map: self.map.clone(),
                    loc: *pointlist[i].1,
                    end: *pointlist[j].1,
                };
                if let Some((_, distance)) = dijkstra_search(&mr, self) {
                    self.dist
                        .insert((*pointlist[i].0, *pointlist[j].0), distance as i32);
                    self.dist
                        .insert((*pointlist[j].0, *pointlist[i].0), distance as i32);
                } else {
                    panic!(
                        "no path found from {} to {}",
                        pointlist[i].0, pointlist[j].0
                    );
                }
            }
        }

        let nonzero = self
            .points
            .iter()
            .filter(|(&x, _)| x != '0')
            .map(|x| *x.0)
            .collect::<Vec<char>>();

        let mut shortest = i32::MAX;
        for p in nonzero.permutations() {
            let mut cur_char = '0';
            let mut total = 0;
            for next_char in p {
                total += self.dist.get(&(cur_char, next_char)).unwrap();
                cur_char = next_char;
            }
            shortest = shortest.min(total);
        }

        aoclib::output(shortest)
    }

    fn part2(&mut self) -> Vec<String> {
        let nonzero: Vec<&char> = self
            .points
            .iter()
            .filter(|(&x, _)| x != '0')
            .map(|x| x.0)
            .collect();

        let mut shortest = i32::MAX;
        for p in nonzero.permutations() {
            let mut cur_char = '0';
            let mut total = 0;
            for next_char in p {
                total += self.dist.get(&(cur_char, *next_char)).unwrap();
                cur_char = *next_char;
            }
            total += self.dist.get(&(cur_char, '0')).unwrap();
            shortest = shortest.min(total);
        }
        aoclib::output(shortest)
    }
}

impl Nodes for Aoc2016_24 {
    fn get_value(&self, _row: usize, _col: usize) -> usize {
        todo!()
    }

    fn get_width(&self) -> usize {
        todo!()
    }

    fn get_height(&self) -> usize {
        todo!()
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

impl Searcher for MazeRunner {
    fn cost<N: Nodes>(&self, _nodes: &N) -> usize {
        1
    }

    fn moves<N: Nodes>(&self, _nodes: &N) -> Vec<Self> {
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

    fn is_win_state<N: Nodes>(&self, _nodes: &N) -> bool {
        self.loc == self.end
    }
}
