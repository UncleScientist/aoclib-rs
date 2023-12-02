use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_02 {
    games: Vec<Vec<Turn>>,
}

impl Aoc2023_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_02 {
    fn name(&self) -> (usize, usize) {
        (2023, 2)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-02.txt");
        for line in lines {
            let (_, turns) = line.split_once(": ").unwrap();
            let turns = turns.split("; ").collect::<Vec<_>>();
            let mut turn_list = Vec::new();
            for t in turns {
                let cubes = t.split(", ").collect::<Vec<_>>();
                let mut turn = Turn::default();
                for c in cubes {
                    let (amount, color) = c.split_once(' ').unwrap();
                    let amount: usize = amount.parse().unwrap();
                    match &color[0..1] {
                        "r" => turn.red = amount,
                        "g" => turn.green = amount,
                        "b" => turn.blue = amount,
                        _ => panic!("bug in your code"),
                    }
                }
                turn_list.push(turn);
            }
            self.games.push(turn_list);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut valid_game_total = 0;

        'next_game: for (index, game) in self.games.iter().enumerate() {
            for turn in game {
                if !turn.is_valid() {
                    continue 'next_game;
                }
            }
            valid_game_total += index + 1;
        }

        aoclib::output(valid_game_total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut power_sum = 0;

        for game in &self.games {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for turn in game {
                red = red.max(turn.red);
                green = green.max(turn.green);
                blue = blue.max(turn.blue);
            }
            power_sum += red * green * blue;
        }

        aoclib::output(power_sum)
    }
}

#[derive(Debug, Default)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

impl Turn {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
