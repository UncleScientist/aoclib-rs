use crate::{Runner, Selector};

mod asm;

mod aoc2016_01;
mod aoc2016_02;
mod aoc2016_03;
mod aoc2016_04;
mod aoc2016_05;
mod aoc2016_06;
mod aoc2016_07;
mod aoc2016_08;
mod aoc2016_09;
mod aoc2016_10;
mod aoc2016_11;
mod aoc2016_12;
mod aoc2016_13;
mod aoc2016_14;
mod aoc2016_15;
mod aoc2016_16;
mod aoc2016_17;
mod aoc2016_18;
mod aoc2016_19;
mod aoc2016_20;
mod aoc2016_21;
mod aoc2016_22;
mod aoc2016_23;
mod aoc2016_24;
mod aoc2016_25;

use aoc2016_01::*;
use aoc2016_02::*;
use aoc2016_03::*;
use aoc2016_04::*;
use aoc2016_05::*;
use aoc2016_06::*;
use aoc2016_07::*;
use aoc2016_08::*;
use aoc2016_09::*;
use aoc2016_10::*;
use aoc2016_11::*;
use aoc2016_12::*;
use aoc2016_13::*;
use aoc2016_14::*;
use aoc2016_15::*;
use aoc2016_16::*;
use aoc2016_17::*;
use aoc2016_18::*;
use aoc2016_19::*;
use aoc2016_20::*;
use aoc2016_21::*;
use aoc2016_22::*;
use aoc2016_23::*;
use aoc2016_24::*;
use aoc2016_25::*;

pub fn run_2016(which: Selector) {
    let mut day01 = Aoc2016_01::new();
    let mut day02 = Aoc2016_02::new();
    let mut day03 = Aoc2016_03::new();
    let mut day04 = Aoc2016_04::new();
    let mut day05 = Aoc2016_05::new();
    let mut day06 = Aoc2016_06::new();
    let mut day07 = Aoc2016_07::new();
    let mut day08 = Aoc2016_08::new();
    let mut day09 = Aoc2016_09::new();
    let mut day10 = Aoc2016_10::new();
    let mut day11 = Aoc2016_11::new();
    let mut day12 = Aoc2016_12::new();
    let mut day13 = Aoc2016_13::new();
    let mut day14 = Aoc2016_14::new();
    let mut day15 = Aoc2016_15::new();
    let mut day16 = Aoc2016_16::new();
    let mut day17 = Aoc2016_17::new();
    let mut day18 = Aoc2016_18::new();
    let mut day19 = Aoc2016_19::new();
    let mut day20 = Aoc2016_20::new();
    let mut day21 = Aoc2016_21::new();
    let mut day22 = Aoc2016_22::new();
    let mut day23 = Aoc2016_23::new();
    let mut day24 = Aoc2016_24::new();
    let mut day25 = Aoc2016_25::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14,
        &mut day15, &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21,
        &mut day22, &mut day23, &mut day24, &mut day25,
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
