use crate::Runner;

pub struct Aoc2015_22;

#[derive(Debug)]
struct GameState {
    player_hp: i32,
    boss_hp: i32,
    mana: i32,
    damage: i32,
    armor: i32,
    spent: i32,
    turn: i32,
    difficulty: i32,
    effects: Vec<Spell>,
    gamelog: String,
}

impl GameState {
    fn new(player_hp: i32, boss_hp: i32, mana: i32, damage: i32) -> Self {
        Self {
            player_hp,
            boss_hp,
            mana,
            damage,
            armor: 0,
            spent: 0,
            turn: 0,
            difficulty: 0,
            effects: Vec::new(),
            gamelog: "".to_string(),
        }
    }

    fn start_turn(&self) -> Option<Self> {
        let mut player_hp = self.player_hp;
        let mut boss_hp = self.boss_hp;
        let mut mana = self.mana;
        let mut armor = 0;
        let mut effects = Vec::new();
        let mut gamelog = if self.turn > 0 {
            format!("{}\n", self.gamelog)
        } else {
            "".to_string()
        };

        player_hp -= self.difficulty;
        if player_hp <= 0 {
            return None;
        }

        gamelog = format!("{gamelog}-- {} turn --\n- Player has {} hit points, {} armor, {} mana\n- Boss has {} hit points\n",
            if self.turn % 2 == 0 {"Player"} else {"Boss"},
            player_hp, self.armor, self.mana, self.boss_hp);

        for effect in &self.effects {
            if effect.armor > 0 {
                armor = effect.armor;
            }

            if effect.damage > 0 {
                boss_hp -= effect.damage;
                gamelog = format!(
                    "{gamelog}{} deals {} damage; its timer is now {}\n",
                    effect.name,
                    effect.damage,
                    effect.duration - 1
                );
            }

            if effect.mana > 0 {
                mana += effect.mana;
                gamelog = format!(
                    "{gamelog}{} provides {} mana; its timer is now {}\n",
                    effect.name,
                    effect.mana,
                    effect.duration - 1
                );
            }

            if effect.duration > 1 {
                effects.push(effect.tick());
            }
        }

        Some(Self {
            player_hp,
            boss_hp,
            mana,
            damage: self.damage,
            armor,
            spent: self.spent,
            turn: self.turn + 1,
            difficulty: self.difficulty,
            effects,
            gamelog,
        })
    }

    fn cast(&self, spell: &Spell) -> Self {
        let gamelog = format!("{}Player casts {}\n", self.gamelog, spell.name);
        if self.mana < spell.cost {
            panic!("bug in move generator");
        }

        if spell.duration > 0 {
            let mut effects = self.effects.clone();
            effects.push(spell.clone());
            Self {
                player_hp: self.player_hp,
                boss_hp: self.boss_hp,
                mana: self.mana - spell.cost,
                damage: self.damage,
                armor: self.armor,
                spent: self.spent + spell.cost,
                turn: self.turn,
                difficulty: self.difficulty,
                effects,
                gamelog,
            }
        } else {
            Self {
                player_hp: self.player_hp + spell.heal,
                boss_hp: self.boss_hp - spell.damage,
                mana: self.mana + spell.mana - spell.cost,
                damage: self.damage,
                armor: self.armor,
                spent: self.spent + spell.cost,
                turn: self.turn,
                difficulty: self.difficulty,
                effects: self.effects.clone(),
                gamelog,
            }
        }
    }

    fn moves(&self) -> Vec<Spell> {
        let mut result = Vec::new();
        'outer: for spell in SPELLS {
            for effect in &self.effects {
                if effect.name == spell.name {
                    continue 'outer;
                }
            }

            if spell.cost <= self.mana {
                result.push(spell.clone());
            }
        }

        result
    }

    fn boss_move(&self) -> Self {
        let damage = 1.max(self.damage - self.armor);
        let gamelog = format!("{}Boss attacks for {damage} damage\n", self.gamelog);
        Self {
            player_hp: self.player_hp - damage,
            boss_hp: self.boss_hp,
            mana: self.mana,
            damage: self.damage,
            armor: self.armor,
            spent: self.spent,
            turn: self.turn,
            difficulty: self.difficulty,
            effects: self.effects.clone(),
            gamelog,
        }
    }

    fn cheapest_win(&self, mut max: i32) -> Option<Self> {
        let mut best_so_far: Option<GameState> = None;

        let turn = self.start_turn()?;

        if turn.player_hp <= 0 {
            return None;
        }

        if turn.boss_hp <= 0 {
            return Some(turn);
        }

        if turn.turn > 50 {
            return None;
        }

        if turn.turn % 2 == 0 {
            let boss_move = turn.boss_move();
            if boss_move.player_hp <= 0 {
                return None;
            }
            return boss_move.cheapest_win(max);
        }

        for spell in &turn.moves() {
            if spell.cost + self.spent >= max {
                continue;
            }

            let move_state = turn.cast(spell);

            if let Some(best) = &best_so_far {
                if best.spent < max {
                    max = best.spent;
                }
            }

            if move_state.boss_hp <= 0 {
                if let Some(best) = &best_so_far {
                    if best.spent > move_state.spent {
                        best_so_far = Some(move_state);
                    }
                } else {
                    best_so_far = Some(move_state);
                }
            } else if move_state.player_hp <= 0 {
                continue;
            } else if let Some(cheapest) = move_state.cheapest_win(max) {
                if let Some(best) = &best_so_far {
                    if best.spent > cheapest.spent {
                        best_so_far = Some(cheapest);
                    }
                } else {
                    best_so_far = Some(cheapest);
                }
            }
        }

        best_so_far
    }
}

