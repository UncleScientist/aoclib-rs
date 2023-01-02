use crate::{Runner, Selector};

mod aoc2017_01;
mod aoc2017_02;
mod aoc2017_03;
mod aoc2017_04;
mod aoc2017_05;
mod aoc2017_06;
mod aoc2017_07;
mod aoc2017_08;
mod aoc2017_09;
mod aoc2017_10;
mod aoc2017_11;
mod aoc2017_12;

use aoc2017_01::*;
use aoc2017_02::*;
use aoc2017_03::*;
use aoc2017_04::*;
use aoc2017_05::*;
use aoc2017_06::*;
use aoc2017_07::*;
use aoc2017_08::*;
use aoc2017_09::*;
use aoc2017_10::*;
use aoc2017_11::*;
use aoc2017_12::*;

pub fn run_2017(which: Selector) {
    let mut day01 = Aoc2017_01::new();
    let mut day02 = Aoc2017_02::new();
    let mut day03 = Aoc2017_03::new();
    let mut day04 = Aoc2017_04::new();
    let mut day05 = Aoc2017_05::new();
    let mut day06 = Aoc2017_06::new();
    let mut day07 = Aoc2017_07::new();
    let mut day08 = Aoc2017_08::new();
    let mut day09 = Aoc2017_09::new();
    let mut day10 = Aoc2017_10::new();
    let mut day11 = Aoc2017_11::new();
    let mut day12 = Aoc2017_12::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12,
    ];

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
