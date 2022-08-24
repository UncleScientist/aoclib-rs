use crate::{Runner, Selector};

mod aoc2015_01;
mod aoc2015_02;
mod aoc2015_03;
mod aoc2015_04;
mod aoc2015_05;
mod aoc2015_06;
mod aoc2015_07;

pub use aoc2015_01::*;
pub use aoc2015_02::*;
pub use aoc2015_03::*;
pub use aoc2015_04::*;
pub use aoc2015_05::*;
pub use aoc2015_06::*;
pub use aoc2015_07::*;

pub fn run_2015(which: Selector) {
    let mut day01 = Aoc2015_01::new();
    let mut day02 = Aoc2015_02::new();
    let mut day03 = Aoc2015_03::new();
    let mut day04 = Aoc2015_04::new();
    let mut day05 = Aoc2015_05::new();
    let mut day06 = Aoc2015_06::new();
    let mut day07 = Aoc2015_07::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
    ];

    match which {
        Selector::All => {
            for d in days {
                crate::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            crate::run_solution(*d);
        }
    }
}
