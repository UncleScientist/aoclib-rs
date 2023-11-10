use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_11 {
    serial: i64,
    grid: Vec<Vec<i64>>,
}

impl Aoc2018_11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn power_level(&self, x: i64, y: i64) -> i64 {
        let rack_id = x + 10;

        ((((rack_id * y) + self.serial) * rack_id) / 100) % 10 - 5
    }

    fn highest_coord_for_size(&self, size: i64) -> (i64, i64, i64) {
        let mut max_pl = 0;
        let mut found_x = 0;
        let mut found_y = 0;

        for x in 1..=300 - size + 1 {
            for y in 1..=300 - size + 1 {
                let mut pl = 0;
                for dx in 0..size {
                    for dy in 0..size {
                        pl += self.grid[(dy + y) as usize][(dx + x) as usize];
                    }
                }
                if pl > max_pl {
                    found_x = x;
                    found_y = y;
                    max_pl = pl;
                }
            }
        }

        (found_x, found_y, max_pl)
    }
}

impl Runner for Aoc2018_11 {
    fn name(&self) -> (usize, usize) {
        (2018, 11)
    }

    fn parse(&mut self) {
        self.serial = 5235;

        self.grid.push(Vec::new());
        for y in 1..=300 {
            let mut row = Vec::new();
            row.push(0);
            for x in 1..=300 {
                row.push(self.power_level(x, y));
            }
            self.grid.push(row);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let (x, y, _) = self.highest_coord_for_size(3);
        crate::output(format!("{x},{y}"))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut found_x = 0;
        let mut found_y = 0;
        let mut max_pl = 0;
        let mut window_size = 0;

        for size in 1..=300 {
            println!("size {size}");
            let (x, y, pl) = self.highest_coord_for_size(size);
            if pl > max_pl {
                found_x = x;
                found_y = y;
                max_pl = pl;
                window_size = size;
                println!("{x},{y},{size}");
            }
        }

        crate::output(format!("{found_x},{found_y},{window_size}"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_levels() {
        let a = Aoc2018_11 {
            serial: 8,
            grid: Vec::new(),
        };
        assert_eq!(a.power_level(3, 5), 4);

        let a = Aoc2018_11 {
            serial: 57,
            grid: Vec::new(),
        };
        assert_eq!(a.power_level(122, 79), -5);

        let a = Aoc2018_11 {
            serial: 39,
            grid: Vec::new(),
        };
        assert_eq!(a.power_level(217, 196), 0);

        let a = Aoc2018_11 {
            serial: 71,
            grid: Vec::new(),
        };
        assert_eq!(a.power_level(101, 153), 4);
    }
}
