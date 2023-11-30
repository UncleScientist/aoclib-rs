use aoclib::Runner;

#[derive(Debug)]
pub struct Aoc2015_21 {
    boss_hp: i64,
    boss_damage: i64,
    boss_armor: i64,
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<[Item; 2]>,
}

impl Aoc2015_21 {
    pub fn new() -> Self {
        let weapons = vec![
            Item::new("Dagger", 8, 4, 0),
            Item::new("Shortsword", 10, 5, 0),
            Item::new("Warhammer", 25, 6, 0),
            Item::new("Longsword", 40, 7, 0),
            Item::new("Greataxe", 74, 8, 0),
        ];
        let armor = vec![
            Item::new("Nothing", 0, 0, 0),
            Item::new("Leather", 13, 0, 1),
            Item::new("Chainmail", 31, 0, 2),
            Item::new("Splitmail", 53, 0, 3),
            Item::new("Bandedmail", 75, 0, 4),
            Item::new("Platemail", 102, 0, 5),
        ];
        let ring_list = vec![
            Item::new("Nothing", 0, 0, 0),
            Item::new("Nothing", 0, 0, 0),
            Item::new("Damage +1", 25, 1, 0),
            Item::new("Damage +2", 50, 2, 0),
            Item::new("Damage +3", 100, 3, 0),
            Item::new("Defense +1", 20, 0, 1),
            Item::new("Defense +2", 40, 0, 2),
            Item::new("Defense +3", 80, 0, 3),
        ];

        let mut rings = Vec::new();
        for i in 0..ring_list.len() - 1 {
            for j in i + 1..ring_list.len() {
                rings.push([ring_list[i].clone(), ring_list[j].clone()]);
            }
        }

        Self {
            boss_hp: 0,
            boss_damage: 0,
            boss_armor: 0,
            weapons,
            armor,
            rings,
        }
    }
}

impl Runner for Aoc2015_21 {
    fn name(&self) -> (usize, usize) {
        (2015, 21)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2015-21.txt");
        let (_, hp) = lines[0].split_once(": ").unwrap();
        self.boss_hp = hp.parse().unwrap();
        let (_, damage) = lines[1].split_once(": ").unwrap();
        self.boss_damage = damage.parse().unwrap();
        let (_, armor) = lines[2].split_once(": ").unwrap();
        self.boss_armor = armor.parse().unwrap();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut min_cost = 74 + 102 + 100 + 80;
        for w in &self.weapons {
            for a in &self.armor {
                for r in &self.rings {
                    let player = Person::new(
                        100,
                        w.damage + r[0].damage + r[1].damage,
                        a.armor + r[0].armor + r[1].armor,
                    );
                    let boss = Person::new(self.boss_hp, self.boss_damage, self.boss_armor);
                    if player.would_win_against(&boss) {
                        min_cost = min_cost.min(w.cost + a.cost + r[0].cost + r[1].cost);
                    }
                }
            }
        }

        aoclib::output(min_cost)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max_cost = 0;

        for w in &self.weapons {
            for a in &self.armor {
                for r in &self.rings {
                    let player = Person::new(
                        100,
                        w.damage + r[0].damage + r[1].damage,
                        a.armor + r[0].armor + r[1].armor,
                    );
                    let boss = Person::new(self.boss_hp, self.boss_damage, self.boss_armor);
                    if !player.would_win_against(&boss) {
                        max_cost = max_cost.max(w.cost + a.cost + r[0].cost + r[1].cost);
                    }
                }
            }
        }

        aoclib::output(max_cost)
    }
}

#[derive(Debug, Clone)]
struct Item {
    _name: String,
    cost: i64,
    damage: i64,
    armor: i64,
}

impl Item {
    fn new(name: &str, cost: i64, damage: i64, armor: i64) -> Self {
        Self {
            _name: name.to_string(),
            cost,
            damage,
            armor,
        }
    }
}

struct Person {
    hp: i64,
    damage: i64,
    armor: i64,
}

impl Person {
    fn new(hp: i64, damage: i64, armor: i64) -> Self {
        Self { hp, damage, armor }
    }

    fn would_win_against(&self, Person { hp, damage, armor }: &Person) -> bool {
        let mut player_hp = self.hp;
        let mut boss_hp = *hp;

        loop {
            let player_damage = 1.max(self.damage - armor);
            boss_hp -= player_damage;
            if boss_hp <= 0 {
                return true;
            }

            let boss_damage = 1.max(damage - self.armor);
            player_hp -= boss_damage;
            if player_hp <= 0 {
                return false;
            }
        }
    }
}
