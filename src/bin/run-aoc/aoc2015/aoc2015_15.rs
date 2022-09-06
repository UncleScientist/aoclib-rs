use crate::Runner;

// Frosting:     capacity 4,  durability -2, flavor 0,  texture 0, calories 5
// Candy:        capacity 0,  durability 5,  flavor -1, texture 0, calories 8
// Butterscotch: capacity -1, durability 0,  flavor 5,  texture 0, calories 6
// Sugar:        capacity 0,  durability 0,  flavor -2, texture 2, calories 1

pub struct Aoc2015_15 {
    part_1: i32,
    part_2: i32,
}

impl Aoc2015_15 {
    pub fn new() -> Self {
        Self {
            part_1: 0,
            part_2: 0,
        }
    }
}

impl Runner for Aoc2015_15 {
    fn name(&self) -> (usize, usize) {
        (2015, 15)
    }

    fn parse(&mut self) {
        for frosting in 0..100 {
            for candy in 0..(100 - frosting) {
                for butterscotch in 0..(100 - frosting - candy) {
                    let sugar = 100 - butterscotch - candy - frosting;

                    if sugar < 0 {
                        continue;
                    }

                    let capacity = ((4 * frosting) - butterscotch).max(0);
                    let durability = (5 * candy - 2 * frosting).max(0);
                    let flavor = (5 * butterscotch - candy - 2 * sugar).max(0);
                    let texture = (2 * sugar).max(0);

                    let score = capacity * durability * flavor * texture;
                    self.part_1 = self.part_1.max(score);

                    let calories = 5 * frosting + 8 * candy + 6 * butterscotch + sugar;
                    if calories == 500 {
                        self.part_2 = self.part_2.max(score);
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.part_1)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.part_2)
    }
}
