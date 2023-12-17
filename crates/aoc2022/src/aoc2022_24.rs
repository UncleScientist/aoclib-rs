use std::hash::{Hash, Hasher};

use aoclib::{astar_search, Nodes, Searcher};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2022_24 {
    valley: Blizzard,
    first_trip_map_idx: usize,
    first_trip_time: usize,
}

impl Aoc2022_24 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Nodes for Aoc2022_24 {
    fn get_value(&self, _row: usize, _col: usize) -> usize {
        1
    }
    fn get_width(&self) -> usize {
        0
    }
    fn get_height(&self) -> usize {
        0
    }
}

impl Runner for Aoc2022_24 {
    fn name(&self) -> (usize, usize) {
        (2022, 24)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-24.txt");

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        self.valley = Blizzard::new(width, height);

        let mut idx = 0;
        let mut map = vec![0; (width * height) as usize];
        for line in lines {
            for c in line.chars() {
                match c {
                    '#' => map[idx] = Blizzard::WALL,
                    '^' => map[idx] = Blizzard::UP,
                    'v' => map[idx] = Blizzard::DOWN,
                    '<' => map[idx] = Blizzard::LEFT,
                    '>' => map[idx] = Blizzard::RIGHT,
                    '.' => {}
                    _ => panic!("unknown char '{c}'"),
                };
                idx += 1;
            }
        }
        self.valley.maps.push(map);

        self.valley.generate_maps();
    }

    fn part1(&mut self) -> Vec<String> {
        let state = State::new(&self.valley);

        let (result, time) = astar_search(
            &state,
            |s| {
                ((s.player_pos.0 - (s.bliz.height - 1)).abs()
                    + (s.player_pos.1 - (s.bliz.width - 2)).abs()) as usize
            },
            self,
        )
        .unwrap();
        self.first_trip_map_idx = result.map_idx;
        self.first_trip_time = time;
        aoclib::output(time)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut return_trip = State::new(&self.valley);
        return_trip.player_pos = (self.valley.height - 1, self.valley.width - 2);
        return_trip.map_idx = self.first_trip_map_idx;
        return_trip.find_start = true;

        let (mut back_to_end, return_time) = astar_search(
            &return_trip,
            |s| ((s.player_pos.0 - 1).abs() + (s.player_pos.1).abs()) as usize,
            self,
        )
        .unwrap();
        back_to_end.find_start = false;

        let final_time = astar_search(
            &back_to_end,
            |s| {
                ((s.player_pos.0 - (s.bliz.height - 1)).abs()
                    + (s.player_pos.1 - (s.bliz.width - 2)).abs()) as usize
            },
            self,
        )
        .unwrap()
        .1;

        aoclib::output(self.first_trip_time + return_time + final_time)
    }
}

#[derive(Default, Clone)]
struct Blizzard {
    width: i32,
    height: i32,
    maps: Vec<Vec<u8>>,
}

#[derive(Clone)]
struct State<'a> {
    player_pos: (i32, i32),
    map_idx: usize,
    find_start: bool,
    bliz: &'a Blizzard,
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.player_pos == other.player_pos
            && self.map_idx == other.map_idx
            && self.find_start == other.find_start
    }
}

impl<'a> Eq for State<'a> {}

impl<'a> Hash for State<'a> {
    fn hash<T: Hasher>(&self, state: &mut T) {
        self.player_pos.hash(state);
        self.map_idx.hash(state);
        self.find_start.hash(state);
    }
}

impl<'a> State<'a> {
    fn new(bliz: &'a Blizzard) -> Self {
        Self {
            player_pos: (0, 1),
            map_idx: 0,
            find_start: false,
            bliz,
        }
    }

    fn tick(&self) -> Self {
        Self {
            player_pos: self.player_pos,
            map_idx: self.map_idx + 1,
            find_start: self.find_start,
            bliz: self.bliz,
        }
    }

    fn get(&self, row: i32, col: i32) -> u8 {
        if row >= 0 && row < self.bliz.height {
            self.bliz.get(self.map_idx, row, col)
        } else {
            Blizzard::WALL
        }
    }
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
            maps: Vec::new(),
        }
    }

    fn get(&self, idx: usize, row: i32, col: i32) -> u8 {
        self.maps[idx % self.maps.len()][(row * self.width + col) as usize]
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

    fn generate_maps(&mut self) {
        let mut map_idx = 0;
        loop {
            let mut map: Vec<u8> = Vec::new();
            let mut idx = 0;
            for row in 0..self.height {
                for col in 0..self.width {
                    map.push(0);
                    let tile = self.get(map_idx, row, col);
                    if row == 0 || row > self.height - 2 || col == 0 || col > self.width - 2 {
                        map[idx] = tile;
                    } else {
                        assert!(row > 0 && row < self.height - 1, "{row},{col}");
                        assert!(col > 0 && col < self.width - 1, "{row},{col}");
                        for l in Self::LOOK {
                            let lrow = row + l.0 .0;
                            let lcol = col + l.0 .1;
                            let tile = self.get(map_idx, lrow, lcol);
                            if tile == Self::WALL {
                                let (lrow, lcol) = self.wrap((row, col), l.0);
                                map[idx] |= self.get(map_idx, lrow, lcol) & l.1;
                            } else {
                                map[idx] |= tile & l.1;
                            }
                        }
                    }
                    idx += 1;
                }
            }

            map_idx += 1;

            if map == self.maps[0] {
                break;
            }
            self.maps.push(map);
        }
    }

    fn _draw(&self, idx: usize, player_pos: (i32, i32)) {
        for row in 0..self.height {
            for col in 0..self.width {
                if (row, col) == player_pos {
                    print!("E");
                } else {
                    print!(
                        "{}",
                        match self.get(idx, row, col) {
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
            }
            println!();
        }
    }
}

impl<'a> Searcher for State<'a> {
    fn cost<N: Nodes>(&self, _nodes: &N) -> usize {
        1
    }

    fn moves<N: Nodes>(&self, _nodes: &N) -> Vec<Self> {
        let mut moves = Vec::new();

        let new_state = self.tick();
        let stay_put = (self.player_pos.0, self.player_pos.1);
        let move_up = (self.player_pos.0 - 1, self.player_pos.1);
        let move_down = (self.player_pos.0 + 1, self.player_pos.1);
        let move_left = (self.player_pos.0, self.player_pos.1 - 1);
        let move_right = (self.player_pos.0, self.player_pos.1 + 1);

        for new_pos in [stay_put, move_up, move_down, move_left, move_right] {
            if new_state.get(new_pos.0, new_pos.1) == 0 {
                let mut valid = new_state.clone();
                valid.player_pos = new_pos;
                moves.push(valid);
            }
        }

        moves
    }

    fn is_win_state<N: Nodes>(&self, _nodes: &N) -> bool {
        if self.find_start {
            self.player_pos == (0, 1)
        } else {
            self.player_pos == (self.bliz.height - 1, self.bliz.width - 2)
        }
    }
}
