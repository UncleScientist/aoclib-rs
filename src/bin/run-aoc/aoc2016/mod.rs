use crate::{Runner, Selector};

mod aoc2016_01;
mod aoc2016_02;
mod aoc2016_03;
mod aoc2016_04;

use aoc2016_01::*;
use aoc2016_02::*;
use aoc2016_03::*;
use aoc2016_04::*;

pub fn run_2016(which: Selector) {
    let mut day01 = Aoc2016_01::new();
    let mut day02 = Aoc2016_02::new();
    let mut day03 = Aoc2016_03::new();
    let mut day04 = Aoc2016_04::new();

    let mut days: Vec<&mut dyn Runner> = vec![&mut day01, &mut day02, &mut day03, &mut day04];

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
