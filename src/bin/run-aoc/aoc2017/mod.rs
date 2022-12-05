use crate::{Runner, Selector};

mod aoc2017_01;
mod aoc2017_02;
mod aoc2017_03;
mod aoc2017_04;
mod aoc2017_05;

use aoc2017_01::*;
use aoc2017_02::*;
use aoc2017_03::*;
use aoc2017_04::*;
use aoc2017_05::*;

pub fn run_2017(which: Selector) {
    let mut day01 = Aoc2017_01::new();
    let mut day02 = Aoc2017_02::new();
    let mut day03 = Aoc2017_03::new();
    let mut day04 = Aoc2017_04::new();
    let mut day05 = Aoc2017_05::new();

    let mut days: Vec<&mut dyn Runner> =
        vec![&mut day01, &mut day02, &mut day03, &mut day04, &mut day05];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            crate::run_solution(*d);
        }
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
