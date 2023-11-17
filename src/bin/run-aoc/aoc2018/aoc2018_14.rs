use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_14 {
    recipe_count: usize,
}

impl Aoc2018_14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_14 {
    fn name(&self) -> (usize, usize) {
        (2018, 14)
    }

    fn parse(&mut self) {
        self.recipe_count = 939601;
    }

    fn part1(&mut self) -> Vec<String> {
        let mut recipes = vec![3, 7];
        let mut elf1 = 0;
        let mut elf2 = 1;

        while recipes.len() < self.recipe_count + 10 {
            let new_recipe = recipes[elf1] + recipes[elf2];
            if new_recipe > 9 {
                recipes.push(1);
            }
            recipes.push(new_recipe % 10);
            elf1 = (elf1 + recipes[elf1] + 1) % recipes.len();
            elf2 = (elf2 + recipes[elf2] + 1) % recipes.len();
        }

        let mut answer = String::new();
        for c in &recipes[self.recipe_count..] {
            answer.push((*c as u8 + b'0') as char);
        }
        crate::output(answer)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
