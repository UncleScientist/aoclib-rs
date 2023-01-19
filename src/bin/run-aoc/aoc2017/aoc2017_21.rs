use std::collections::{HashMap, HashSet};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_21 {
    pattern: HashMap<String, String>,
}

impl Aoc2017_21 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_21 {
    fn name(&self) -> (usize, usize) {
        (2017, 21)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-21.txt");

        for line in lines {
            let (left, right) = line.split_once(" => ").unwrap();
            self.pattern.insert(left.to_string(), right.to_string());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = Grid::new();
        for _ in 0..5 {
            let sub_grids = grid.split();

            let mut enhanced_grids = Vec::new();
            for sg in sub_grids {
                enhanced_grids.push(sg.transform(&self.pattern));
            }

            grid = Grid::rejoin(&enhanced_grids);
        }
        crate::output(grid.pixels.len())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    pixels: HashSet<(usize, usize)>,
}

impl ToString for Grid {
    // #.#
    // .##   => #.#/.##/.#.
    // .#.
    fn to_string(&self) -> String {
        let mut result = "".to_string();
        for row in 0..self.size {
            for col in 0..self.size {
                if self.pixels.contains(&(row, col)) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            if row < self.size - 1 {
                result.push('/');
            }
        }

        result
    }
}

impl Grid {
    fn new() -> Self {
        let starting_pixels = vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
        Self {
            size: 3,
            pixels: HashSet::from_iter(starting_pixels),
        }
    }

    // 0011
    // 0011
    // 2233
    // 2233
    // 000111222
    // 000111222
    // 000111222
    // 333444555
    // 333444555
    // 333444555
    // 666777888
    fn split(&self) -> Vec<Self> {
        let size = if self.size % 2 == 0 { 2 } else { 3 };

        let width = self.size / size;
        let mut result = vec![
            Grid {
                size,
                pixels: HashSet::new()
            };
            width * width
        ];
        for row in 0..self.size {
            for col in 0..self.size {
                let which = (row / size) * width + (col / size);
                if self.pixels.contains(&(row, col)) {
                    result[which].pixels.insert((row % size, col % size));
                }
            }
        }

        result
    }

    fn transform(&self, pattern: &HashMap<String, String>) -> Self {
        let mut s = self.to_string();

        for _ in 0..2 {
            for _ in 0..4 {
                if let Some(enhanced) = pattern.get(&s) {
                    return enhanced.into();
                }
                s = s.rotate();
            }
            s = self.to_string().flip();
        }

        panic!("could not find transform for {s}");
    }

    fn rejoin(split: &[Grid]) -> Self {
        let sqrt = (split.len() as f64).sqrt() as usize;
        let split_size = split[0].size;
        let size = split_size * sqrt;

        let mut pixels = HashSet::new();
        for row in 0..size {
            for col in 0..size {
                let split_row = row / split_size;
                let split_col = col / split_size;
                let which = split_row * sqrt + split_col;
                let pixel_row = row % split_size;
                let pixel_col = col % split_size;
                if split[which].pixels.contains(&(pixel_row, pixel_col)) {
                    pixels.insert((row, col));
                }
            }
        }

        Self { pixels, size }
    }

    fn _draw(&self) {
        for row in 0..self.size {
            for col in 0..self.size {
                if self.pixels.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl From<&String> for Grid {
    fn from(s: &String) -> Grid {
        let lines = s.split('/').collect::<Vec<_>>();
        let size = lines.len();
        let mut pixels = HashSet::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    pixels.insert((row, col));
                }
            }
        }
        Grid { size, pixels }
    }
}

trait Flip {
    fn flip(&self) -> String;
}

impl Flip for String {
    fn flip(&self) -> String {
        // 01       1 <-> 0
        // 34       3 <-> 4
        // 012      2 <-> 0
        // 456      4 <-> 6
        // 89a      8 <-> 10
        let mut result = self.chars().collect::<Vec<_>>();
        if result.len() == 5 {
            result.swap(0, 1);
            result.swap(3, 4);
        } else if result.len() == 11 {
            result.swap(0, 2);
            result.swap(4, 6);
            result.swap(8, 10);
        } else {
            panic!("invalid string for swapping");
        }
        result.iter().collect::<String>()
    }
}

trait Rotate {
    fn rotate(&self) -> String;
}

impl Rotate for String {
    fn rotate(&self) -> String {
        let original = self.chars().collect::<Vec<_>>();
        let mut result = original.clone();

        if self.len() == 5 {
            // 01   0 <- 1, 1 <- 4, 4 <- 3, 3 <- 0
            // 34
            result[0] = original[1];
            result[1] = original[4];
            result[4] = original[3];
            result[3] = original[0];
        } else if self.len() == 11 {
            // 012    26a
            // 456 <- 159
            // 89a    048
            result[0] = original[2];
            result[1] = original[6];
            result[2] = original[10];
            result[4] = original[1];
            result[6] = original[9];
            result[8] = original[0];
            result[9] = original[4];
            result[10] = original[8];
        } else {
            panic!("invalid grid for rotation");
        }

        result.iter().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flip_2x2() {
        let s = "#./.#".to_string();
        assert_eq!(s.flip(), ".#/#.".to_string());
    }

    #[test]
    fn flip_3x3() {
        assert_eq!("#../.##/..#".to_string().flip(), "..#/##./#..".to_string())
    }

    #[test]
    fn rotate_2x2() {
        // #.           .#
        // .#           #.
        let s = "#./.#".to_string();
        assert_eq!(s.rotate(), ".#/#.".to_string());
    }

    #[test]
    fn rotate_3x3() {
        // #..          .##
        // .##          .#.
        // ..#          #..
        assert_eq!(
            "#../.##/..#".to_string().rotate(),
            ".##/.#./#..".to_string()
        );
    }

    #[test]
    fn grid_to_string() {
        assert_eq!(".#./..#/###".to_string(), Grid::new().to_string());
    }
}
