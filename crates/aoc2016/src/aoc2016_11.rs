use aoclib::{dijkstra_search, Searcher};
use aoclib::{Nodes, Runner};

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub struct Aoc2016_11 {
    building: Building,
}

const _TEST_DATA: &str =
    "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Building {
    floor: Vec<Floor>,
    elevator: usize,
}

impl Searcher for Building {
    fn cost<N: Nodes>(&self, _nodes: &N) -> usize {
        1
    }

    fn moves<N: Nodes>(&self, _nodes: &N) -> Vec<Building> {
        let mut answer = Vec::new();

        let items = self.floor[self.elevator]
            .micros
            .iter()
            .chain(self.floor[self.elevator].gens.iter())
            .collect::<Vec<&Item>>();

        for item in items.iter() {
            if self.elevator > 0 {
                // move item down by one
                let mut new_building = self.clone();
                new_building.elevator -= 1;
                new_building.floor[self.elevator].remove(item);
                new_building.floor[new_building.elevator].insert(item);

                if new_building.valid() {
                    answer.push(new_building);
                }
            }

            if self.elevator < self.floor.len() - 1 {
                // move item up by one
                let mut new_building = self.clone();
                new_building.elevator += 1;
                new_building.floor[self.elevator].remove(item);
                new_building.floor[new_building.elevator].insert(item);

                if new_building.valid() {
                    answer.push(new_building);
                }
            }
        }

        for i in 0..items.len() - 1 {
            for j in i + 1..items.len() {
                if self.elevator > 0 {
                    let mut new_building = self.clone();
                    new_building.elevator -= 1;
                    new_building.floor[self.elevator].remove(items[i]);
                    new_building.floor[self.elevator].remove(items[j]);
                    new_building.floor[new_building.elevator].insert(items[i]);
                    new_building.floor[new_building.elevator].insert(items[j]);

                    if new_building.valid() {
                        answer.push(new_building);
                    }
                }

                if self.elevator < self.floor.len() - 1 {
                    let mut new_building = self.clone();
                    new_building.elevator += 1;
                    new_building.floor[self.elevator].remove(items[i]);
                    new_building.floor[self.elevator].remove(items[j]);
                    new_building.floor[new_building.elevator].insert(items[i]);
                    new_building.floor[new_building.elevator].insert(items[j]);

                    if new_building.valid() {
                        answer.push(new_building);
                    }
                }
            }
        }

        answer
    }

    fn is_win_state<N: Nodes>(&self, _nodes: &N) -> bool {
        for i in 0..self.floor.len() - 1 {
            if !self.floor[i].is_empty() {
                return false;
            }
        }

        true
    }
}

impl Building {
    fn new(floors: usize) -> Self {
        Self {
            floor: vec![Floor::default(); floors],
            elevator: 0,
        }
    }

