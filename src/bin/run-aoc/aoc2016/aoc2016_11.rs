use crate::Runner;

pub struct Aoc2016_11;

const _TEST_DATA: &str =
    "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

#[derive(Debug, Default, Clone)]
struct Building {
    floor: Vec<Floor>,
    elevator: usize,
}

impl Building {
    fn new(floors: usize) -> Self {
        Self {
            floor: vec![Floor::default(); floors],
            elevator: 0,
        }
    }

    #[cfg(test)]
    fn valid(&self) -> bool {
        for f in &self.floor {
            if !f.valid() {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    fn moves(&self) -> Vec<Building> {
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

                if self.elevator < self.floor.len() {
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Microchip(String),
    Generator(String),
}

impl Item {
    #[cfg(test)]
    fn opposite(&self) -> Self {
        match self {
            Item::Microchip(m) => Item::Generator(m.clone()),
            Item::Generator(g) => Item::Microchip(g.clone()),
        }
    }

    fn display(&self) {
        match self {
            Item::Microchip(m) => print!(" {m}:M"),
            Item::Generator(g) => print!(" {g}:G"),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Floor {
    micros: Vec<Item>,
    gens: Vec<Item>,
}

impl Floor {
    #[cfg(test)]
    fn add_microchip(&mut self, name: &str) {
        self.micros.push(Item::Microchip(name.to_string()));
    }

    #[cfg(test)]
    fn add_generator(&mut self, name: &str) {
        self.gens.push(Item::Generator(name.to_string()));
    }

    #[cfg(test)]
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

    #[cfg(test)]
    fn remove(&mut self, item: &Item) {
        match item {
            Item::Generator(_) => self.gens.retain(|g| g != item),
            Item::Microchip(_) => self.micros.retain(|m| m != item),
        }
    }

    #[cfg(test)]
    fn insert(&mut self, item: &Item) {
        match item {
            Item::Generator(_) => self.gens.push(item.clone()),
            Item::Microchip(_) => self.micros.push(item.clone()),
        }
    }
}

impl Aoc2016_11 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2016_11 {
    fn name(&self) -> (usize, usize) {
        (2016, 11)
    }

    fn parse(&mut self) {
        let _lines = aoclib::read_lines("input/2016-11.txt");
        let lines = _TEST_DATA
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
                        building.floor[f]
                            .micros
                            .push(Item::Microchip(left.to_string()));
                    } else {
                        building.floor[f]
                            .gens
                            .push(Item::Generator(next.to_string()));
                    }
                }
            }
        }

        building.display();
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
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
        f.micros.push(Item::Microchip("foo".to_string()));
        assert!(f.valid());
    }

    #[test]
    fn floor_has_only_generators() {
        let mut f = Floor::default();
        f.gens.push(Item::Generator("foo".to_string()));
        assert!(f.valid());
    }

    #[test]
    fn floors_with_pairs() {
        let mut f = Floor::default();
        f.micros.push(Item::Microchip("foo".to_string()));
        f.gens.push(Item::Generator("foo".to_string()));
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
        b.floor[0].micros.push(Item::Microchip("foo".to_string()));
        assert!(b.valid());
    }

    #[test]
    fn building_with_invalid_floor() {
        let mut b = Building::new(4);
        b.floor[0].micros.push(Item::Microchip("foo".to_string()));
        b.floor[2].micros.push(Item::Microchip("foo".to_string()));
        b.floor[2].gens.push(Item::Generator("foo".to_string()));
        b.floor[2].micros.push(Item::Microchip("bar".to_string()));
        assert!(!b.valid());
    }

    #[test]
    fn generate_one_move() {
        let mut b = Building::new(4);
        b.floor[0].add_microchip("hydrogen");
        b.floor[0]
            .micros
            .push(Item::Microchip("lithium".to_string()));
        b.floor[1].add_generator("hydrogen");
        b.floor[2].gens.push(Item::Generator("lithium".to_string()));
        let b_move = b.moves();
        assert_eq!(1, b_move.len());
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
}
