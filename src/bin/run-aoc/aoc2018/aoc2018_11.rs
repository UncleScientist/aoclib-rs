use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_11 {
    serial: i64,
}

impl Aoc2018_11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn power_level(&self, x: i64, y: i64) -> i64 {
        let rack_id = x + 10;

        ((((rack_id * y) + self.serial) * rack_id) / 100) % 10 - 5
    }
}

impl Runner for Aoc2018_11 {
    fn name(&self) -> (usize, usize) {
        (2018, 11)
    }

    fn parse(&mut self) {
        self.serial = 5235;
    }

    fn part1(&mut self) -> Vec<String> {
        let mut max_pl = 0;
        let mut found_x = 0;
        let mut found_y = 0;

        for x in 1..=298 {
            for y in 1..=298 {
                let mut pl = 0;
                for dx in 0..3 {
                    for dy in 0..3 {
                        pl += self.power_level(x + dx, y + dy);
                    }
                }
                if pl > max_pl {
                    found_x = x;
                    found_y = y;
                    max_pl = pl;
                }
            }
        }

        crate::output(format!("{found_x},{found_y}"))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_levels() {
        let a = Aoc2018_11 { serial: 8 };
        assert_eq!(a.power_level(3, 5), 4);

        let a = Aoc2018_11 { serial: 57 };
        assert_eq!(a.power_level(122, 79), -5);

        let a = Aoc2018_11 { serial: 39 };
        assert_eq!(a.power_level(217, 196), 0);

        let a = Aoc2018_11 { serial: 71 };
        assert_eq!(a.power_level(101, 153), 4);
    }
}
