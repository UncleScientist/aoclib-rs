mod aoc2015;
use aoc2015::*;

pub trait Runner {
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
    fn name(&self) -> String;
}

fn main() {
    run_2015();
}

pub fn run_solution<T: Runner>(solution: &mut T) {
    println!("---- {} ----", solution.name());

    let v1 = solution.part1();
    let mut p1 = v1.iter();
    let first = p1.next().unwrap();
    println!("Part 1: {first}");
    while let Some(line) = p1.next() {
        println!("       {line}");
    }

    let v2 = solution.part2();
    let mut p2 = v2.iter();
    let first = p2.next().unwrap();
    println!("Part 2: {first}");
    while let Some(line) = p2.next() {
        println!("       {line}");
    }
}