    fn valid(&self) -> bool {
        for f in &self.floor {
            if !f.valid() {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    fn display(&self) {
        for f in (0..self.floor.len()).rev() {
            print!("F{}", f + 1);
            if f == self.elevator {
                print!(" E");
            } else {
                print!(" .");
            }

            for item in self.floor[f].micros.iter().chain(self.floor[f].gens.iter()) {
                item.display();
            }

            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Item {
    Microchip(String),
    Generator(String),
}

impl Item {
    fn opposite(&self) -> Self {
        match self {
            Item::Microchip(m) => Item::Generator(m.clone()),
            Item::Generator(g) => Item::Microchip(g.clone()),
        }
    }

    #[cfg(test)]
    fn display(&self) {
        match self {
            Item::Microchip(m) => print!(" {m}:M"),
            Item::Generator(g) => print!(" {g}:G"),
        }
    }
}

#[derive(Debug, Default, Clone, Eq)]
struct Floor {
    micros: HashSet<Item>,
    gens: HashSet<Item>,
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        self.micros.len() == other.micros.len() && self.gens.len() == other.gens.len()
    }
}
impl Hash for Floor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.micros.len().hash(state);
        self.gens.len().hash(state);
    }
}

impl Floor {
    fn add_microchip(&mut self, name: &str) {
        self.micros.insert(Item::Microchip(name.to_string()));
    }

    fn add_generator(&mut self, name: &str) {
        self.gens.insert(Item::Generator(name.to_string()));
    }

    fn valid(&self) -> bool {
        if self.micros.is_empty() || self.gens.is_empty() {
            return true;
        }

        for m in &self.micros {
            let matching_pair = m.opposite();
            if self.gens.iter().filter(|&g| *g == matching_pair).count() == 0 {
                return false;
            }
        }

        true
    }

    fn remove(&mut self, item: &Item) {
        match item {
            Item::Generator(_) => self.gens.retain(|g| g != item),
            Item::Microchip(_) => self.micros.retain(|m| m != item),
        }
    }

    fn insert(&mut self, item: &Item) {
        match item {
            Item::Generator(_) => self.gens.insert(item.clone()),
            Item::Microchip(_) => self.micros.insert(item.clone()),
        };
    }

    fn is_empty(&self) -> bool {
        self.gens.is_empty() && self.micros.is_empty()
    }
}

impl Aoc2016_11 {
    pub fn new() -> Self {
        Self {
            building: Building::new(4),
        }
    }
}

impl Runner for Aoc2016_11 {
    fn name(&self) -> (usize, usize) {
        (2016, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-11.txt");
        let _lines = _TEST_DATA
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut building = Building::new(4);

        for (f, l) in lines.iter().enumerate() {
            let words = l.split(' ').collect::<Vec<&str>>();
            if words[4] != "nothing" {
                for &next in words.iter().skip(5) {
                    if next == "a"
                        || next == "and"
                        || next.starts_with("micro")
                        || next.starts_with("gen")
                    {
                        continue;
                    }

                    if let Some((left, _)) = next.split_once('-') {
                        building.floor[f].add_microchip(left);
                    } else {
                        building.floor[f].add_generator(next);
                    }
                }
            }
        }

        self.building = building;
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(dijkstra_search(&self.building, self).unwrap().1)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut alt = self.building.clone();
        alt.floor[0].add_microchip("elerium");
        alt.floor[0].add_microchip("dilithium");
        alt.floor[0].add_generator("elerium");
        alt.floor[0].add_generator("dilithium");
        aoclib::output(dijkstra_search(&alt, self).unwrap().1)
    }
}

impl Nodes for Aoc2016_11 {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_floor_is_valid() {
        assert!(Floor::default().valid());
    }

    #[test]
    fn floor_has_only_microchips() {
        let mut f = Floor::default();
        f.add_microchip("foo");
        assert!(f.valid());
    }

    #[test]
    fn floor_has_only_generators() {
        let mut f = Floor::default();
        f.add_generator("foo");
        assert!(f.valid());
    }

    #[test]
    fn floors_with_pairs() {
        let mut f = Floor::default();
        f.add_microchip("foo");
        f.add_generator("foo");
        assert!(f.valid());
    }

    #[test]
    fn floors_without_matching_generator_is_invalid() {
        let mut f = Floor::default();
        f.add_microchip("foo");
        f.add_generator("foo");
        f.add_microchip("bar");
        assert!(!f.valid());
    }

    #[test]
    fn building_with_empty_floors() {
        let b = Building::new(4);
        assert!(b.valid());
    }

    #[test]
    fn building_with_valid_floor() {
        let mut b = Building::new(4);
        b.floor[0].add_microchip("foo");
        assert!(b.valid());
    }

    #[test]
    fn building_with_invalid_floor() {
        let mut b = Building::new(4);
        b.floor[0].add_microchip("foo");
        b.floor[2].add_microchip("foo");
        b.floor[2].add_generator("foo");
        b.floor[2].add_microchip("bar");
        assert!(!b.valid());
    }

    #[test]
    fn generate_one_move() {
        let mut b = Building::new(4);
        b.floor[0].add_microchip("hydrogen");
        b.floor[0].add_microchip("lithium");
        b.floor[1].add_generator("hydrogen");
        b.floor[2].add_generator("lithium");
        let b_move = b.moves();
        assert_eq!(1, b_move.len());
    }

    #[test]
    fn generate_move_from_top() {
        let mut b = Building::new(4);
        b.elevator = 3;
        b.floor[3].add_microchip("foo");
        b.floor[3].add_generator("foo");
        assert_eq!(3, b.moves().len());
    }

    #[test]
    fn generate_double_move() {
        let mut b = Building::new(4);
        b.elevator = 1;
        b.floor[0].add_microchip("lithium");
        b.floor[1].add_microchip("hydrogen");
        b.floor[1].add_generator("hydrogen");
        b.floor[2].add_generator("lithium");
        let moves = b.moves();
        let mut found = false;
        for m in moves {
            if m.floor[2].micros.len() == 1 && m.floor[2].gens.len() == 2 {
                found = true;
                break;
            }
        }

        assert!(found)
    }

    #[test]
    fn generate_double_microchip_move() {
        let mut b = Building::new(4);
        b.elevator = 2;
        b.floor[2].add_microchip("lithium");
        b.floor[2].add_microchip("hydrogen");
        b.floor[3].add_generator("hydrogen");
        b.floor[3].add_generator("lithium");
        let moves = b.moves();
        let mut found = false;
        for m in moves {
            m.display();
            if m.floor[3].micros.len() == 2 && m.floor[3].gens.len() == 2 {
                found = true;
                for i in 0..3 {
                    assert!(m.floor[i].micros.is_empty());
                    assert!(m.floor[i].gens.is_empty());
                }
                break;
            }
        }

        assert!(found)
    }

    #[test]
    fn can_find_final_move() {
        let mut b = Building::new(4);
        b.elevator = 2;
        b.floor[2].add_microchip("lithium");
        b.floor[2].add_microchip("hydrogen");
        b.floor[3].add_generator("hydrogen");
        b.floor[3].add_generator("lithium");

        assert_eq!(1, dijkstra_search(&b).unwrap().1);
    }

    #[test]
    fn can_solve_example_problem() {
        let mut b = Building::new(4);
        b.elevator = 0;
        b.floor[0].add_microchip("lithium");
        b.floor[0].add_microchip("hydrogen");
        b.floor[1].add_generator("hydrogen");
        b.floor[2].add_generator("lithium");

        assert_eq!(11, dijkstra_search(&b).unwrap().1);
    }
}
