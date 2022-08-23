mod aoc2015_01;
mod aoc2015_02;
mod aoc2015_03;
mod aoc2015_04;

pub use aoc2015_01::*;
pub use aoc2015_02::*;
pub use aoc2015_03::*;
pub use aoc2015_04::*;

pub fn run_2015() {
    let mut day01 = Aoc2015_01::new();
    crate::run_solution(&mut day01);

    let mut day02 = Aoc2015_02::new();
    crate::run_solution(&mut day02);

    let mut day03 = Aoc2015_03::new();
    crate::run_solution(&mut day03);

    let mut day04 = Aoc2015_04::new();
    crate::run_solution(&mut day04);
}