impl Aoc2015_22 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2015_22 {
    fn name(&self) -> (usize, usize) {
        (2015, 22)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        // let starting_state = GameState::new(10, 14, 250, 8);
        let starting_state = GameState::new(50, 58, 500, 9);
        let win = starting_state.cheapest_win(i32::MAX).unwrap();
        crate::output(format!("{} mana spent in {} turns", win.spent, win.turn))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut starting_state = GameState::new(50, 58, 500, 9);
        starting_state.difficulty = 1;
        let win = starting_state.cheapest_win(i32::MAX).unwrap();
        crate::output(format!("{} mana spent in {} turns", win.spent, win.turn))
    }
}

// --------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Spell {
    name: &'static str,
    cost: i32,
    damage: i32,
    heal: i32,
    armor: i32,
    mana: i32,
    duration: i32,
}

impl Spell {
    fn tick(&self) -> Self {
        let mut next = self.clone();
        next.duration -= 1;
        next
    }
}

const SPELLS: [Spell; 5] = [
    Spell {
        name: "Magic Missile",
        cost: 53,
        damage: 4,
        heal: 0,
        armor: 0,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: "Drain",
        cost: 73,
        damage: 2,
        heal: 2,
        armor: 0,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: "Shield",
        cost: 113,
        damage: 0,
        heal: 0,
        armor: 7,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: "Poison",
        cost: 173,
        damage: 3,
        heal: 0,
        armor: 0,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: "Recharge",
        cost: 229,
        damage: 0,
        heal: 0,
        armor: 0,
        mana: 101,
        duration: 5,
    },
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_start_a_turn_with_no_effects() {
        let state = GameState::new(10, 13, 250, 8);
        let next_state = state.start_turn().unwrap();
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp, next_state.boss_hp);
        assert_eq!(state.mana, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
    }

    #[test]
    fn can_start_a_turn_with_shield() {
        let mut state = GameState::new(10, 13, 250, 8);
        state.effects.push(SPELLS[2].clone());
        let next_state = state.start_turn().unwrap();
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp, next_state.boss_hp);
        assert_eq!(state.mana, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.armor, 7);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].duration, SPELLS[2].duration - 1);
    }

    #[test]
    fn can_start_a_turn_with_poison() {
        let mut state = GameState::new(10, 13, 250, 8);
        state.effects.push(SPELLS[3].clone());
        let next_state = state.start_turn().unwrap();
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp - 3, next_state.boss_hp);
        assert_eq!(state.mana, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].duration, SPELLS[3].duration - 1);
    }

    #[test]
    fn can_start_a_turn_with_recharge() {
        let mut state = GameState::new(10, 13, 250, 8);
        state.effects.push(SPELLS[4].clone());
        let next_state = state.start_turn().unwrap();
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp, next_state.boss_hp);
        assert_eq!(state.mana + 101, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].duration, SPELLS[4].duration - 1);
    }

    #[test]
    fn can_cast_magic_missile() {
        let state = GameState::new(10, 13, 250, 8);
        let next_state = state.cast(&SPELLS[0]);
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp - SPELLS[0].damage, next_state.boss_hp);
        assert_eq!(state.mana - SPELLS[0].cost, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 0);
        assert_eq!(next_state.spent, SPELLS[0].cost);
    }

    #[test]
    fn can_cast_drain() {
        let state = GameState::new(10, 13, 250, 8);
        let next_state = state.cast(&SPELLS[1]);
        assert_eq!(state.player_hp + SPELLS[1].heal, next_state.player_hp);
        assert_eq!(state.boss_hp - SPELLS[1].damage, next_state.boss_hp);
        assert_eq!(state.mana - SPELLS[1].cost, next_state.mana);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 0);
        assert_eq!(next_state.spent, SPELLS[1].cost);
    }

    #[test]
    fn can_cast_poison() {
        let state = GameState::new(10, 13, 250, 8);
        let next_state = state.cast(&SPELLS[3]);
        assert_eq!(state.player_hp, next_state.player_hp);
        assert_eq!(state.boss_hp, next_state.boss_hp);
        assert_eq!(state.mana - SPELLS[3].cost, next_state.mana);
        assert_eq!(state.spent + SPELLS[3].cost, next_state.spent);
        assert_eq!(state.damage, next_state.damage);
        assert_eq!(next_state.effects.len(), 1);
        assert_eq!(next_state.effects[0].damage, 3);
        assert_eq!(next_state.spent, SPELLS[3].cost);
    }

    #[test]
    fn can_find_all_spells_to_cast() {
        let state = GameState::new(10, 13, 250, 8);
        let list = state.moves();
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn can_find_only_two_spells() {
        let state = GameState::new(10, 13, 73, 8);
        let list = state.moves();
        assert_eq!(list.len(), 2); // <- assuming these are the right ones
    }

    #[test]
    fn no_cheapest_win_without_mana() {
        assert!(GameState::new(10, 13, 0, 8)
            .cheapest_win(i32::MAX)
            .is_none());
    }
}
