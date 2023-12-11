use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_11 {
    galaxies: HashSet<(i64, i64)>,
    row_offsets: Vec<i64>,
    col_offsets: Vec<i64>,
}

impl Aoc2023_11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_11 {
    fn name(&self) -> (usize, usize) {
        (2023, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-11.txt");
        // let lines = aoclib::read_lines("test-input");

        let mut row_offset = 0;
        let mut col_has_galaxy = vec![false; lines[0].len()];

        for (row, line) in lines.iter().enumerate() {
            let mut galaxy_spotted = false;
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.galaxies.insert((row as i64, col as i64));
                    galaxy_spotted = true;
                    col_has_galaxy[col] = true;
                }
            }
            if !galaxy_spotted {
                row_offset += 1;
            }
            self.row_offsets.push(row_offset);
        }

        let mut col_offset = 0;
        for col in col_has_galaxy {
            if !col {
                col_offset += 1;
            }
            self.col_offsets.push(col_offset);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let galaxies = self.galaxies.iter().collect::<Vec<_>>();

        let mut total_distance = 0;
        for i in 0..galaxies.len() - 1 {
            let (irow, icol) = (
                galaxies[i].0 + self.row_offsets[galaxies[i].0 as usize],
                galaxies[i].1 + self.col_offsets[galaxies[i].1 as usize],
            );

            for j in i + 1..galaxies.len() {
                let (jrow, jcol) = (
                    galaxies[j].0 + self.row_offsets[galaxies[j].0 as usize],
                    galaxies[j].1 + self.col_offsets[galaxies[j].1 as usize],
                );
                total_distance += (irow - jrow).abs() + (icol - jcol).abs();
            }
        }

        aoclib::output(total_distance)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
