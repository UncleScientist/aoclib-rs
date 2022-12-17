use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_17 {
    chamber: Chamber,
}

impl Aoc2022_17 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_17 {
    fn name(&self) -> (usize, usize) {
        (2022, 17)
    }

    fn parse(&mut self) {
        // self.chamber = Chamber::new(aoclib::read_to_chars("test-input.txt"));
        self.chamber = Chamber::new(aoclib::read_to_chars("input/2022-17.txt"));
        if self.chamber.jets[self.chamber.jets.len() - 1] == '\n' {
            self.chamber.jets.pop();
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut part1 = self.chamber.clone();

        for _ in 0..2022 {
            part1.drop_one();
        }

        crate::output(part1.height())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Default, Clone)]
struct Chamber {
    jets: Vec<char>,
    rocks: Vec<u8>,
    piecenum: usize,
    jetnum: usize,
}

impl Chamber {
    const PIECES: [[u8; 4]; 5] = [
        [0b0000000, 0b0000000, 0b0000000, 0b0011110],
        [0b0000000, 0b0001000, 0b0011100, 0b0001000],
        [0b0000000, 0b0000100, 0b0000100, 0b0011100],
        [0b0010000, 0b0010000, 0b0010000, 0b0010000],
        [0b0000000, 0b0000000, 0b0011000, 0b0011000],
    ];

    fn new(jets: Vec<char>) -> Self {
        Self {
            jets,
            rocks: vec![0, 0, 0, 0, 0, 0, 0],
            piecenum: 0,
            jetnum: 0,
        }
    }

    fn drop_one(&mut self) {
        let mut piece = Self::PIECES[self.piecenum];
        self.piecenum = (self.piecenum + 1) % Self::PIECES.len();

        // make room at the top for the new piece
        let mut last = self.rocks.len() - 7;
        while self.rocks[last] != 0 {
            self.rocks.push(0);
            last += 1;
        }

        let mut bottom = self.rocks.len() - 4;

        loop {
            // start off by using the jet to move the piece left or right
            let jet = self.jets[self.jetnum];
            self.jetnum = (self.jetnum + 1) % self.jets.len();

            match jet {
                '<' => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p <<= 1;
                        }
                    }
                }
                '>' => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p >>= 1;
                        }
                    }
                }
                _ => panic!("bad input '{jet}'"),
            }

            // drop the piece by one if it can
            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break;
            }
        }

        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            self.rocks[bottom] |= piece[prow];
            bottom += 1;
        }
    }

    fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x40) != 0 || (self.rocks[bottom] & (piece[prow] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x01) != 0 || (self.rocks[bottom] & (piece[prow] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (self.rocks[bottom] & piece[prow]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn height(&self) -> usize {
        let mut top = self.rocks.len();
        while top > 0 && self.rocks[top - 1] == 0 {
            top -= 1;
        }
        top
    }

    fn _print_piece(piece: &[u8; 4]) {
        for p in piece {
            Self::_print_row(*p);
            println!();
        }
    }

    fn _print_row(row: u8) {
        let mut bit = 0x40;
        while bit > 0 {
            print!("{}", if (bit & row) != 0 { '#' } else { '.' });
            bit >>= 1;
        }
    }

    fn _draw(&self) {
        let mut top = self.rocks.len();
        while top > 0 {
            top -= 1;
            print!("|");
            Self::_print_row(self.rocks[top]);
            println!("|");
        }
        println!("+-------+");
    }
}
