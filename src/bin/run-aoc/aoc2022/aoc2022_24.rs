use aoclib::{dijkstra_search, DijkstraSearch};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_24 {
    valley: Blizzard,
}

impl Aoc2022_24 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_24 {
    fn name(&self) -> (usize, usize) {
        (2022, 24)
    }

    fn parse(&mut self) {
        let _lines = aoclib::read_lines("test-input.txt");
        let lines = aoclib::read_lines("input/2022-24.txt");

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        self.valley = Blizzard::new(width, height);

        let mut idx = 0;
        for line in lines {
            for c in line.chars() {
                match c {
                    '#' => self.valley.map[idx] = Blizzard::WALL,
                    '^' => self.valley.map[idx] = Blizzard::UP,
                    'v' => self.valley.map[idx] = Blizzard::DOWN,
                    '<' => self.valley.map[idx] = Blizzard::LEFT,
                    '>' => self.valley.map[idx] = Blizzard::RIGHT,
                    '.' => {}
                    _ => panic!("unknown char '{c}'"),
                };
                idx += 1;
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(dijkstra_search(&self.valley).unwrap().1)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    width: i32,
    height: i32,
    map: Vec<u8>,
    player_pos: (i32, i32),
}

impl Blizzard {
    const UP: u8 = 0x01;
    const DOWN: u8 = 0x02;
    const LEFT: u8 = 0x04;
    const RIGHT: u8 = 0x08;
    const WALL: u8 = 0x80;

    const LOOK: [((i32, i32), u8); 4] = [
        ((-1, 0), Self::DOWN),
        ((1, 0), Self::UP),
        ((0, -1), Self::RIGHT),
        ((0, 1), Self::LEFT),
    ];

    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            map: vec![0; (width * height) as usize],
            player_pos: (0, 1),
        }
    }

    fn get(&self, row: i32, col: i32) -> u8 {
        self.map[(row * self.width + col) as usize]
    }

    fn wrap(&self, from: (i32, i32), look: (i32, i32)) -> (i32, i32) {
        match look {
            (-1, 0) => (self.height - 2, from.1),
            (1, 0) => (1, from.1),
            (0, -1) => (from.0, self.width - 2),
            (0, 1) => (from.0, 1),
            _ => panic!("bad look {look:?}"),
        }
    }

    fn tick(&self) -> Self {
        let mut map: Vec<u8> = Vec::new();

        let mut idx = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                map.push(0);
                let tile = self.get(row, col);
                if row == 0 || row > self.height - 2 || col == 0 || col > self.width - 2 {
                    map[idx] = tile;
                } else {
                    assert!(row > 0 && row < self.height - 1, "{row},{col}");
                    assert!(col > 0 && col < self.width - 1, "{row},{col}");
                    for l in Self::LOOK {
                        let lrow = row + l.0 .0;
                        let lcol = col + l.0 .1;
                        let tile = self.get(lrow, lcol);
                        if tile == Self::WALL {
                            let (lrow, lcol) = self.wrap((row, col), l.0);
                            map[idx] |= self.get(lrow, lcol) & l.1;
                        } else {
                            map[idx] |= tile & l.1;
                        }
                    }
                }
                idx += 1;
            }
        }

        Self {
            width: self.width,
            height: self.height,
            map,
            player_pos: self.player_pos,
        }
    }

    fn _draw(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!(
                    "{}",
                    match self.get(row, col) {
                        0 => '.',
                        Self::UP => '^',
                        Self::DOWN => 'v',
                        Self::LEFT => '<',
                        Self::RIGHT => '>',
                        Self::WALL => '#',
                        _ => 'X',
                    }
                );
            }
            println!();
        }
    }
}

impl DijkstraSearch for Blizzard {
    fn moves(&self) -> Vec<Self> {
        let mut moves = Vec::new();

        let new_state = self.tick();
        let stay_put = (self.player_pos.0, self.player_pos.1);
        let move_up = (self.player_pos.0 - 1, self.player_pos.1);
        let move_down = (self.player_pos.0 + 1, self.player_pos.1);
        let move_left = (self.player_pos.0, self.player_pos.1 - 1);
        let move_right = (self.player_pos.0, self.player_pos.1 + 1);

        for new_pos in [stay_put, move_up, move_down, move_left, move_right] {
            if new_pos.0 >= 0 && new_pos.0 < self.height && new_state.get(new_pos.0, new_pos.1) == 0
            {
                let mut valid = new_state.clone();
                valid.player_pos = new_pos;
                moves.push(valid);
            }
        }

        moves
    }

    fn is_win_state(&self) -> bool {
        self.player_pos == (self.height - 1, self.width - 2)
    }
}
