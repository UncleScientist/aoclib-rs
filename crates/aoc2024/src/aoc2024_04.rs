use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_04 {
    grid: Vec<Vec<char>>,
}

impl Aoc2024_04 {
    pub fn new() -> Self {
        Self::default()
    }

    fn has_xmas(&self, mut row: i64, mut col: i64, dir: &(i64, i64)) -> bool {
        for x in &XMAS {
            if row < 0
                || col < 0
                || row >= self.grid.len() as i64
                || col >= self.grid[0].len() as i64
            {
                return false;
            }
            if self.grid[row as usize][col as usize] != *x {
                return false;
            }
            row += dir.0;
            col += dir.1;
        }

        true
    }
}

impl Runner for Aoc2024_04 {
    fn name(&self) -> (usize, usize) {
        (2024, 4)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-04.txt");
        for line in lines {
            self.grid.push(line.chars().collect());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let row = row as i64;
                let col = col as i64;
                for dir in &DIRS {
                    total += self.has_xmas(row, col, dir) as usize;
                }
            }
        }
        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let lines = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let mut aoc = Aoc2024_04::default();
        for line in lines {
            aoc.grid.push(line.chars().collect());
        }

        let mut total = 0;
        for row in 0..aoc.grid.len() {
            for col in 0..aoc.grid[row].len() {
                let row = row as i64;
                let col = col as i64;
                for dir in &DIRS {
                    if aoc.has_xmas(row, col, dir) {
                        println!("found one at {row},{col} in dir={dir:?}");
                        total += 1;
                    }
                }
            }
        }
        assert_eq!(18, total);
    }
}
