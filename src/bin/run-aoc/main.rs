mod aoc2015;
use aoc2015::*;

pub trait Runner {
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
    fn name(&self) -> (usize, usize);
}

fn main() {
    run_2015();
}

pub fn run_solution<T: Runner>(solution: &mut T) {
    let name = solution.name();
    println!("---- {}, Day {} ----", name.0, name.1);

    print_solution(1, &solution.part1());
    print_solution(2, &solution.part2());
}

fn print_solution(which: usize, output: &[String]) {
    let mut i = output.iter();
    println!("Part {which}: {}", i.next().unwrap());
    for line in i {
        println!("        {line}");
    }
}
