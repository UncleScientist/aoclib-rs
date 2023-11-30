use aoclib::{Runner, Selector};

mod aoc2018_01;
mod aoc2018_02;
mod aoc2018_03;
mod aoc2018_04;
mod aoc2018_05;
mod aoc2018_06;
mod aoc2018_07;
mod aoc2018_08;
mod aoc2018_09;
mod aoc2018_10;
mod aoc2018_11;
mod aoc2018_12;
mod aoc2018_13;
mod aoc2018_14;
mod aoc2018_15;
mod aoc2018_16;
mod aoc2018_17;

use aoc2018_01::*;
use aoc2018_02::*;
use aoc2018_03::*;
use aoc2018_04::*;
use aoc2018_05::*;
use aoc2018_06::*;
use aoc2018_07::*;
use aoc2018_08::*;
use aoc2018_09::*;
use aoc2018_10::*;
use aoc2018_11::*;
use aoc2018_12::*;
use aoc2018_13::*;
use aoc2018_14::*;
use aoc2018_15::*;
use aoc2018_16::*;
use aoc2018_17::*;

pub fn main() {
    run_2018(Selector::Last);
}

pub fn run_2018(which: Selector) {
    let mut day01 = Aoc2018_01::new();
    let mut day02 = Aoc2018_02::new();
    let mut day03 = Aoc2018_03::new();
    let mut day04 = Aoc2018_04::new();
    let mut day05 = Aoc2018_05::new();
    let mut day06 = Aoc2018_06::new();
    let mut day07 = Aoc2018_07::new();
    let mut day08 = Aoc2018_08::new();
    let mut day09 = Aoc2018_09::new();
    let mut day10 = Aoc2018_10::new();
    let mut day11 = Aoc2018_11::new();
    let mut day12 = Aoc2018_12::new();
    let mut day13 = Aoc2018_13::new();
    let mut day14 = Aoc2018_14::new();
    let mut day15 = Aoc2018_15::new();
    let mut day16 = Aoc2018_16::new();
    let mut day17 = Aoc2018_17::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14,
        &mut day15, &mut day16, &mut day17,
    ];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            aoclib::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                aoclib::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            aoclib::run_solution(*d);
        }
    }
}
